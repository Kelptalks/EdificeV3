use std::sync::{Arc, RwLock};

use miniquad::{GlContext, RenderingBackend, TextureId, TextureParams};

use crate::game_data::{TextureManager, screen::{camera_data, iso_cord_tool, renderer::{casted_block_manager::casted_chunk::CastedChunk, render_cache_manager::canvas_data::{self, CanvasData}}, text}, types::BlockTriangle};

pub struct CanvasChunk {
    pub iso_cords: [i32; 2],
    pub canvas_id: u32,

    // Canvas data
    pub canvas_ndc_cords: [f32; 2],
    pub tile_uv: [f32; 4],

    rendered_to_sprite_sheet: bool,
}

impl CanvasChunk {
    pub fn new(iso_cords: [i32; 2], canvas_id: u32, canvas_data: CanvasData) -> CanvasChunk {
        let canvas_ndc_cords = canvas_data.id_to_canvas_render_offset(canvas_id);
        let tile_uv = canvas_data.id_to_uv_cords(canvas_id);

        //println!("canvas_id: {}", (canvas_id % canvas_data.tiles_per_collumn) as f32);
        //println!("tile_uv: {}, {}, {}, {}", tile_uv[0], tile_uv[1], tile_uv[2], tile_uv[3]);
        

        CanvasChunk{ 
            // Identification
            iso_cords,
            canvas_id,
            
            // Canvas properties
            canvas_ndc_cords,
            rendered_to_sprite_sheet: false,
            tile_uv: tile_uv,
        }
    }

    pub fn render_chunk_texture_to_canvas(&self, canvas_data: &CanvasData, texture_manager: &mut TextureManager, casted_chunk: &Arc<RwLock<CastedChunk>>) {
        let unpacked_chunk = casted_chunk.write().unwrap();
        let casted_tile_ndc_scale = canvas_data.casted_tile_ndc_scale;

        // Get render offset coordinates (includes X centering offset)
        let render_offset = self.canvas_ndc_cords;
        let canvas_tile_x_offset = render_offset[0] - casted_tile_ndc_scale;
        let canvas_tile_y_offset = render_offset[1];
        
        for index in 0..CastedChunk::get_chunk_tile_area() {
            let casted_tile = unpacked_chunk.get_tile_at_index(index as usize);

            // Calculate draw cords
            let x_iso_cor = index as i32 % CastedChunk::get_chunk_tile_dimensions() as i32;
            let y_iso_cor = index as i32 / CastedChunk::get_chunk_tile_dimensions() as i32;
            let mut ndc_cords = iso_cord_tool::float_iso_to_ndc_cords(
                casted_tile_ndc_scale,[x_iso_cor as f32, y_iso_cor as f32]
            );

            
            ndc_cords[0] += canvas_tile_x_offset;
            ndc_cords[1] += canvas_tile_y_offset;

            let triangles = casted_tile.get_triangles();
            triangles[0].render_casted_triangle(texture_manager, ndc_cords, casted_tile_ndc_scale);
            ndc_cords[0] += casted_tile_ndc_scale; // Offset for right triangle
            triangles[1].render_casted_triangle(texture_manager, ndc_cords, casted_tile_ndc_scale);
        }

        /*
        texture_manager.render_block_triangle(crate::game_data::types::BlockType::Core,
            BlockTriangle::TopLeft,
             [canvas_tile_x_offset, canvas_tile_y_offset],
            canvas_data.tile_ndc_scale[0]
        );
         */
    }
    
    pub fn render_tile(&self, canvas_data: CanvasData, texture_manager: &mut TextureManager, ndc_cords: [f32; 2], scale: f32) {
        let expanded_scale = scale * canvas_data.expander;

        let pos = [
            ndc_cords[0] - scale,
            ndc_cords[1],
            ndc_cords[0] + expanded_scale,
            ndc_cords[1] + expanded_scale
        ];
        
        texture_manager.get_texture_renderer().add_quad(
            pos,
            self.tile_uv
        );
    }

    pub fn is_rendered_to_sprite_sheet(&self) -> bool {
        return self.rendered_to_sprite_sheet;
    }

    pub fn set_rendered_to_sprite_sheet(&mut self, rendered: bool) {
        self.rendered_to_sprite_sheet = rendered;
    }

    pub fn get_iso_cords(&self) -> [i32; 2] {
        return self.iso_cords;
    }

}
