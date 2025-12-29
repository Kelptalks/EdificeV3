use std::sync::{Arc, RwLock};

use miniquad::{GlContext, PassAction, RenderPass, RenderingBackend};

use crate::game_data::{TextureManager, screen::renderer::{casted_block_manager::casted_chunk::CastedChunk, render_cache_manager::{canvas::Canvas, canvas_data::CanvasData, canvas_chunk::CanvasChunk}}, texture_manager};

pub struct RenderCacheManager {
    canvas: Canvas,
    render_pass: RenderPass,
}

impl RenderCacheManager {
    pub fn new(ctx: &mut GlContext) -> Self {
        let canvas = Canvas::new(ctx);
        let canvas_texture_id = canvas.get_texture_id();
        Self {
            canvas: canvas,
            render_pass: ctx.new_render_pass(canvas_texture_id, None),
        }
    }

    pub fn add_chunk_to_canvas(&mut self, texture_manager: &mut TextureManager, casted_chunk: Arc<RwLock<CastedChunk>>) {
        let iso_cords = casted_chunk.read().unwrap().get_chunk_cords();

        self.canvas.create_tile_at(iso_cords);
        let _tile = self.canvas.get_tile_at(iso_cords);

        
    }

    pub fn get_canvas_tile(&mut self, iso_cords: [i32; 2]) -> Option<&CanvasChunk> {
        return self.canvas.get_tile_at(iso_cords);
    }

    pub fn get_mut_canvas_tile(&mut self, iso_cords: [i32; 2]) -> Option<&mut CanvasChunk> {
        return self.canvas.get_mut_tile_at(iso_cords);
    }

    pub fn get_canvas_data (&self) -> &CanvasData {
        return self.canvas.get_canvas_data();
    }

    pub fn get_canvas(&self) -> &Canvas {
        return &self.canvas;
    }

    pub fn start_canvas_render_pass(&mut self, ctx: &mut GlContext) {
        ctx.begin_pass(Some(self.render_pass), PassAction::Nothing);    
    }

    pub fn end_canvas_render_pass(&mut self, texture_manager: &mut TextureManager, ctx: &mut GlContext) {
        texture_manager.get_texture_renderer().flush(ctx);
        ctx.end_render_pass();
        ctx.begin_default_pass(PassAction::Nothing);
    }

    pub fn test(&mut self, texture_manager: &mut TextureManager, ctx: &mut GlContext) {
        self.start_canvas_render_pass(ctx);
        
        // Render to the canvas texture
        //self.canvas.render_chunk_tiles(texture_manager, ctx);
        
        self.end_canvas_render_pass(texture_manager, ctx);
        


        // Setting the spritesheet of the texture renderer to the canvas texture
        texture_manager.get_texture_renderer().set_texture(self.canvas.get_texture_id());

        // Render from the canvas texture
        texture_manager.get_texture_renderer().add_quad([-1.0, -1.0, 1.0, 1.0], [1.0, 1.0, 0.0, 0.0]);
        
        // Flush renderings and set texture back to atlas
        texture_manager.get_texture_renderer().flush(ctx);
        texture_manager.set_texture_renderer_to_atlas();
            
    }


}
