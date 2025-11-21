use std::{clone, sync::Arc};

use crate::game_data::{TextureManager, World, screen::{self, iso_cord_tool, renderer::{camera, casted_block_manager::casted_tile::CastedTile, thread_manager::raycast_thread_pool::RaycastThreadPool}}, types::{BlockTriangle, BlockType}};
use super::casted_block_manager::casted_block_manager::CastedChunkManager;

#[derive(PartialEq, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Clone)]
pub struct CameraData {
    window_rez : [f32; 2],

    cam_world_cords : [f32 ; 3],

    //Camera offset by controls
    render_cords : [f32; 2],
    //Camera offset for centering
    ndc_draw_offset : [f32 ; 2],

    // Render Location
    iso_cam_center_cords : [f32; 2],

    // Render Scaling
    zoom : f32,
    render_scale : f32,
    tile_pixel_scale : f32,
    tile_ndi_scale : f32,

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
            ndc_draw_offset : [0.0, 0.0],
            render_cords : [0.0, 0.0],
            
            // Renderer location
            iso_cam_center_cords : [0.0, 0.0],

            // Render scaling
            zoom : 1.0,
            tile_ndi_scale : 0.0,
            render_scale : 0.0005,
            tile_pixel_scale : 32.0,

            // Ray casting
            direction : Direction::North,
            direction_mods : [1, 1, 1],
            draw_distance : 200,
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

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
        self.zoom *= zoom_mod;

        //self.render_scale *= zoom_mod;

    }

    pub fn get_ndc_draw_offset(&self) -> [f32 ; 2]{
        let x_draw_offset = self.render_cords[0] + self.ndc_draw_offset[0];
        let y_draw_offset = self.render_cords[1] + self.ndc_draw_offset[1];

        return [x_draw_offset, y_draw_offset];
    }

    pub fn get_cam_world_cords(&self) -> [f32 ; 3] {
        return self.cam_world_cords;
    }

    pub fn get_render_scale(&self) -> f32 {
        return self.render_scale * self.zoom;
    }

    pub fn get_zoom(&self) -> f32 {
        return self.zoom;
    }

    pub fn get_tile_render_scale(&self) -> f32 {
        return self.tile_pixel_scale;
    }

    pub fn get_tile_ndi_scale(&self) -> f32 {
        return self.tile_ndi_scale;
    }

    pub fn get_tile_pixel_scale(&self) -> f32 {
        return self.tile_pixel_scale;
    }

    pub fn get_iso_cam_center(&self) -> [f32; 2] {
        return self.iso_cam_center_cords;
    }

}

pub struct Camera
{
    camera_data : CameraData,
    casted_chunk_manager : CastedChunkManager,
}

impl Camera {
    pub fn new(world : &World) -> Self {
        let camera_data = CameraData::new();


        Self {
            camera_data : camera_data,
            casted_chunk_manager : CastedChunkManager::new(),
        }
    }

    //=====================================
    // Getters
    //=====================================

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

    //=====================================
    // Rendering
    //=====================================

    pub fn update_camera_values(&mut self) {
        let camera_data = self.get_mut_camera_data();
        camera_data.tile_ndi_scale = camera_data.get_render_scale() * (BlockTriangle::PIXLE_REZ as f32);


        // Get iso world cords of center of stream
        let screen_ndi_center = [
            -self.camera_data.get_ndc_draw_offset()[0],
            -self.camera_data.get_ndc_draw_offset()[1],
        ];

        let screen_center_iso_cords = iso_cord_tool::ndi_screen_cords_to_iso_cords(
            self.camera_data.tile_ndi_scale,
            screen_ndi_center,
        );

        self.camera_data.iso_cam_center_cords = [
            screen_center_iso_cords[0],
            screen_center_iso_cords[1],
        ];

    }

    pub fn render_camera(&mut self, texture_manager : &mut TextureManager, world : &World) {
        
        self.update_camera_values();
        texture_manager.update_expander_cache(self.camera_data.get_render_scale());


        let camera_data = self.get_mut_camera_data().clone();

        let iso_sceen_center = [
            camera_data.get_iso_cam_center()[0] as i32,
            camera_data.get_iso_cam_center()[1] as i32,
        ];

        let iso_chunk_center = CastedChunkManager::get_chunk_cords_from_tile_cords(iso_sceen_center);

        //println!("Rendering Chunks around Iso Chunk Center: ({}, {})", iso_chunk_center[0], iso_chunk_center[1]);

        for x_rel_cor in -3..3 {
            for y_rel_cor in -3..3 {
                let relative_chunk_cords = [
                    iso_chunk_center[0] + x_rel_cor,
                    iso_chunk_center[1] + y_rel_cor
                ];

                if let Some(_chunk) = self.get_mut_casted_chunk_manager().get_mut_chunk_at_chunk_cords(relative_chunk_cords) {
                    // Chunk already exists
                    _chunk.render_chunk(&camera_data, texture_manager, world);
                }
                else {   
                    self.casted_chunk_manager.create_chunk_at_cords(&self.camera_data, relative_chunk_cords);

                }
            }
        }

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

}
