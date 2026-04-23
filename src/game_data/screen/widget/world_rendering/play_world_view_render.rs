use std::{cell::RefCell, ops::Index, rc::Rc};


use crate::game_data::{
    TextureManager, game_event_manager::player_data_event_manager::var_event_manager::var_events::VarEvents, locations::world_area::WorldArea, player_data::drone_script::var::{game_vars::{dynamic_var::{self, DynamicVarType}, game_var_type::GameVarType}, var_type::VarType}, screen::{
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

    fn handle_camera_panning(&mut self, screen_data: &ScreenData, game_event_manager: &mut EventManager) {
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

        self.rendering_config.borrow().get_cursor_config().add_move_cursor_event_with_shift_mod(game_event_manager, cords_offset);
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
        let largest_side_of_location = config.get_cursor_location_ref().borrow().get_area().get_largest_dimension_scale();
        let block_diementions = largest_side_of_location + config.get_zoom() * 2 + 1;

        self.ndc_block_scale = (scale[0] / block_diementions as f32) / 2.0;
        self.ndc_tile_scale = self.ndc_block_scale / 2.0;
        self.ndc_tile_half_scale = self.ndc_tile_scale / 2.0;

        self.ndc_draw_centering_offset[0] = self.center_ndc[0] - self.ndc_block_scale;
        self.ndc_draw_centering_offset[1] = self.center_ndc[1] - self.ndc_block_scale;
    }

    pub fn get_rendering_center_world_cor(&self) -> [i32; 3] {
        let config = self.rendering_config.borrow();
        return config.get_cursor_location_ref().borrow().get_area().get_center_world_cords();
    }

    fn world_to_area_cords(&self, cords: [i32; 3]) -> [i32; 3] {
        cords_tool::diff_cords(
            cords,
            self.get_rendering_center_world_cor()
        )
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

    pub fn get_world_area_of_view(&self) -> WorldArea {
        let config = self.rendering_config.borrow();
        let camera_cords = self.get_rendering_center_world_cor();

        let mut p1 = [-config.get_zoom(); 3];
        let mut p2 = [config.get_zoom(); 3];
        for (axis, axis_cord) in camera_cords.iter().enumerate() {
            p1[axis] += axis_cord;
            p2[axis] += axis_cord;
        }

        return WorldArea::new_with_cords([p1, p2])
    }

    pub fn ray_cast_view(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager
    ) {
        self.size();

        let world_arc = self.rendering_config.borrow().get_world_ref().clone();
        let world = world_arc.read().unwrap();

        

        
        
        let config = self.rendering_config.borrow();
        

        let world_area;
        
        match config.get_render_mode() {
            rendering_config::play_world_view_config::RenderMode::All => {
                world_area = self.get_world_area_of_view()
            },
            rendering_config::play_world_view_config::RenderMode::VarOnly(var) => {
                if let Some(location_ref) = var.as_location() {
                    println!("test");
                    world_area = *location_ref.borrow().get_area()
                }
                else {
                    println!("bad test");
                    world_area = self.get_world_area_of_view()
                }
            },
        }

        let mut area_rendering_manager = AreaRenderingManager::new(&world_area);
        let tiles = area_rendering_manager.get_casted_tile_rays(&world, &*config);

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
                texture_manager.render_texture(texture, left_pos);
            }

            let right_pos = [
                draw_cords[0] + self.ndc_block_scale,
                draw_cords[1],
                draw_cords[0] + (self.ndc_block_scale * 2.0),
                draw_cords[1] + self.ndc_block_scale,
            ];
            for texture in right_textures {
                texture_manager.render_texture(texture, right_pos);
            }
        }

        for var in config.get_vars_to_render() {
            let borrow = var.get_var_type_ref().borrow();
            if let VarType::Game(GameVarType::Dynamic(dynamic_var)) = &*borrow {
                if let DynamicVarType::Drone(Some(drone_ref_option)) = dynamic_var {
                    let area_cords = self.world_to_area_cords(drone_ref_option.borrow().get_cords());
                    let draw_cords = self.area_to_draw_cords(area_cords);



                }
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
        game_event_manager: &mut EventManager,
    ) {
        self.ray_cast_view(texture_manager, screen_data, game_event_manager);

        if !screen_data.mouse_on_ndc_pos(self.widget_properties.pos) {
            return;
        } else {
            game_event_manager.add_events(&self.events);
        }

        self.handle_camera_panning(screen_data, game_event_manager);
    }
}
