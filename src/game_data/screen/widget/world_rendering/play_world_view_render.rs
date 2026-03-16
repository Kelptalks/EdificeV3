use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use miniquad::KeyCode;

use crate::game_data::{TextureManager, World, game_event_manager::game_event_manager::GameEventManager, screen::{ScreenData, camera_data::Direction, iso_cord_tool, widget::{panel::panel::Panel, widget::Widget, widget_calculations, world_rendering::play_block::PlayBlock}}, types::BlockTexture};

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


    // World Rendering
    world_ref: Option<Arc<RwLock<World>>>,
    camera_cords_ref: Option<Rc<RefCell<[i32; 3]>>>,
    camera_direction: ViewDirection,

    // Camera Motion
    zoom: i32,
    camera_ndc_offset: [f32; 2],
    render_scale: f32,

    // Cached scale values
    ndc_block_scale: f32,
    ndc_tile_scale: f32,
    ndc_tile_half_scale: f32,
}

impl PlayWorldViewRender {
    pub fn new(parent_pos: [f32; 4], buffers: [f32; 4]) -> PlayWorldViewRender{
        PlayWorldViewRender {            
           // Parent Rendering
            parent_pos: parent_pos,
            parent_scale: [0.0; 2],
            prefered_scale: [1.0; 2],

            // Self Rendering
            external_buffers: buffers, 
            internal_buffers: [0.012; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],

            center_ndc: [0.0; 2],


            // Player Data Links
            world_ref: None,
            camera_cords_ref: None,

            camera_direction: ViewDirection::North,

            // Rendering
            zoom: 10,
            camera_ndc_offset: [0.0, 0.0],
            render_scale: 0.5,

            ndc_block_scale: 0.0,
            ndc_tile_scale: 0.0,
            ndc_tile_half_scale: 0.0,
        }
    }

    //=====================================
    // Controls
    //=====================================
    
    fn handle_camera_panning(&mut self, screen_data: &ScreenData) {
        // Update camera offset based off scrolling change
        if screen_data.is_middle_mouse_held() {
            let scrolling_offset = screen_data.get_change_in_mouse_ndc();
            self.camera_ndc_offset[0] += scrolling_offset[0];
            self.camera_ndc_offset[1] += scrolling_offset[1];
        }

        if let Some(cords_ref) = &self.camera_cords_ref {
            // Get iso offset amounts
            let iso_offset = 
                iso_cord_tool::ndi_screen_cords_to_iso_cords(self.ndc_tile_scale, self.camera_ndc_offset);

            // Iso X camera movment
            if iso_offset[0] > 1.0 {
                cords_ref.borrow_mut()[0] -= 1;
                self.camera_ndc_offset[0] -= self.ndc_tile_scale;
                self.camera_ndc_offset[1] -= self.ndc_tile_half_scale;
            }
            if iso_offset[0] < -1.0 {
                cords_ref.borrow_mut()[0] += 1;
                self.camera_ndc_offset[0] += self.ndc_tile_scale;
                self.camera_ndc_offset[1] += self.ndc_tile_half_scale;
            }

            // Iso Y Cam movment
            if iso_offset[1] > 1.0 {
                cords_ref.borrow_mut()[1] -= 1;
                self.camera_ndc_offset[0] += self.ndc_tile_scale;
                self.camera_ndc_offset[1] -= self.ndc_tile_half_scale;
            }

            if iso_offset[1] < -1.0 {
                cords_ref.borrow_mut()[1] += 1;
                self.camera_ndc_offset[0] -= self.ndc_tile_scale;
                self.camera_ndc_offset[1] += self.ndc_tile_half_scale;
            }
        }
        else {
            panic!("No refrence player cords was provided for render view");
        }
    

    }

    fn zoom_in(&mut self) {
        if self.zoom > 1 {
            self.zoom-=1;
        }
    }

