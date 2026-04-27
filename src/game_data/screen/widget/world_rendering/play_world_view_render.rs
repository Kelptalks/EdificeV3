use std::{cell::RefCell, ops::Index, rc::Rc};


use crate::game_data::{
    TextureManager, game_event_manager::player_data_event_manager::var_event_manager::var_events::VarEvents, locations::world_area::WorldArea, player_data::{self, cursor::{self, cursor::Cursor, cursor_event_scheduler::{self, CursorEventScheduler}}, drone_script::var::{game_vars::{dynamic_var::{self, DynamicVarType}, game_var_type::GameVarType}, var_type::VarType}, drones::{drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, prim_actions::drone_world_actions::DroneWorldAction}, drone_event_scheduler}, player_data::PlayerData}, screen::{
        ScreenData, input_data, iso_cord_tool, screen_data, widget::{button::button::Button, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot, play_world_view_config::PlayViewRenderingConfig}, widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties, world_rendering::{area_rendering_manager::{area_rendering_manager::AreaRenderingManager, block_lair_manager::lair_block_manager::LairBlockManager, ray_caster::{ray::TileRay, ray_casting_config::{self, RayCastingConfig}}}, rendering_config}}
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

    // World Rendering
    rendering_config: Rc<RefCell<PlayViewRenderingConfig>>,

    camera_direction: ViewDirection,
    mouse_ray: TileRay,

    // Camera Motion
    camera_ndc_offset: [f32; 2],

    // Cached scale values
    ndc_block_scale: f32,
    ndc_tile_scale: f32,
    ndc_tile_half_scale: f32,
    ndc_draw_centering_offset: [f32; 3],
}

