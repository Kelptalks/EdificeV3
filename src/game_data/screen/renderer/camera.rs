use std::{alloc::System, clone, sync::{Arc, RwLock}, time::SystemTime};

use miniquad::{GlContext, RenderingBackend};

use crate::game_data::{TextureManager, World, screen::{self, camera_data::CameraData, iso_cord_tool, render_string, renderer::{camera, casted_block_manager::{self, casted_chunk::{self, CastedChunk}}, render_cache_manager::{self, canvas_data, render_cashe_manager::RenderCacheManager}, thread_manager::raycast_thread_pool::RaycastThreadPool}}, types::{BlockTriangle, BlockType}};
use super::casted_block_manager::casted_block_manager::CastedChunkManager;

pub struct Camera
{
    initialized : bool,

    camera_data : CameraData,
    casted_chunk_manager : CastedChunkManager,
    thread_manager: RaycastThreadPool,
    render_cache_manager : Option<RenderCacheManager>,
}

impl Camera {
    pub fn new() -> Self {
        let camera_data = CameraData::new();

        Self {
            initialized : false,

            camera_data : camera_data,
            casted_chunk_manager : CastedChunkManager::new(),
            thread_manager: RaycastThreadPool::new(10),
            render_cache_manager : None,
        }
    }
    
    // initialize the camera
    pub fn initialize_camera(&mut self, ctx: &mut miniquad::GlContext)
    {
        self.render_cache_manager = Some(RenderCacheManager::new(ctx));
        self.initialized = true;
    }

    //=====================================
    // Setters
    //=====================================

