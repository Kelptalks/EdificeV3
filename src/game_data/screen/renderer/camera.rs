use std::{alloc::System, clone, sync::{Arc, RwLock, TryLockError}, time::SystemTime};

use miniquad::{GlContext, RenderingBackend};

use crate::game_data::{TextureManager, World, debuging::debug_data::DebugData, screen::{self, camera_data::{self, CameraData}, iso_cord_tool, render_string, renderer::{camera, casted_block_manager::{self, casted_chunk::{self, CastedChunk}}, render_cache_manager::{self, canvas_data, render_cashe_manager::RenderCacheManager}, thread_manager::raycast_thread_pool::RaycastThreadPool}, text}, types::{BlockTriangle, BlockType, UITextures}};
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
    // Render Updating
    //=====================================

    pub fn ray_cast_tile_at_cords(&mut self, world: &Arc<RwLock<World>>, casted_tile_cords: [i32; 2]) {
        // Re raycast the tile
        self.casted_chunk_manager.ray_cast_tile_at_casted_cords(world, &self.camera_data, casted_tile_cords);

        // Re render the chunk to the canvas cache
        let chunk_cords = CastedChunkManager::tile_cords_to_chunk_cords(casted_tile_cords);
        let cashed_chunk_option = self.render_cache_manager.as_mut().unwrap().get_mut_canvas_tile(chunk_cords);
        if let Some(cashed_chunk) = cashed_chunk_option {
            cashed_chunk.set_rendered_to_sprite_sheet(false);
        }
    }

    pub fn ray_cast_area_at_cords(&mut self, world: &Arc<RwLock<World>>, casted_tile_cords: [i32; 2], range: i32) {
        for x_offset in -3..3 {
            for y_offset in -3..3 {
                let casted_cords_to_rerender = [
                    casted_tile_cords[0] + x_offset,
                    casted_tile_cords[1] + y_offset
                ];
                self.ray_cast_tile_at_cords(world, casted_cords_to_rerender);
            }
        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn init_chunks_in_area(&mut self, 
        texture_manager: &mut TextureManager, 
        world: Arc<RwLock<World>>, 
        camera_data: &CameraData, 
        arc_camera_data: Arc<CameraData>, 
        range: i32
    ) {
        let casted_chunk_manager = &mut self.casted_chunk_manager;
        
        // Center around world center
        let cam_z_offset = camera_data.get_cam_world_cords()[2] as i32;
        let world_center_chunk_shift = (cam_z_offset / CastedChunkManager::get_chunk_tile_scale() as i32) / 2;
        
        for x_rel_cor in -range..range {
            for y_rel_cor in -range..range {
                let relative_chunk_cords = [
                    x_rel_cor + world_center_chunk_shift,
                    y_rel_cor + world_center_chunk_shift
                ];

                // If chunk exists
                if let Some(_chunk) = casted_chunk_manager.get_chunk_at_chunk_cords(relative_chunk_cords) {
                    match _chunk.read() {
                        Ok(guard) => {
                            // Chunk is not locked, safe to render
                            if !guard.is_ray_casted() {
                                // submit raycast task
                                self.thread_manager.submit_task(_chunk.clone(), arc_camera_data.clone(), world.clone());
                            }
                            else {
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
                        self.render_cache_manager.as_mut().unwrap().add_chunk_to_canvas(_chunk.clone());
                        self.thread_manager.submit_task(_chunk.clone(), arc_camera_data.clone(), world.clone());
                        
                    }

                }
            }
        }
        self.thread_manager.wait_for_completion();
    }

    pub fn render_chunks_around_camera(&mut self, texture_manager: &mut TextureManager, world: Arc<RwLock<World>>, camera_data: &CameraData, arc_camera_data: Arc<CameraData>) {
        let iso_sceen_center = [
            camera_data.get_iso_cam_center()[0] as i32,
            camera_data.get_iso_cam_center()[1] as i32,
        ];

        let iso_chunk_center = CastedChunkManager::get_chunk_cords_from_tile_cords(iso_sceen_center);
        let casted_chunk_manager = &mut self.casted_chunk_manager;
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
                            if !guard.is_ray_casted() {
    
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
                        self.render_cache_manager.as_mut().unwrap().add_chunk_to_canvas(_chunk.clone());
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
        let mut chunks_left_to_cache_this_frame = camera_data.get_max_chunk_cache_per_frame();
        render_cache_manager.start_canvas_render_pass(ctx);
        for x_rel_cor in -view_distance..view_distance {
            for y_rel_cor in -view_distance..view_distance {
                if chunks_left_to_cache_this_frame <= 0 {
                    break;
                }
                
                let relative_chunk_cords = [
                    iso_chunk_center[0] + x_rel_cor,
                    iso_chunk_center[1] + y_rel_cor
                ];
                
                let canvas_tile = render_cache_manager.get_mut_canvas_tile(relative_chunk_cords);

                if canvas_tile.is_some() {
                    let _tile = canvas_tile.unwrap();

                    // If tile needs to be rendered to sprite sheet
                    if !_tile.is_rendered_to_sprite_sheet() {
                        
                        let casted_chunk = self.casted_chunk_manager.get_chunk_at_chunk_cords(relative_chunk_cords);
                        


                        if let Some(casted_chunk) = casted_chunk {
                            
                            match casted_chunk.try_read() {
                                Ok(casted_chunk_guard) => {
                                    if casted_chunk_guard.is_ray_casted() {
                                        _tile.render_chunk_texture_to_canvas(&canvas_data, texture_manager, &casted_chunk_guard);
                                        chunks_left_to_cache_this_frame-=1;
                                    }
                                }
                                Err(TryLockError::WouldBlock) => {
                                    
                                }
                                Err(TryLockError::Poisoned(err)) => {

                                }
                            }
                        }
    
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
                        draw_cords[0] + draw_offset[0] + camera_data.get_tile_ndc_scale(),
                        draw_cords[1] + draw_offset[1]
                ];
                    if _tile.is_rendered_to_sprite_sheet() {
                        _tile.render_tile(canvas_data, texture_manager, final_draw_cords, scale);
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
        // Start frame time
        let frame_start_time = SystemTime::now();
        
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::VoidBackground, [-1.0, -1.0, 1.0, 1.0]);
        texture_manager.get_texture_renderer().flush(ctx);

        // Update values
        self.get_mut_camera_data().update_camera_values();
        self.get_mut_camera_data().increment_frame_number();

        // Create clones of camera data for threading frame
        let camera_data = self.get_mut_camera_data().clone();
        let arc_camera_data = camera_data.clone().get_arc_ref();

        // Render cashed chunks around camera
        texture_manager.set_cached_expander(0.0); // Clear expander for cashing renderings
        self.render_cashed_chunks_around_camera(texture_manager, &camera_data, ctx);

        // Render chunks around camera
        texture_manager.update_expander_cache(self.camera_data.get_render_scale()); // Update for main rendering
        self.render_chunks_around_camera(texture_manager, world.clone(), &camera_data, arc_camera_data);
        texture_manager.get_texture_renderer().flush(ctx); // Flush chunk renderings

        // End Frame time
        let system_time_end = SystemTime::now();
        let frame_duration = system_time_end.duration_since(frame_start_time).unwrap();
        let frame_duration_ms = frame_duration.as_millis();
        self.camera_data.set_frame_time(frame_duration_ms as u32);
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


    //=====================================
    // Debugging
    //=====================================

    pub fn collect_debug_data(&self, debug_data: &mut DebugData) { 
        debug_data.set_frame_time(self.camera_data.get_frame_time());
        debug_data.set_frame_count(self.camera_data.get_frame_count());

        self.render_cache_manager.as_ref().unwrap().collect_debug_data(debug_data);
    }
    
}
