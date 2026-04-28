use std::{cell::RefCell, collections::HashMap, ops::Index, rc::Rc};


use crate::game_data::{
    TextureManager, game_event_manager::player_data_event_manager::var_event_manager::var_events::VarEvents, locations::world_area::WorldArea, player_data::{self, cursor::{self, cursor::Cursor, cursor_event_scheduler::{self, CursorEventScheduler}}, drone_script::var::{game_vars::{dynamic_var::{self, DynamicVarType}, game_var_type::GameVarType}, var_type::VarType}, drones::{drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, prim_actions::drone_world_actions::DroneWorldAction}, drone_event_scheduler}, player_data::PlayerData}, screen::{
        ScreenData, input_data, iso_cord_tool, screen_data, widget::{button::button::Button, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot, play_world_view_config::PlayViewRenderingConfig}, widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties, world_rendering::{area_rendering_manager::{area_rendering_manager::AreaRenderingManager, block_lair_manager::{lair_block::LairBlockMod, lair_block_manager::LairBlockManager}, ray_caster::{casted_tile::CastedTile, ray_casting_config::{self, RayCastingConfig}}}, rendering_config, view_mode::ViewMode}}
    }, tools::cords_tool
};

use crate::game_data::game_event_manager::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum ViewDirection {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl ViewDirection {
    pub fn rotation_matrix(&self) -> [[i32; 2]; 2] {
        match self {
            ViewDirection::North => [[ 1,  0], [ 0,  1]],
            ViewDirection::East  => [[ 0,  1], [-1,  0]],
            ViewDirection::South => [[-1,  0], [ 0, -1]],
            ViewDirection::West  => [[ 0, -1], [ 1,  0]],
        }
    }

    pub fn from_id(id: u8) -> Self {
        match id {
            0 => ViewDirection::North,
            1 => ViewDirection::South,
            2 => ViewDirection::East,
            3 => ViewDirection::West,
            _ => ViewDirection::North,
        }
    }

    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn to_string(&self) -> String {
        match self {
            ViewDirection::North => "North".to_string(),
            ViewDirection::South => "South".to_string(),
            ViewDirection::East => "East".to_string(),
            ViewDirection::West => "West".to_string(),
        }
    }
}

pub struct PlayWorldViewRender {
    widget_properties: WidgetProperties,
    overlay_panel: Panel,

    center_ndc: [f32; 2],

    // Input | Handling
    events: Vec<Event>,

    area_rendering_manager: AreaRenderingManager,
    lair_block_mods: Vec<LairBlockMod>,
    casted_tiles: HashMap<[i32; 2], CastedTile>,

    camera_direction: ViewDirection,

    // Camera Motion
    camera_ndc_offset: [f32; 2],

    // Cached scale values
    ndc_block_scale: f32,
    ndc_tile_scale: f32,
    ndc_tile_half_scale: f32,
    ndc_draw_centering_offset: [f32; 3],
}

impl PlayWorldViewRender {
    pub fn new() -> PlayWorldViewRender {

        let mut wp = WidgetProperties::new_blank();
        wp.prefered_scale = [1.0; 2];
        wp.internal_buffers = [0.012; 4];

        PlayWorldViewRender {
            widget_properties: wp,
            overlay_panel: Panel::new_blank(),

            center_ndc: [0.0; 2],

            events: Vec::new(),

            area_rendering_manager: AreaRenderingManager::new(),
            lair_block_mods: Vec::new(),
            casted_tiles: HashMap::new(),

            camera_direction: ViewDirection::North,
            


            camera_ndc_offset: [0.0, 0.0],

            ndc_block_scale: 0.0,
            ndc_tile_scale: 0.0,
            ndc_tile_half_scale: 0.0,
            ndc_draw_centering_offset: [0.0; 3],
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::PlayWorldViewRender(self)
    }


    //=====================================
    // Controls
    //=====================================

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn add_events(&mut self, events: &mut Vec<Event>) {
        self.events.append(events);
    }

    fn handle_camera_zooming(
        &mut self, 
        screen_data: &ScreenData, 
        event_manager: &mut EventManager, 
        player_data: &PlayerData
    ) {
        let mut cursor_scheduler = player_data.get_cursor_event_scheduler();
        if screen_data.get_input_manager().get_mouse_input_data().scrolled_up() {
            cursor_scheduler.zoom_in()
        }
        else if screen_data.get_input_manager().get_mouse_input_data().scrolled_down() {
            cursor_scheduler.zoom_out()
        }
        cursor_scheduler.schedul_events(event_manager);
    }

    fn handle_camera_panning(
        &mut self, 
        screen_data: &ScreenData, 
        event_manager: &mut EventManager, 
        player_data: &PlayerData
    ) {
        
        let mut cursor_scheduler = player_data.get_cursor_event_scheduler();

        // Key panning
        // Z Axis || Up and down
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::Space) {
            cursor_scheduler.mod_cords([0, 0, 1]);
        }
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::LeftShift) {
            cursor_scheduler.mod_cords([0, 0, -1]);
        }

        // X Axis 
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::A) {
            cursor_scheduler.mod_cords([-1, 0, 0]);
        }
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::D) {
            cursor_scheduler.mod_cords([1, 0, 0]);
        }

        // Y Axis 
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::W) {
            cursor_scheduler.mod_cords([0, -1, 0]);
        }
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::S) {
            cursor_scheduler.mod_cords([0, 1, 0]);
        }


        // Middle mouse button panning
        if screen_data.is_middle_mouse_held() {
            let scrolling_offset = screen_data.get_change_in_mouse_ndc();
            self.camera_ndc_offset[0] += scrolling_offset[0];
            self.camera_ndc_offset[1] += scrolling_offset[1];
        }

        let mut cords_offset = [0; 3];

        let iso_offset =
            iso_cord_tool::ndi_screen_cords_to_iso_cords(self.ndc_tile_scale, self.camera_ndc_offset);

        if iso_offset[0] > 1.0 {
            cords_offset[0] -= 1;
            self.camera_ndc_offset[0] -= self.ndc_tile_scale;
            self.camera_ndc_offset[1] -= self.ndc_tile_half_scale;
        }
        if iso_offset[0] < -1.0 {
            cords_offset[0] += 1;
            self.camera_ndc_offset[0] += self.ndc_tile_scale;
            self.camera_ndc_offset[1] += self.ndc_tile_half_scale;
        }

        if iso_offset[1] > 1.0 {
            cords_offset[1] -= 1;
            self.camera_ndc_offset[0] += self.ndc_tile_scale;
            self.camera_ndc_offset[1] -= self.ndc_tile_half_scale;
        }

        if iso_offset[1] < -1.0 {
            cords_offset[1] += 1;
            self.camera_ndc_offset[0] -= self.ndc_tile_scale;
            self.camera_ndc_offset[1] += self.ndc_tile_half_scale;
        }

        cursor_scheduler.mod_cords(cords_offset);
        cursor_scheduler.schedul_events(event_manager);
    }

    fn get_mouse_world_cords(
        &mut self, 
        screen_data: &ScreenData, 
        event_manager: &mut EventManager, 
        player_data: &PlayerData
    ) -> Option<[i32; 3]> {

        let mut mouse_cords = screen_data.get_mouse_ndc();

        mouse_cords[0] -= self.ndc_draw_centering_offset[0];
        mouse_cords[1] -= self.ndc_draw_centering_offset[1];

        mouse_cords[0] -= self.camera_ndc_offset[0];
        mouse_cords[1] -= self.camera_ndc_offset[1];


        let iso_mouse_cords = 
            iso_cord_tool::ndi_screen_cords_to_iso_cords(
                self.ndc_block_scale, 
                mouse_cords
            );

        let tile_key = [
            iso_mouse_cords[0].round() as i32 - 1, 
            iso_mouse_cords[1].round() as i32
        ];

        let iso_offest = 
            iso_cord_tool::casted_to_ndc_cords(
                self.ndc_block_scale, 
                tile_key
            );

        let tile_ndc_cords = [
            mouse_cords[0] - iso_offest[0],
            mouse_cords[1] - iso_offest[1],
        ];

        let mouses_tile = self.casted_tiles.get(&tile_key);
        
        if let Some(mouse_tile) = mouses_tile {
            
            let triangle;
            if tile_ndc_cords[0] > (self.ndc_block_scale) {
                triangle = mouse_tile.get_right_triangle();
            } 
            else {
                triangle = mouse_tile.get_left_triangle();
            }

            if triangle.has_first_struck {
                let cords = triangle.get_first_block_cords_struck();
                
                self.lair_block_mods.push(
                    LairBlockMod::AddOverlayTexture(
                        crate::game_data::types::BlockTexture::Selector, 
                        cords
                    )
                );

                return Some(cords);
                
            }
            else {
                None
            }
        }
        else {
            return None
        }


        
    }


    //=====================================
    // Rendering
    //=====================================

    pub fn size_play_view(&mut self, cursor: &Cursor) {
        self.widget_properties.scale_based_off_parent();

        let scale = self.widget_properties.scale;

        
        let largest_side_of_location = 1;
        let block_diementions = largest_side_of_location + cursor.get_zoom() * 2 + 1;

        self.ndc_block_scale = (scale[0] / block_diementions as f32) / 2.0;
        self.ndc_tile_scale = self.ndc_block_scale / 2.0;
        self.ndc_tile_half_scale = self.ndc_tile_scale / 2.0;

        self.center_ndc = [
            -self.ndc_tile_half_scale,
            -self.ndc_tile_half_scale / 2.0,
        ];


        self.ndc_draw_centering_offset[0] = self.center_ndc[0] - self.ndc_block_scale;
        self.ndc_draw_centering_offset[1] = self.center_ndc[1] - self.ndc_block_scale;
    }

    pub fn ray_cast_view(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {
        self.size_play_view(player_data.get_cursor());

        let world_arc = player_data.get_world_ref();
        let world = world_arc.read().unwrap();
    


        let cursor = player_data.get_cursor();
        let world_area = cursor.get_rendering_area();
        self.area_rendering_manager.set_world_area(world_area);



        
        // Cast the tiles and add them to the map
        self.casted_tiles.clear();
        let tiles = self.area_rendering_manager.get_casted_tile_rays(&world, player_data, &self.lair_block_mods);
        for tile in tiles {
            
            let area_cords = tile.get_area_cords();
            let flattened_iso_cords = [
                area_cords[0] - area_cords[2],
                area_cords[1] - area_cords[2],
            ];
            self.casted_tiles.insert(flattened_iso_cords, tile);
            
        }


        texture_manager.update_expander_cache(self.ndc_block_scale);


        for (flattened_iso_cords, tile) in &mut self.casted_tiles {
            let tile_area_cords = tile.get_area_cords();

            let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(self.ndc_block_scale, *flattened_iso_cords);
            
            
            draw_cords[0] += self.ndc_draw_centering_offset[0];
            draw_cords[1] += self.ndc_draw_centering_offset[1];

            draw_cords[0] += self.camera_ndc_offset[0];
            draw_cords[1] += self.camera_ndc_offset[1];

            let left_textures = tile.get_left_triangle().get_textures().clone();
            let left_pos = [
                draw_cords[0],
                draw_cords[1],
                draw_cords[0] + self.ndc_block_scale,
                draw_cords[1] + self.ndc_block_scale,
            ];
            for texture in left_textures {
                texture_manager.render_expanded_texture(texture, left_pos);
            }

            let right_textures = tile.get_right_triangle().get_textures().clone();
            let right_pos = [
                draw_cords[0] + self.ndc_block_scale,
                draw_cords[1],
                draw_cords[0] + (self.ndc_block_scale * 2.0),
                draw_cords[1] + self.ndc_block_scale,
            ];
            for texture in right_textures {
                texture_manager.render_expanded_texture(texture, right_pos);
            }

        }


    }

    //=====================================
    // Settings
    //=====================================

    pub fn set_prefered_size(&mut self, scale: f32) {
        self.widget_properties.prefered_scale = [scale; 2];
    }

}

