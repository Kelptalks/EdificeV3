use std::{cell::RefCell, collections::HashMap, ops::Index, rc::Rc, time::{Instant, SystemTime}};


use crate::game_data::{
    TextureManager, game_event_manager::{self, player_data_event_manager::var_event_manager::var_events::VarEvents}, locations::world_area::WorldArea, player_data::{self, cursor::{self, cursor::Cursor, cursor_event_scheduler::{self, CursorEventScheduler}}, drone_script::var::{game_vars::{dynamic_var::{self, DynamicVarType}, game_var_type::GameVarType}, var_type::VarType}, drones::{drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, prim_actions::drone_world_actions::DroneWorldAction}, drone_event_scheduler}, player_data::PlayerData}, screen::{
        ScreenData, input_data, iso_cord_tool, screen_data, widget::{button::button::Button, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot, play_world_view_config::PlayViewRenderingConfig}, widget::{Widget, WidgetType}, widget_calculations, widget_properties::{self, WidgetProperties}, world_rendering::{area_rendering_manager::{area_rendering_manager::AreaRenderingManager, block_lair_manager::{lair_block::LairBlockMod, lair_block_manager::LairBlockManager}, ray_caster::{casted_tile::CastedTile, casted_triangle::CastedTriangle, ray_casting_config::{self, RayCastingConfig}}}, rendering_config, tile_map::{TileMap, TileMapId}, tile_map_manager::TileMapManager, view_mode::ViewMode}}
    }, texture_manager::texture::Texture, tools::cords_tool
};

use crate::game_data::game_event_manager::prelude::*;

/*
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

 */

pub struct PlayWorldViewRender {
    widget_properties: WidgetProperties,
    overlay_panel: Panel,

    center_ndc: [f32; 2],

    // Input | Handling
    events: Vec<Event>,

    area_rendering_manager: AreaRenderingManager,
    lair_block_mods: Vec<LairBlockMod>,
    tile_map_manager: TileMapManager,
    tile_map: TileMap,


    // Camera Motion
    camera_ndc_offset: [f32; 2],

    // Cached scale values
    ndc_block_scale: f32,
    ndc_tile_scale: f32,
    ndc_tile_half_scale: f32,
    ndc_draw_centering_offset: [f32; 3],

    // debug
    entitys_drawn: u32,
}

impl PlayWorldViewRender {
    
    pub fn new(parent_props: &WidgetProperties) -> PlayWorldViewRender {

        let mut wp = WidgetProperties::new_with_parent_props(parent_props);
        wp.prefered_scale = [1.0; 2];
        wp.internal_buffers = [0.012; 4];


        let tile_map_manager = TileMapManager::new();
        let tile_map = TileMap::new(0);

        PlayWorldViewRender {
            widget_properties: wp,
            overlay_panel: Panel::new_blank(),

            center_ndc: [0.0; 2],

            events: Vec::new(),

            area_rendering_manager: AreaRenderingManager::new(),
            lair_block_mods: Vec::new(),
            tile_map_manager: tile_map_manager, // Just set blank as they are reset every frame in render_view
            tile_map,

            camera_ndc_offset: [0.0, 0.0],

            ndc_block_scale: 1.0,
            ndc_tile_scale: 1.0,
            ndc_tile_half_scale: 1.0,
            ndc_draw_centering_offset: [0.0; 3],

            // debug
            entitys_drawn: 0,
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
        
        let cursor = player_data.get_cursor();
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
            iso_cord_tool::ndi_screen_cords_to_iso_cords(
                self.ndc_tile_scale, 
                self.camera_ndc_offset
            );


        cursor_scheduler.mod_cords(cords_offset);
        cursor_scheduler.schedul_events(event_manager);
    }

    fn get_mouse_triangle(
        &mut self, 
        screen_data: &ScreenData, 
        event_manager: &mut EventManager, 
        player_data: &PlayerData
    ) -> Option<CastedTriangle> {

        let mut mouse_cords = screen_data.get_mouse_ndc();

        

        mouse_cords[0] -= self.ndc_draw_centering_offset[0];
        mouse_cords[1] -= self.ndc_draw_centering_offset[1];

        mouse_cords[0] += self.camera_ndc_offset[0];
        mouse_cords[1] += self.camera_ndc_offset[1];



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
            mouse_cords[0] + iso_offest[0],
            mouse_cords[1] + iso_offest[1],
        ];

        let mouses_tile = self.tile_map_manager.get_tile_with_flattened_cords(&tile_key);
        
        if let Some(mouse_tile) = mouses_tile {
            
            let triangle;
            if tile_ndc_cords[0] > (self.ndc_block_scale) {
                let triangle = mouse_tile.get_right_triangle().clone();
                return Some(triangle);
            } 
            else {
                triangle = mouse_tile.get_left_triangle().clone();
                return Some(triangle);
            }
        }
        else {
            return None
        }


        
    }

