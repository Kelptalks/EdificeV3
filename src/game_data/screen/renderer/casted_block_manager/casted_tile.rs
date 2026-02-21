use image::imageops::FilterType::Triangle;

use crate::game_data::{TextureManager, screen::renderer::camera_data::CameraData, types::{BlockTriangle, BlockTexture}};
use super::{casted_chunk::CastedChunk, casted_triangle::CastedTriangle};
use super::super::{iso_cord_tool};

#[derive(Clone)]
pub struct CastedTile
{
    //Cords
    casted_cor : [i32; 2],

    world_camera_cords : [i32 ; 3],

    //Left Triangle
    left_triangle : CastedTriangle,
    right_triangle : CastedTriangle,
    

}

impl CastedTile {
    pub fn new(casted_cor : [i32; 2]) -> Self {
        // World cords : the place where the ray will be cast from
        
        // Iso Cords : The isometric rendering location
        
        // Draw Cords : The location the tile should be drawn

        
        
        Self{
            //TileData
            casted_cor : casted_cor,
            world_camera_cords: [0, 0, 0],

            //Triangles
            left_triangle : CastedTriangle::new(),
            right_triangle : CastedTriangle::new(),            
        }
    }

    pub fn get_cam_world_cords(&self) -> [i32; 3]{
        return self.world_camera_cords;
    }

    pub fn get_triangles(&self) -> [&CastedTriangle; 2] {
        return [&self.left_triangle, &self.right_triangle];
    }


    pub fn get_mut_triangles(&mut self) -> [&mut CastedTriangle; 2] {
        return [&mut self.left_triangle, &mut self.right_triangle];
    }

    pub fn set_world_camera_cords(&mut self, cords : [i32; 3]) {
        self.world_camera_cords = cords;
    }

    pub fn reset_casting_values(&mut self) {
        self.left_triangle.clear_triangle();
        self.right_triangle.clear_triangle();

    }

    pub fn render_tile(&self, camera_data : &CameraData, texture_manager : &mut TextureManager)
    {
        // If tile has no textures add default clouds
        let scale = camera_data.get_tile_ndc_scale();

        let draw_cords = iso_cord_tool::casted_to_ndc_cords(scale, self.casted_cor);
        let draw_cam_offset = camera_data.get_ndc_draw_offset();

        let final_left_draw_cords = [draw_cords[0] + draw_cam_offset[0], draw_cords[1] + draw_cam_offset[1]];
        let final_right_draw_cords = [(draw_cords[0] + draw_cam_offset[0]) + (scale), draw_cords[1] + draw_cam_offset[1]];

        //Render Triangles
        self.left_triangle.render_casted_triangle(texture_manager, final_left_draw_cords, scale);
        self.right_triangle.render_casted_triangle(texture_manager, final_right_draw_cords, scale);
    }

    
}
