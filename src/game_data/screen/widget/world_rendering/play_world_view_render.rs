use std::{cell::RefCell, ops::Index, rc::Rc};


use crate::game_data::{
    TextureManager, game_event_manager::player_data_event_manager::var_event_manager::var_events::VarEvents, locations::world_area::WorldArea, player_data::{self, cursor::cursor_event_scheduler::CursorEventScheduler, drone_script::var::{game_vars::{dynamic_var::{self, DynamicVarType}, game_var_type::GameVarType}, var_type::VarType}, player_data::PlayerData}, screen::{
        ScreenData,
        iso_cord_tool,
        widget::{button::button::Button, prelude::{VarSlot, play_world_view_config::PlayViewRenderingConfig}, widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties, world_rendering::{area_rendering_manager::area_rendering_manager::AreaRenderingManager, rendering_config}}
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

    center_ndc: [f32; 2],

    // Input | Handling
    events: Vec<Event>,

    // World Rendering
    rendering_config: Rc<RefCell<PlayViewRenderingConfig>>,

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
    pub fn new(play_view_rendering_config: Rc<RefCell<PlayViewRenderingConfig>>) -> PlayWorldViewRender {

        let mut wp = WidgetProperties::new_blank();
        wp.prefered_scale = [1.0; 2];
        wp.internal_buffers = [0.012; 4];

        PlayWorldViewRender {
            widget_properties: wp,

            center_ndc: [0.0; 2],

            events: Vec::new(),

            rendering_config: play_view_rendering_config,

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


    fn handle_mouse(&mut self, screen_data: &ScreenData, event_manager: &mut EventManager, player_data: &PlayerData) {
        if self.mouse_on(screen_data) {
            let mouse_cords = screen_data.get_mouse_ndc();
            let mouse_play_view_cords = [
                self.ndc_draw_centering_offset[0] + mouse_cords[0],
                self.ndc_draw_centering_offset[1] + mouse_cords[1],
            ];

            let iso_mouse_cords = iso_cord_tool::ndi_screen_cords_to_iso_cords(self.ndc_block_scale, mouse_play_view_cords);
            
            println!("Mouse_Cords: {:?}", iso_mouse_cords);


            


        }
    }


    //=====================================
    // Rendering
    //=====================================

    pub fn size(&mut self) {
        self.widget_properties.scale_based_off_parent();

        let pos   = self.widget_properties.pos;
        let scale = self.widget_properties.scale;

        self.center_ndc = [
            pos[0] + (scale[0] / 2.0),
            pos[1] + (scale[1] / 2.0),
        ];

        let config = self.rendering_config.borrow();
        let largest_side_of_location = 1;
        let block_diementions = largest_side_of_location + config.get_zoom() * 2 + 1;

        self.ndc_block_scale = (scale[0] / block_diementions as f32) / 2.0;
        self.ndc_tile_scale = self.ndc_block_scale / 2.0;
        self.ndc_tile_half_scale = self.ndc_tile_scale / 2.0;

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
        self.size();

        let world_arc = self.rendering_config.borrow().get_world_ref().clone();
        let world = world_arc.read().unwrap();
    

        // Get Mouse Area Cords
        let mouse_cords = screen_data.get_mouse_ndc();
        let mouse_play_view_cords = [
            self.ndc_draw_centering_offset[0] + mouse_cords[0],
            self.ndc_draw_centering_offset[1] + mouse_cords[1],
        ];
        

        

        let cursor = player_data.get_cursor();
        let world_area = cursor.get_rendering_area();
        
        let mut area_rendering_manager = AreaRenderingManager::new(&world_area);
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
        self.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {

    
        self.ray_cast_view(texture_manager, screen_data, event_manager, &player_data);


        if !screen_data.mouse_on_ndc_pos(self.widget_properties.pos) {
            return;
        } else {
            event_manager.add_events(&self.events);
        }


        
        


        self.handle_camera_panning(screen_data, event_manager, player_data);
    }
}