    pub fn update_viewport(&mut self, rez: [f32; 2], offset: [f32; 2]) {
        self.camera_data.set_viewport_rez(rez);
        self.camera_data.set_viewport_offset(offset);
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

    pub fn init_chunk_area(&mut self, texture_manager: &mut TextureManager, world: Arc<RwLock<World>>, init_area: [i32; 2]) {
        let casted_chunk_manager = &mut self.casted_chunk_manager;
        let arc_camera_data = self.camera_data.clone().get_arc_ref();

        // loop through area and raycast chunks
        for x in 0..init_area[0] {
            for y in 0..init_area[1] {
                // Initialize chunks here
                let relative_chunk_cords = [x, y];
                if let Some(_chunk) = casted_chunk_manager.get_chunk_at_chunk_cords(relative_chunk_cords) {
                    match _chunk.try_read() {
                        Ok(guard) => {
                            // Chunk is not locked, safe to render
                            if !casted_chunk_manager.get_chunk_ray_casted_status(relative_chunk_cords) {
                                casted_chunk_manager.set_chunk_ray_casted_status(relative_chunk_cords, true);
    
                                // submit raycast task
                                self.thread_manager.submit_task(_chunk.clone(), arc_camera_data.clone(), world.clone());
                            }
                            else {
                                guard.render_chunk(&self.camera_data, texture_manager);
                                
                            }
                        },
                        Err(_) => {
                            // Chunk is locked, skip rendering this frame

                        }
                    }
                }
                else {   
                    // Create the missing chunk
                    casted_chunk_manager.create_chunk_at_cords(&self.camera_data, relative_chunk_cords);
                    if let Some(_chunk) = casted_chunk_manager.get_chunk_at_chunk_cords(relative_chunk_cords) {
                        self.render_cache_manager.as_mut().unwrap().add_chunk_to_canvas(texture_manager, _chunk.clone());
                    }
                }
            }
        }



    }

    pub fn render_chunks_around_camera(&mut self, texture_manager: &mut TextureManager, world: Arc<RwLock<World>>, camera_data: &CameraData, arc_camera_data: Arc<CameraData>) {
        let iso_sceen_center = [
            camera_data.get_iso_cam_center()[0] as i32,
            camera_data.get_iso_cam_center()[1] as i32,
        ];

        let iso_chunk_center = CastedChunkManager::get_chunk_cords_from_tile_cords(iso_sceen_center);
        let mut casted_chunk_manager = &mut self.casted_chunk_manager;
        let view_distance = camera_data.get_view_distance() as i32;
        
        for x_rel_cor in -view_distance..view_distance {
            for y_rel_cor in -view_distance..view_distance {
                let relative_chunk_cords = [
                    iso_chunk_center[0] + x_rel_cor,
                    iso_chunk_center[1] + y_rel_cor
                ];

                // If chunk exists
                if let Some(_chunk) = casted_chunk_manager.get_chunk_at_chunk_cords(relative_chunk_cords) {
                    match _chunk.try_read() {
                        Ok(guard) => {
                            // Chunk is not locked, safe to render
                            if !casted_chunk_manager.get_chunk_ray_casted_status(relative_chunk_cords) {
                                casted_chunk_manager.set_chunk_ray_casted_status(relative_chunk_cords, true);
    
                                // submit raycast task
                                self.thread_manager.submit_task(_chunk.clone(), arc_camera_data.clone(), world.clone());
                            }
                            else {
                                guard.render_chunk(&camera_data, texture_manager);
                                
                            }
                        },
                        Err(_) => {
                            // Chunk is locked, skip rendering this frame

                        }
                    }
                }
                else {   
                    // Create the missing chunk
                    casted_chunk_manager.create_chunk_at_cords(&self.camera_data, relative_chunk_cords);

                    if let Some(_chunk) = casted_chunk_manager.get_chunk_at_chunk_cords(relative_chunk_cords) {
                        self.render_cache_manager.as_mut().unwrap().add_chunk_to_canvas(texture_manager, _chunk.clone());
                    }

                }
            }
        }
    }

    pub fn render_cashed_chunks_around_camera(&mut self, texture_manager: &mut TextureManager, camera_data: &CameraData, ctx: &mut GlContext) {
        let view_distance = camera_data.get_cashed_view_distance() as i32;

        // Get iso world cords of center of camera
        let iso_sceen_center = [
            camera_data.get_iso_cam_center()[0] as i32,
            camera_data.get_iso_cam_center()[1] as i32,
        ];
        let iso_chunk_center = CastedChunkManager::get_chunk_cords_from_tile_cords(iso_sceen_center);

        // Clone canvas_data in its own scope to ensure the borrow is released
        let canvas_data = {
            self.render_cache_manager.as_ref().unwrap().get_canvas_data().clone()
        };

        let render_cache_manager = self.render_cache_manager.as_mut().unwrap();

        // Render cashed chunks to spritesheet
        render_cache_manager.start_canvas_render_pass(ctx);
        for x_rel_cor in -view_distance..view_distance {
            for y_rel_cor in -view_distance..view_distance {
                let relative_chunk_cords = [
                    iso_chunk_center[0] + x_rel_cor,
                    iso_chunk_center[1] + y_rel_cor
                ];
                
                let canvas_tile = render_cache_manager.get_mut_canvas_tile(relative_chunk_cords);

                if canvas_tile.is_some() {
                    let _tile = canvas_tile.unwrap();

                    // If tile needs to be rendered to sprite sheet
                    if !_tile.is_rendered_to_sprite_sheet() && self.casted_chunk_manager.get_chunk_ray_casted_status(relative_chunk_cords) {
                        
                        let casted_chunk = self.casted_chunk_manager.get_chunk_at_chunk_cords(relative_chunk_cords);
                        


                        if casted_chunk.is_some() {
                            let casted_chunk = casted_chunk.unwrap();
                            _tile.render_chunk_texture_to_canvas(&canvas_data, texture_manager, &casted_chunk);
                        }
                        _tile.set_rendered_to_sprite_sheet(true);
    
                    }

                }

            }
        }
        //texture_manager.get_texture_renderer().add_quad([-1.0, -1.0, 1.0, 1.0], [1.0, 1.0, 0.0, 0.0]);
        render_cache_manager.end_canvas_render_pass(texture_manager, ctx);

        // Fix viewport after changing render targets
        let viewport_rez = camera_data.get_viewport_rez();
        let viewport_offset = camera_data.get_viewport_offset();
        ctx.apply_viewport( viewport_offset[0] as i32, viewport_offset[1] as i32, viewport_rez[0] as i32, viewport_rez[1] as i32);


        // Render from chunks the canvas texture
        texture_manager.get_texture_renderer().set_texture(render_cache_manager.get_canvas().get_texture_id());
        // Render from the canvas texture
        for x_rel_cor in -view_distance..view_distance {
            for y_rel_cor in -view_distance..view_distance {
                let relative_chunk_cords = [
                    iso_chunk_center[0] + x_rel_cor,
                    iso_chunk_center[1] + y_rel_cor
                ];
                
                let canvas_tile = render_cache_manager.get_mut_canvas_tile(relative_chunk_cords);

                if canvas_tile.is_some() {
                    let _tile = canvas_tile.unwrap();
                    let canvas_casted_iso_cords = _tile.get_iso_cords();
                    
                    let scale = camera_data.get_chunk_ndc_scale();
                    let draw_cords = iso_cord_tool::casted_to_ndc_cords(scale, relative_chunk_cords);

                    let draw_offset = camera_data.get_ndc_draw_offset();

                    let final_draw_cords = [
                        draw_cords[0] + draw_offset[0],
                        draw_cords[1] + draw_offset[1]
                ];


                    //println!("NDC Cords: {:?}, Chunk NDC Scale: {}", ndc_cords, chunk_ndc_scale);

                    if _tile.is_rendered_to_sprite_sheet() {
                        _tile.render_tile(texture_manager, final_draw_cords, scale);
                    }

                }

            }
        }


        // Render full screen quad for testing
        
        // Flush renderings and set texture back to atlas
        texture_manager.get_texture_renderer().flush(ctx);
        texture_manager.set_texture_renderer_to_atlas();

    }

    pub fn render_camera(&mut self, texture_manager : &mut TextureManager, world : Arc<RwLock<World>>, ctx: &mut GlContext) {
        self.get_mut_camera_data().update_camera_values();
        self.get_mut_camera_data().increment_frame_number();
        texture_manager.update_expander_cache(self.camera_data.get_render_scale());

        // Create clones of camera data for threading frame
        let camera_data = self.get_mut_camera_data().clone();
        let arc_camera_data = camera_data.clone().get_arc_ref();

        // Render cashed chunks around camera
        self.render_cashed_chunks_around_camera(texture_manager, &camera_data, ctx);

        // Render chunks around camera
        self.render_chunks_around_camera(texture_manager, world.clone(), &camera_data, arc_camera_data);
        texture_manager.get_texture_renderer().flush(ctx); // Flush chunk renderings
    }

    pub fn get_quadrent_of_cords(cords : [i32; 2]) -> usize {
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
