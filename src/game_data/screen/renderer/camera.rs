use std::{clone, sync::{Arc, RwLock}};

use crate::game_data::{TextureManager, World, screen::{self, camera_data::CameraData, renderer::{thread_manager::raycast_thread_pool::RaycastThreadPool}}, types::{BlockTriangle, BlockType}};
use super::casted_block_manager::casted_block_manager::CastedChunkManager;

pub struct Camera
{
    camera_data : CameraData,
    casted_chunk_manager : CastedChunkManager,
    thread_manager: RaycastThreadPool,
}

impl Camera {
    pub fn new() -> Self {
        let camera_data = CameraData::new();

        Self {
            camera_data : camera_data,
            casted_chunk_manager : CastedChunkManager::new(),
            thread_manager: RaycastThreadPool::new(4),
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

    pub fn render_camera(&mut self, texture_manager : &mut TextureManager, world : &RwLock<World>) {
        
        self.get_mut_camera_data().update_camera_values();
        self.get_mut_camera_data().increment_frame_number();

        texture_manager.update_expander_cache(self.camera_data.get_render_scale());


        let world_guard = world.read().unwrap();

        let camera_data = self.get_mut_camera_data().clone();

        let iso_sceen_center = [
            camera_data.get_iso_cam_center()[0] as i32,
            camera_data.get_iso_cam_center()[1] as i32,
        ];

        let iso_chunk_center = CastedChunkManager::get_chunk_cords_from_tile_cords(iso_sceen_center);

        for x_rel_cor in -3..3 {
            for y_rel_cor in -3..3 {
                let relative_chunk_cords = [
                    iso_chunk_center[0] + x_rel_cor,
                    iso_chunk_center[1] + y_rel_cor
                ];

                if let Some(_chunk) = self.get_mut_casted_chunk_manager().get_mut_chunk_at_chunk_cords(relative_chunk_cords) {
                    // Chunk already exists
                    _chunk.render_chunk(&camera_data, texture_manager, &world_guard);
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
        if cords[0] >= 0 && cords[1] >= 0 {
            return 1;
        }
        else if cords[0] < 0 && cords[1] >= 0 {
            return 2;
        }
        else if cords[0] >= 0 && cords[1] < 0 {
            return 3;
        }
        else if cords[0] < 0 && cords[1] < 0 {
            return 4;
        }
        else
        {
            return 5;
        }
    }

}
