use std::{cell::RefCell, rc::Rc};


use crate::game_data::{TextureManager, locations::world_area::WorldArea, ray_caster::ray::TileRay, screen::{ScreenData, iso_cord_tool, widget::{button::button::Button, prelude::play_world_view_config::PlayViewRenderingConfig, widget::{Widget, WidgetType}, widget_calculations, world_rendering::{area_rendering_manager::{self, area_rendering_manager::AreaRenderingManager, play_block::PlayBlock}, play_view_control_manager::PlayViewControlManager}}}, types::{BlockTexture, UITextures}};

use crate::game_data::game_event_manager::prelude::*;

/*
###############
## Direction ##
###############
Enum for controlling the cameras direction
main use is to get direction modify values
*/
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
    // Parent rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Self Rendering
    external_buffers: [f32; 4],  
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    scale: [f32; 2],

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
}

impl PlayWorldViewRender {
    pub fn new(play_view_rendering_config: Rc<RefCell<PlayViewRenderingConfig>>) -> PlayWorldViewRender{


        let config = play_view_rendering_config.borrow();
        let input_manager = PlayViewControlManager::new(
            config.get_camera_movement_event_type_ref(),
            config.get_cursor_location_ref()
        );
        drop(config);

        PlayWorldViewRender {            
           // Parent Rendering
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],
            prefered_scale: [1.0; 2],

            // Self Rendering
            external_buffers: [0.0; 4], 
            internal_buffers: [0.012; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],

            center_ndc: [0.0; 2],


            // Input | Handling
            events: Vec::new(),

            // Player Data Links
            rendering_config: play_view_rendering_config,

            camera_direction: ViewDirection::North,

            // Rendering
            camera_ndc_offset: [0.0, 0.0],

            ndc_block_scale: 0.0,
            ndc_tile_scale: 0.0,
            ndc_tile_half_scale: 0.0,
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
        // Update camera offset based off scrolling change
        if screen_data.is_middle_mouse_held() {
            let scrolling_offset = screen_data.get_change_in_mouse_ndc();
            self.camera_ndc_offset[0] += scrolling_offset[0];
            self.camera_ndc_offset[1] += scrolling_offset[1];
        }

        let mut cords_offset = [0; 3];

        // Get iso offset amounts
        let iso_offset = 
            iso_cord_tool::ndi_screen_cords_to_iso_cords(self.ndc_tile_scale, self.camera_ndc_offset);

        // Iso X camera movment
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

        // Iso Y Cam movment
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

    /// Size blocks based off zoom level and the space avalible to the widget
    /// 
    /// Why: The size of blocks scale needs to be ajusted based off the space avalible and
    /// the area requried by the rendering
    /// 
    pub fn size(&mut self) {
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
        
        self.center_ndc = [
            self.pos[0] + (self.scale[0] / 2.0),
            self.pos[1] + (self.scale[1] / 2.0),
        ];


        let config = self.rendering_config.borrow();
        let largest_side_of_location = config.get_cursor_location_ref().borrow().get_area().get_largest_dimension_scale();
        let block_diementions = largest_side_of_location + config.get_zoom() * 2 + 1;

        self.ndc_block_scale = (self.scale[0] / block_diementions as f32) / 2.0;
        self.ndc_tile_scale = self.ndc_block_scale / 2.0;
        self.ndc_tile_half_scale = self.ndc_tile_scale / 2.0;

    }

    /// Get the starting and ending points render view should loop through
    /// 
    /// Why : Based of the location's area and rendering configs zoom level
    /// the area's shape needs to be determined.
    /// 
    pub fn get_area_to_render(&self) -> [[i32; 3]; 2] {
        let config = self.rendering_config.borrow();
        let half_dimensions = config.get_cursor_location_ref().borrow().get_area().get_half_dimensions();

        let mut start_cords = half_dimensions.map(|d| -d);
        for axis in &mut start_cords {
            *axis -= config.get_zoom();
        }

        let mut end_cords = half_dimensions;
        for axis in &mut end_cords {
            *axis += config.get_zoom();
        }
        return [start_cords, end_cords];
    }

    pub fn get_rendering_center_world_cor(&self) -> [i32; 3] {
        let config = self.rendering_config.borrow();
        return config.get_cursor_location_ref().borrow().get_area().get_center_world_cords();
    }

    pub fn ray_cast_view(&mut self, texture_manager: &mut TextureManager) {
        self.size();

        let world_arc = self.rendering_config.borrow().get_world_ref().clone();
        let world = world_arc.read().unwrap();

        let ndc_x_draw_center_offset = self.center_ndc[0] - self.ndc_block_scale;
        let ndc_y_draw_center_offset = self.center_ndc[1] - self.ndc_block_scale;

        let rot = self.camera_direction.rotation_matrix();

        let camera_cords = self.get_rendering_center_world_cor();

        let area_to_render = self.get_area_to_render();
        
        // Create a world area and shift in the correct location
        let mut world_area = WorldArea::new_with_cords(area_to_render);
        world_area.shift_cords(camera_cords);

        let tiles = AreaRenderingManager::get_casted_tile_rays(&world_area, &world);
        for tile in tiles {
            let tile_area_cords = tile.get_area_cords();

            
            let mut draw_iso_cords = [
                tile_area_cords[0] - tile_area_cords[2],
                tile_area_cords[1] - tile_area_cords[2],
            ];
            

            let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(self.ndc_block_scale, draw_iso_cords);
            draw_cords[0] += ndc_x_draw_center_offset;
            draw_cords[1] += ndc_y_draw_center_offset;
            
            draw_cords[0] += self.camera_ndc_offset[0];
            draw_cords[1] += self.camera_ndc_offset[1];

            let [left_textures, right_textures] = tile.get_tile_textures();

            // Left side 
            let left_pos = [
                draw_cords[0],
                draw_cords[1],
                draw_cords[0] + self.ndc_block_scale,
                draw_cords[1] + self.ndc_block_scale,
            ];
            for texture in left_textures {
                texture_manager.render_texture_with_pos(texture, left_pos);
            }

            // Right side 
            let right_pos = [
                draw_cords[0] + self.ndc_block_scale,
                draw_cords[1],
                draw_cords[0] + (self.ndc_block_scale * 2.0),
                draw_cords[1] + self.ndc_block_scale,
            ];
            for texture in right_textures {
                texture_manager.render_texture_with_pos(texture, right_pos);
            }
        }

    }

    pub fn render_view(
        &mut self,
        texture_manager: &mut TextureManager,
    ) {
        self.size();

        let world_arc = self.rendering_config.borrow().get_world_ref().clone();
        let block_ghost = self.rendering_config.borrow().get_block_ghost();
        let location_rc = self.rendering_config.borrow().get_cursor_location_ref().clone();

        let world = world_arc.read().unwrap();

        let ndc_x_draw_center_offset = self.center_ndc[0] - self.ndc_block_scale;
        let ndc_y_draw_center_offset = self.center_ndc[1] - self.ndc_block_scale;

        let rot = self.camera_direction.rotation_matrix();
        
        let camera_cords = self.get_rendering_center_world_cor();

        let area_to_render = self.get_area_to_render();
        let start_cords = area_to_render[0];
        let end_cords = area_to_render[1];

        // Pass 1: Build all play blocks
        let mut play_blocks: Vec<PlayBlock> = Vec::new();

        for z in start_cords[2]..=end_cords[2] {
            for y in start_cords[1]..=end_cords[1] {
                for x in start_cords[0]..=end_cords[0] {
                    let world_block_cords = [
                        camera_cords[0] + (rot[0][0] * x + rot[0][1] * y),
                        camera_cords[1] + (rot[1][0] * x + rot[1][1] * y),
                        camera_cords[2] + z,
                    ];

                    let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(self.ndc_block_scale, [x - z, y - z]);
                    draw_cords[0] += ndc_x_draw_center_offset + self.camera_ndc_offset[0];
                    draw_cords[1] += ndc_y_draw_center_offset + self.camera_ndc_offset[1];

                    let mut play_block = PlayBlock::new_blank();
                    play_block.block_type = BlockTexture::from_id(world.get_world_value(world_block_cords));
                    play_block.block_world_cords = world_block_cords;
                    play_block.rendering_block_cords = [x, y, z];
                    play_block.draw_cords = draw_cords;
                    play_block.ndc_block_scale = self.ndc_block_scale;

                    play_blocks.push(play_block);
                }
            }
        }

        // Pass 2: Render in order
        let config = self.rendering_config.borrow();
        let locations_to_render = config.get_locations_to_render().borrow();
        let render_all_locations = config.should_render_all_locations();

        for play_block in play_blocks.iter() {
            play_block.render_block(texture_manager);
            play_block.render_cursor(texture_manager, block_ghost);

            if render_all_locations {
                for location in locations_to_render.iter() {
                    play_block.render_area(texture_manager, location.borrow().get_area());
                }
            }
        }

    }

    //=====================================
    // Settings
    //=====================================

    pub fn set_prefered_size(&mut self, scale: f32) {
        self.prefered_scale = [scale; 2];
    }

}

impl Widget for PlayWorldViewRender {
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }

    fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.external_buffers = pos;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
    }

    fn size(&mut self) {
        self.size();
    }

    fn render(
        &mut self, 
        texture_manager: &mut TextureManager, 
        screen_data: &ScreenData, 
        game_event_manager: &mut EventManager
    ) {
        
        self.render_view(texture_manager,);
        
        // self.ray_cast_view(texture_manager);

        // Don't handle input if mouse is not on render
        if !screen_data.mouse_on_ndc_pos(self.pos) { 
            return;
        }
        else {
            game_event_manager.add_events(&self.events);
        }

        self.handle_camera_panning(screen_data, game_event_manager);
    }
}