impl Widget for PlayWorldViewRender {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.widget_properties.external_buffers = pos;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
    }

    fn size(&mut self) {
        
    }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {
        // Render View
        self.ray_cast_view(texture_manager, screen_data, event_manager, &player_data);
        
        self.lair_block_mods.clear();

        if let Some(view_mode) = player_data.get_view_mode() {
            match view_mode {
                ViewMode::Start() => {
                    // Handle Visuals
                    let mut panel = Panel::new_blank();
                    panel.set_parent_pos(screen_data.get_viewport_uv());
                    panel.set_color(PanelColor::Clear);
                    panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
                    
                    // Add instructions
                    panel.add_text_display("Spawn your drone".to_string())
                        .set_text_scale(widget_calculations::TextSize::Large);
                    panel.add_text_display("right click to place".to_string())
                        .set_text_scale(widget_calculations::TextSize::Medium);

                    panel.size();

                    self.overlay_panel = panel;

                    // Handle Controls
                    self.handle_camera_panning(screen_data, event_manager, player_data);
                    self.handle_camera_zooming(screen_data, event_manager, player_data);
                    self.get_mouse_world_cords(screen_data, event_manager, player_data);

                    if screen_data.was_right_released() {
                        // Spawn drone at cursor cords
                        let mut cursor_event_scheduler = player_data.get_cursor_event_scheduler();
                        cursor_event_scheduler.spawn_drone();
                        cursor_event_scheduler.schedul_events(event_manager);
                    }
                }
                ViewMode::Drone(drone_id) => {
                    // Handle Visuals
                    let mut panel = Panel::new_blank();
                    panel.set_parent_pos(screen_data.get_viewport_uv());
                    panel.set_color(PanelColor::Clear);
                    panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
                    

                    panel.size();
                    self.overlay_panel = panel;

                    // Handle Controls
                    self.handle_camera_zooming(screen_data, event_manager, player_data);
                    let mut drone_event_scheduler = player_data.get_drone_event_scheduler(drone_id);
                    

                    if let Some(mut drone_event_scheduler) = drone_event_scheduler {
                        let mut cursor_event_scheduler = player_data.get_cursor_event_scheduler();
                            
                        let drone = drone_event_scheduler.get_drone();
                        cursor_event_scheduler.set_cords(drone.get_cords());

                        let cords = self.get_mouse_world_cords(screen_data, event_manager, player_data);
                        if let Some(mut cords) = cords {
                            cords[2] += 1;

                            if screen_data.was_right_pressed() {
                                
                                drone_event_scheduler.give_action(
                                    DroneAction::AdvancedAction(
                                        DroneAdvancedAction::PathToCords(
                                            cords
                                        )
                                    )
                                );
                            }
                        }

                        cursor_event_scheduler.schedul_events(event_manager);
                        drone_event_scheduler.schedul_events(event_manager);

                    }                  


                },
                ViewMode::Location(location_id) => {

                },
            }
            self.overlay_panel.render(texture_manager, screen_data, event_manager, player_data);
        }
        else {
            self.handle_camera_panning(screen_data, event_manager, player_data);
            self.handle_camera_zooming(screen_data, event_manager, player_data);
            self.overlay_panel = Panel::new_blank();
        }

        
        
        


    }
}