impl PlayWorldViewRender {
    pub fn new(play_view_rendering_config: Rc<RefCell<PlayViewRenderingConfig>>) -> PlayWorldViewRender {

        let mut wp = WidgetProperties::new_blank();
        wp.prefered_scale = [1.0; 2];
        wp.internal_buffers = [0.012; 4];

        PlayWorldViewRender {
            widget_properties: wp,
            overlay_panel: Panel::new_blank(),

            center_ndc: [0.0; 2],

            events: Vec::new(),

            rendering_config: play_view_rendering_config,

            camera_direction: ViewDirection::North,
            mouse_ray: TileRay::new([0; 3], [0; 3]),

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

    pub fn get_rendering_config(&self) -> &Rc<RefCell<PlayViewRenderingConfig>> {
        return &self.rendering_config;
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




        if screen_data.get_input_manager().get_mouse_input_data().scrolled_up() {
            cursor_scheduler.zoom_in()
        }
        else if screen_data.get_input_manager().get_mouse_input_data().scrolled_down() {
            cursor_scheduler.zoom_out()
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


    fn get_mouse_world_cords(&mut self, screen_data: &ScreenData, event_manager: &mut EventManager, player_data: &PlayerData) -> [i32; 3] {

        let mouse_cords = screen_data.get_mouse_ndc();


        let iso_mouse_cords = 
            iso_cord_tool::ndi_screen_cords_to_iso_cords(
                self.ndc_block_scale, 
                mouse_cords
            );

        // get the highest z value
        let cursor = player_data.get_cursor();
        
        // Get the cords of the highest value
        let mut ray_world_cords = cursor.get_cords();
        
        // Offset hight with zoom
        ray_world_cords[0] += cursor.get_zoom() as i32;
        ray_world_cords[1] += cursor.get_zoom() as i32;
        ray_world_cords[2] += cursor.get_zoom() as i32;
        
        // Offset horizontal with mouse
        ray_world_cords[0] += iso_mouse_cords[0] as i32;
        ray_world_cords[1] += iso_mouse_cords[1] as i32;
        

        let block = self.mouse_ray.left_block_struck();
        let cords = self.mouse_ray.left_block_struck_cords();

        
        // println!("Block of mouse: {} | Cords: {:?}", block.get_name(), cords);

        self.mouse_ray = TileRay::new(ray_world_cords, [0; 3]);
        

        ray_world_cords
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

    fn area_to_draw_cords(&self, area_cords: [i32; 3]) -> [f32; 2] {
        let draw_iso_cords = [
            area_cords[0] - area_cords[2],
            area_cords[1] - area_cords[2],
        ];

        let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(self.ndc_block_scale, draw_iso_cords);
        draw_cords[0] += self.ndc_draw_centering_offset[0];
        draw_cords[1] += self.ndc_draw_centering_offset[1];

        draw_cords[0] += self.camera_ndc_offset[0];
        draw_cords[1] += self.camera_ndc_offset[1];

        return draw_cords;
    }

    pub fn ray_cast_view(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {
        self.size_play_view(player_data.get_cursor());

        let world_arc = self.rendering_config.borrow().get_world_ref().clone();
        let world = world_arc.read().unwrap();
    

        let cursor = player_data.get_cursor();
        let world_area = cursor.get_rendering_area();
        

        let mut area_rendering_manager = AreaRenderingManager::new(&world_area);

        // Mouse ray
        self.get_mouse_world_cords(screen_data, event_manager, player_data);
        let config = RayCastingConfig::new(
            LairBlockManager::new(),
            &world_area,
            [1, 1, 1],
            200,
        );
        self.mouse_ray.cast(&world, &config);
        let tiles = area_rendering_manager.get_casted_tile_rays(&world, player_data);


        texture_manager.update_expander_cache(self.ndc_block_scale);

        for tile in tiles {
            let tile_area_cords = tile.get_area_cords();

            let draw_cords = self.area_to_draw_cords(tile_area_cords);
            let [left_textures, right_textures] = tile.get_tile_textures();

            let left_pos = [
                draw_cords[0],
                draw_cords[1],
                draw_cords[0] + self.ndc_block_scale,
                draw_cords[1] + self.ndc_block_scale,
            ];
            for texture in left_textures {
                texture_manager.render_expanded_texture(texture, left_pos);
            }

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
        

        if let Some(view_mode) = player_data.get_view_mode() {
            match view_mode {
                player_data::player_data::ViewMode::Start() => {
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
                    if screen_data.was_right_released() {
                        // Spawn drone at cursor cords
                        let mut cursor_event_scheduler = player_data.get_cursor_event_scheduler();
                        cursor_event_scheduler.spawn_drone();
                        cursor_event_scheduler.schedul_events(event_manager);
                    }
                }
                player_data::player_data::ViewMode::Drone(drone_id) => {
                    // Handle Visuals
                    let mut panel = Panel::new_blank();
                    panel.set_parent_pos(screen_data.get_viewport_uv());
                    panel.set_color(PanelColor::Clear);
                    panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
                    

                    panel.size();
                    self.overlay_panel = panel;

                    // Handle Controls
                    let mut drone_event_scheduler = player_data.get_drone_event_scheduler(drone_id);
                    
                    if let Some(mut drone_event_scheduler) = drone_event_scheduler {
                        let mut cursor_event_scheduler = player_data.get_cursor_event_scheduler();
                            
                        let drone = drone_event_scheduler.get_drone();
                        cursor_event_scheduler.set_cords(drone.get_cords());
                        

                        if screen_data.was_right_pressed() {
                            let mut cords = self.mouse_ray.left_block_struck_cords();
                            cords[2] += 1;

                            drone_event_scheduler.give_action(
                                DroneAction::AdvancedAction(
                                    DroneAdvancedAction::PathToCords(
                                        cords
                                    )
                                )
                            );
                        }


                        cursor_event_scheduler.schedul_events(event_manager);
                        drone_event_scheduler.schedul_events(event_manager);
                    }                    
                },
                player_data::player_data::ViewMode::Location(location_id) => {

                },
            }
            self.overlay_panel.render(texture_manager, screen_data, event_manager, player_data);
        }
        else {
            self.handle_camera_panning(screen_data, event_manager, player_data);
            self.overlay_panel = Panel::new_blank();
        }

        
        
        


    }
}