    fn zoom_out(&mut self) {
        self.zoom+=1;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn size(&mut self) {
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
        
        self.center_ndc = [
            self.pos[0] + (self.scale[0]),
            self.pos[1] + (self.scale[1] / 2.0),
        ];


        self.ndc_block_scale = (self.scale[0] / ((self.zoom * 2) + 1) as f32) / 2.0;
        self.ndc_tile_scale = self.ndc_block_scale / 2.0;
        self.ndc_tile_half_scale = self.ndc_tile_scale / 2.0;

    }

    pub fn render_view(&mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager,
    ) {
        self.size();
        self.handle_camera_panning(screen_data);

        let world;
        if let Some(world_ref) = &self.world_ref {
            world = world_ref.read().unwrap();
        }
        else {
            panic!("No refrence world was provided for render view");
        }


        let ndc_x_draw_center_offset = self.center_ndc[0] + self.ndc_block_scale;
        let ndc_y_draw_center_offset = self.center_ndc[1] - self.ndc_block_scale;

        let rot = self.camera_direction.rotation_matrix();
        
        let camera_cords;
        if let Some(player_cords_ref) = &self.camera_cords_ref {
            camera_cords = *player_cords_ref.borrow();
        }
        else {
            panic!("No refrence player cords was provided for render view");
        }


        // Loop through blocks in zoom
        for z in -self.zoom..=self.zoom {
            for y in -self.zoom..=self.zoom {
                for x in -self.zoom..=self.zoom {
                    let world_block_cords = [
                        camera_cords[0] + (rot[0][0] * x + rot[0][1] * y),
                        camera_cords[1] + (rot[1][0] * x + rot[1][1] * y),
                        camera_cords[2] + z,
                    ];

                    // Block Type
                    let block_type_at_cord = BlockTexture::from_id(world.get_world_value(world_block_cords));
                    
                    // Draw Cords
                    let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(self.ndc_block_scale, [x - z, y - z]);
                    
                    draw_cords[0] -= ndc_x_draw_center_offset;
                    draw_cords[1] += ndc_y_draw_center_offset;
                    
                    draw_cords[0] += self.camera_ndc_offset[0];
                    draw_cords[1] += self.camera_ndc_offset[1];

                    // Create a play block if block needs to be rendered
                    let mut play_block = PlayBlock::new_blank();

                    // World
                    play_block.block_type = block_type_at_cord;
                    play_block.block_world_cords = world_block_cords;

                    // Rendering
                    play_block.rendering_block_cords = [x, y, z];
                    play_block.draw_cords = draw_cords;
                    play_block.ndc_block_scale = self.ndc_block_scale;


                    play_block.render_block(texture_manager);
                    play_block.render_cursor(texture_manager);

                }
            }
        }


        /*
        if screen_data.is_left_mouse_held() || screen_data.is_right_mouse_held() {
            play_view_data.get_mut_cursor_area().set_point_1(camera_cords);
        }
        else {
            play_view_data.get_mut_cursor_area().set_point_1(camera_cords);
            play_view_data.get_mut_cursor_area().set_point_2(camera_cords);
        }

        */


    }

    //=====================================
    // Linking
    //=====================================

    pub fn link_world_ref(&mut self, world_ref: Arc<RwLock<World>>) {
        self.world_ref = Some(world_ref);
    }

    pub fn link_camera_world_cords_ref(&mut self, world_cords_ref: Rc<RefCell<[i32; 3]>>) {
        self.camera_cords_ref = Some(world_cords_ref)
    }


    //=====================================
    // Settings
    //=====================================

    pub fn set_prefered_size(&mut self, scale: f32) {
        self.parent_scale = [scale; 2];
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
        game_event_manager: &mut GameEventManager
    ) {        
        self.render_view(screen_data, texture_manager);


        
        let inputs = screen_data.get_inputs();
        for input in inputs {
            match input {
                crate::game_data::screen::input_data::Input::KeyDown(key_code) => {
                    if let Some(cords_ref) = &self.camera_cords_ref {
                        if *key_code == KeyCode::LeftShift {
                            cords_ref.borrow_mut()[2] -= 1;
                        }
                        else if *key_code == KeyCode::Space {
                            cords_ref.borrow_mut()[2] += 1;
                        }
                    }
                },
                crate::game_data::screen::input_data::Input::MouseWheel(x, y) => {
                    if *y > 0.0 {
                        self.zoom_in();
                    }
                    else if *y < 0.0 { 
                        self.zoom_out();
                    }
                },
                _ => {

                }

            }
        }
        

    }
}