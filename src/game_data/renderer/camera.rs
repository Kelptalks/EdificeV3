use crate::game_data::{renderer::casted_block_manager::casted_block_manager::CastedChunkManager, TextureManager, World, Types::{BlockTriangle, BlockType}};

#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

pub struct CameraData {
    window_rez : [f32; 2],

    cam_world_cords : [f32 ; 3],

    //Camera offset by controls
    render_cords : [f32; 2],
    //Camera offset for centering
    draw_offset : [f32 ; 2],


    render_scale : f32,
    tile_pixel_scale : f32,


    // Ray Casting
    direction : Direction,
    direction_mods : [i32; 3],
    draw_distance : usize,
}

impl CameraData {
    pub fn new() -> Self {
        Self {
            // Window info
            window_rez : [1920.0, 1080.0],

            // World cords
            cam_world_cords : [100.0, 100.0, 100.0],
            
            // Pixel Drawing Offsets
            draw_offset : [0.0, 0.0],
            render_cords : [0.0, 0.0],
            
            // Render scaling
            render_scale : 0.0005,
            tile_pixel_scale : 32.0,

            // Ray casting
            direction : Direction::North,
            direction_mods : [1, 1, 1],
            draw_distance : 100,
        }
    }

    pub fn get_window_rez(&self) -> [f32; 2] {
        return self.window_rez;
    }

    pub fn set_screen_rez(&mut self, new_rez: [f32; 2]) {
        self.window_rez = new_rez;
    }

    // Ray Casting getters
    pub fn get_draw_distance(&self) -> usize
    {
        return self.draw_distance;
    }
    pub fn get_direction_mods(&self) -> [i32; 3]
    {
        return self.direction_mods;
    }
    pub fn get_direction(&self) -> &Direction {
        return &self.direction
    }

    pub fn mod_x_cam_cor(&mut self, x_mod : f32) {
        self.render_cords[0] += x_mod;
    }
    pub fn mod_y_cam_cor(&mut self, y_mod : f32) {
        self.render_cords[1] += y_mod;
    }

    pub fn mod_scale(&mut self, zoom_mod: f32) {
        self.render_scale *= zoom_mod;
    }

    pub fn get_draw_offset(&self) -> [f32 ; 2]{

        let x_draw_offset = self.render_cords[0] + self.draw_offset[0];
        let y_draw_offset = self.render_cords[1] + self.draw_offset[1];

        return [x_draw_offset, y_draw_offset];
    }

    pub fn get_cam_world_cords(&self) -> [f32 ; 3] {
        return self.cam_world_cords;
    }

    pub fn get_render_scale(&self) -> f32 {
        return self.render_scale;
    }

    pub fn get_tile_render_scale(&self) -> f32 {
        return self.tile_pixel_scale;
    }

}

pub struct Camera
{
    camera_data : CameraData,
    casted_chunk_manager : CastedChunkManager,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            camera_data : CameraData::new(),
            casted_chunk_manager : CastedChunkManager::new(),
        }
    }

    pub fn get_camera_data(&self) -> &CameraData {
        return &self.camera_data;
    }

    pub fn get_casted_chunk_manager(&self) -> &CastedChunkManager {
        return &self.casted_chunk_manager;
    }

    pub fn get_mut_casted_chunk_manager(&mut self) -> &mut CastedChunkManager {
        return &mut self.casted_chunk_manager;
    }

    pub fn get_mut_camera_data(&mut self) -> &mut CameraData {
        return &mut self.camera_data;
    }

    pub fn render_camera(&mut self, texture_manager : &mut TextureManager, world : &World) {
        
        texture_manager.update_expander_cache(self.camera_data.get_render_scale());
        self.casted_chunk_manager.render_all_chunks(&self.camera_data, texture_manager, world);

        
    }

    pub fn get_quadrent_of_cords(cords : [i32; 2]) -> usize
    {
        // Identify the quadrent the chunk is located in and invert based on it
        if (cords[0] >= 0 && cords[1] >= 0) {
            return 1;
        }
        else if (cords[0] < 0 && cords[1] >= 0) {
            return 2;
        }
        else if (cords[0] >= 0 && cords[1] < 0) {
            return 3;
        }
        else if (cords[0] < 0 && cords[1] < 0) {
            return 4;
        }
        else
        {
            return 5;
        }
    }

    pub fn create_casted_chunks(&mut self)
    {
        for x in -3..3 {
            for y in -3..3  {
                self.casted_chunk_manager.create_chunk_at_cords(&self.camera_data, [x, y]);
            }
        }
    }
}