    //=====================================
    // Core Rendering
    //=====================================

    fn render_left_triangle(&self, texture_manager: &mut TextureManager, flattened_iso_cords: &[i32; 2], tile: &CastedTile) {
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
    }

    fn render_right_triangle(&self, texture_manager: &mut TextureManager, flattened_iso_cords: &[i32; 2], tile: &CastedTile) {
        let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(self.ndc_block_scale, *flattened_iso_cords);
        
        draw_cords[0] += self.ndc_draw_centering_offset[0];
        draw_cords[1] += self.ndc_draw_centering_offset[1];

        draw_cords[0] += self.camera_ndc_offset[0];
        draw_cords[1] += self.camera_ndc_offset[1];

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

    fn render_enitity_at_world_pos(
        &mut self, 
        texture_manager: &mut TextureManager, 
        player_data: &PlayerData, 
        world_pos: [f32; 3], 
        texture: Texture
    ) {
        // render the entity texture
        let cursor_cords = player_data.get_cursor().get_cords();
        let offset_world_cords = [
            world_pos[0] - cursor_cords[0] as f32 - 0.25, // add the 0.5 to center on the block
            world_pos[1] - cursor_cords[1] as f32, // ^
            world_pos[2] - cursor_cords[2] as f32,
        ];

        let mut draw_cords = iso_cord_tool::world_pos_to_ndc_cords(self.ndc_block_scale, offset_world_cords);


        // Scale to size of block
        let draw_pos = [
            draw_cords[0] - self.ndc_block_scale, 
            draw_cords[1] - self.ndc_block_scale,
            draw_cords[0] + self.ndc_block_scale, 
            draw_cords[1] + self.ndc_block_scale,
        ];
        
        texture_manager.render_texture(texture, draw_pos);

        let sprite_depth = iso_cord_tool::get_depth_from_world_cords(iso_cord_tool::world_pos_to_world_cords(world_pos));
        let flattened_cords = iso_cord_tool::world_pos_to_tile_cords(offset_world_cords);

        for x in -3..3 {
            for y in -3..3 {
                let cords = [
                    flattened_cords[0] + x,
                    flattened_cords[1] + y,
                ];
                let tile = self.tile_map_manager.get_tile_with_flattened_cords(&cords);
                
                if let Some(tile) = tile {
                    let left_tile_world_cords = tile.get_left_triangle().get_first_solid_block_cords_struck();
                    let left_tile_depth = iso_cord_tool::get_depth_from_world_cords(left_tile_world_cords);
                    let re_render_left = left_tile_depth > sprite_depth;

                    let right_tile_world_cords = tile.get_right_triangle().get_first_solid_block_cords_struck();
                    let right_tile_depth = iso_cord_tool::get_depth_from_world_cords(right_tile_world_cords);
                    let re_render_right = right_tile_depth > sprite_depth;

                    
                    if re_render_left {
                        self.render_left_triangle(texture_manager, &cords, &tile);
                    }
                    if re_render_right {
                        self.render_right_triangle(texture_manager, &cords, &tile);
                    }
                }
            }
        }

        self.entitys_drawn += 1;
    }

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

    fn render_full_view(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {

        
        self.size_play_view(player_data.get_cursor());

        let world_arc = player_data.get_world_ref();
        let mut world = world_arc.write().unwrap();
    
        

        let cursor = player_data.get_cursor();

        self.camera_ndc_offset = iso_cord_tool::world_pos_to_ndc_cords(
            self.ndc_block_scale, 
            iso_cord_tool::world_cords_to_world_pos(cursor.get_cords())
        );

        let world_area = cursor.get_rendering_area();
        self.area_rendering_manager.set_world_area(world_area);
        

        let test_world_chunk = world.get_chunk_at_world_cords_mut(cursor.get_cords());
        

        let mut draw_cords = [0.0; 2];
        draw_cords[0] += self.ndc_draw_centering_offset[0];
        draw_cords[1] += self.ndc_draw_centering_offset[1];

        draw_cords[0] -= self.camera_ndc_offset[0];
        draw_cords[1] -= self.camera_ndc_offset[1];

        self.tile_map_manager.update_rendering_data(self.ndc_block_scale, draw_cords);


        texture_manager.update_expander_cache(self.ndc_block_scale);
        if (cursor.get_zoom() * 2) > 16 {
            let chunk_cords = test_world_chunk.get_cords();
            for x in -2..2 {
                for y in -2..2 {
                    for z in -2..2 {
                        let chunk_index = [
                            chunk_cords[0] + x,
                            chunk_cords[1] + y,
                            chunk_cords[2] + z,
                        ];

                        world.render_chunk(
                            texture_manager,
                            &mut self.tile_map_manager, 
                            chunk_index
                        );
                    }
                }

            }
            self.tile_map_manager.flatten_lairs();
        }
        else {
            self.tile_map.reset(0);
            self.tile_map.ray_cast_world_area(world_area, &world);
            self.tile_map.render(texture_manager, self.ndc_block_scale, draw_cords);
        }
        
    }

    //=====================================
    // Getters
    //=====================================

    pub fn get_tile_map_manager(&mut self) -> &mut TileMapManager {
        &mut self.tile_map_manager
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
        let start = Instant::now();

        // Render View
        self.render_full_view(texture_manager, screen_data, event_manager, &player_data);        
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
                    self.get_mouse_triangle(screen_data, event_manager, player_data);

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
                    let drone_event_scheduler = player_data.get_drone_event_scheduler(drone_id);
                    

                    if let Some(mut drone_event_scheduler) = drone_event_scheduler {
                        let mut cursor_event_scheduler = player_data.get_cursor_event_scheduler();
                            
                        let drone = drone_event_scheduler.get_drone();
                        self.render_enitity_at_world_pos(texture_manager, player_data, drone.get_world_pos(), drone.get_texture());

                        cursor_event_scheduler.set_cords(drone.get_cords());

                        let mouse_triangle = self.get_mouse_triangle(screen_data, event_manager, player_data);
                        
                        if let Some(mouse_triangle) = mouse_triangle {
                            let mut cords = mouse_triangle.get_first_solid_block_cords_struck();
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
                ViewMode::Location(_location_id) => {

                },
            }
            self.overlay_panel.render(texture_manager, screen_data, event_manager, player_data);
        }
        else {
            self.handle_camera_panning(screen_data, event_manager, player_data);
            self.handle_camera_zooming(screen_data, event_manager, player_data);
            self.overlay_panel = Panel::new_blank();
        }
        

        let frame_time = start.elapsed().as_secs_f32() * 1000.0;

        let debug = event_manager.get_mut_debug_data().get_rendering_debug_data();
        debug.frame_time_ms = frame_time;
        
        debug.tiles_cashed = 0;
        debug.tiles_rendered = 0;

        debug.chunks_cashed = 0;
        debug.chunks_rendered = 0;
        
        debug.total_lairs = self.tile_map_manager.get_lairs().len();

        debug.entitys_drawn = self.entitys_drawn;
        self.entitys_drawn = 0;

        
    }


    
}
