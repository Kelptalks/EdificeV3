#![allow(dead_code)]
use std::sync::atomic::{AtomicU32, Ordering};

use miniquad::{FilterMode, GlContext, MipmapFilterMode, RenderingBackend, TextureId};

const ATLAS_SIZE: u32 = 8192;

use crate::game_data::texture_manager::rendering_managager::rendering_batch::RenderBatch;

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq)]
pub struct CashedTextureID {
    id: u32,

    src_texture_index: usize,
    src_pos: [f32; 4],
    src_uv: [f32; 4],

}

impl CashedTextureID {
    fn get_next_id(src_texture_index: usize, src_pos: [f32; 4], src_uv: [f32; 4]) -> CashedTextureID {
        CashedTextureID {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            
            src_texture_index,
            
            src_pos,
            src_uv,
        }
    }
}


pub struct TextureCashe {
    free_id: Vec<CashedTextureID>,
    pending_clears: Vec<CashedTextureID>,
    pending_frees: Vec<CashedTextureID>,

    textures_init: bool,
    atlas: Vec<TextureId>,


    cashing_batches: Vec<RenderBatch>,
    drawing_batches: Vec<RenderBatch>,



    atlas_count: usize,
    scale: usize,    
}

impl TextureCashe {
    pub fn new() -> TextureCashe {
        let atlas_count: usize = 8;
        let scale = 8;

        
        TextureCashe {
            free_id: Vec::new(),
            pending_clears: Vec::new(),
            pending_frees: Vec::new(),

            textures_init: false,
            atlas: Vec::new(),


            cashing_batches: Vec::new(),
            drawing_batches: Vec::new(),

            atlas_count,
            scale,
        }
    }


    pub fn total_free_textures(&self) -> usize {
        self.free_id.len()
    }
    

    fn get_batch_cashing_batch(&mut self, cashed_texture: CashedTextureID) -> Option<&mut RenderBatch> {
        self.cashing_batches.get_mut(cashed_texture.src_texture_index)
    }

    fn get_drawing_batch(&mut self, cashed_texture: CashedTextureID) -> Option<&mut RenderBatch> {
        self.drawing_batches.get_mut(cashed_texture.src_texture_index)
    }

    pub fn render_to_cashed_texture(
        &mut self,
        cashed_texture: CashedTextureID,
        uv: [f32; 4],
        pos: [f32; 4],
    ) {
        let cell_size = 1.0 / self.scale as f32;
        let cashe_pos = [
            cashed_texture.src_pos[0] + cell_size,
            cashed_texture.src_pos[1] + cell_size,
        ];

        // Map from negated tile-map-local NDC → atlas NDC
        let atlas_pos = [
            cashe_pos[0] - pos[0] * cell_size,
            cashe_pos[1] - pos[1] * cell_size,
            cashe_pos[0] - pos[2] * cell_size,
            cashe_pos[1] - pos[3] * cell_size,
        ];

        // Clip to cell bounds to prevent bleeding into adjacent cells
        if let Some((clipped_uv, clipped_atlas)) = Self::clip_to_bounds(uv, atlas_pos, cashed_texture.src_pos) {
            // add_quad negates Y, so pass -atlas_y to get the correct GPU vertex Y
            let quad_pos = [clipped_atlas[0], -clipped_atlas[1], clipped_atlas[2], -clipped_atlas[3]];
            if let Some(render_batch) = self.get_batch_cashing_batch(cashed_texture) {
                render_batch.add_quad(quad_pos, clipped_uv);
            } else {
                eprintln!("Missing render batch for texture cashe id({})", cashed_texture.id);
            }
        }
    }

    fn clip_to_bounds(uv: [f32; 4], draw_pos: [f32; 4], bounds: [f32; 4]) -> Option<([f32; 4], [f32; 4])> {
        let [dx1, dy1, dx2, dy2] = draw_pos;
        let [bx1, by1, bx2, by2] = bounds;
        let cx1 = dx1.max(bx1);
        let cy1 = dy1.max(by1);
        let cx2 = dx2.min(bx2);
        let cy2 = dy2.min(by2);
        if cx1 >= cx2 || cy1 >= cy2 { return None; }
        let dw = dx2 - dx1;
        let dh = dy2 - dy1;
        let clip_l = (cx1 - dx1) / dw;
        let clip_t = (cy1 - dy1) / dh;
        let clip_r = (dx2 - cx2) / dw;
        let clip_b = (dy2 - cy2) / dh;
        let [u, v, u2, v2] = uv;
        let uw = u2 - u;
        let uh = v2 - v;
        Some((
            [u + clip_l * uw, v + clip_t * uh, u2 - clip_r * uw, v2 - clip_b * uh],
            [cx1, cy1, cx2, cy2],
        ))
    }

    pub fn render_cashed_texture(
        &mut self, 
        cashed_texture: CashedTextureID,
        pos: [f32; 4],
    ) {
        let src_texture_id = self.atlas.get(cashed_texture.src_texture_index);
        if let Some(src_texture_id) = src_texture_id {
            
            let mut render_batch = RenderBatch::new(Some(*src_texture_id), None); 
            
            let uv = cashed_texture.src_uv;
            render_batch.add_quad(pos, uv);

            self.drawing_batches.push(render_batch);
        }
        else {
            eprintln!("Missing draw batch fo texture cashe id({})", cashed_texture.id)
        }
    }


    fn init(&mut self, ctx: &mut GlContext) {
        println!("Init Texture Cashe");
        for i in 0..self.atlas_count {
            
            // Create the textures
            let new_texture = ctx.new_texture_from_rgba8(1, 1, &[0, 0, 0, 0]);
            ctx.texture_resize(new_texture, 8192, 8192, None);
            ctx.texture_set_min_filter(new_texture, FilterMode::Nearest, MipmapFilterMode::None);
            ctx.texture_set_mag_filter(new_texture, FilterMode::Nearest);



            self.atlas.push(new_texture);
            
            let batch: RenderBatch = RenderBatch::new(None, Some(new_texture));
            self.cashing_batches.push(batch);
            
            
            let uv_scale = 2.0 / self.scale as f32; // back to 2.0 — this is NDC space

            for x in 0..self.scale {
                for y in 0..self.scale {
                    let src_pos = [
                        uv_scale * x as f32 - 1.0,
                        uv_scale * y as f32 - 1.0,
                        uv_scale * (x as f32 + 1.0) - 1.0,
                        uv_scale * (y as f32 + 1.0) - 1.0,
                    ];

                    let src_uv = [
                        src_pos[0] * 0.5 + 0.5,
                        src_pos[1] * 0.5 + 0.5,
                        src_pos[2] * 0.5 + 0.5,
                        src_pos[3] * 0.5 + 0.5,
                    ];

                    self.free_id.push(
                        CashedTextureID::get_next_id(
                            i, 
                            src_pos, 
                            src_uv
                        )
                    );
                }
            }

        }
        self.textures_init = true;
        println!("total_texture_cashe_slots: {}", self.free_id.len())
    }

    pub fn get_free_cashed_texture(&mut self) -> Option<CashedTextureID> {
        self.free_id.pop()
    }

    pub fn free_cashed_texture(&mut self, id: CashedTextureID) {
        self.pending_frees.push(id);
    }

    pub fn clear_cashed_texture(&mut self, id: CashedTextureID) {
        self.pending_clears.push(id);
    }


    pub fn get_cashing_batches(&mut self, ctx: &mut GlContext) -> Vec<RenderBatch> {
        if !self.textures_init {
            self.init(ctx);
        }

        let pending = std::mem::take(&mut self.pending_frees);
        for id in pending {
            if let Some(&atlas_texture) = self.atlas.get(id.src_texture_index) {
                let x = (id.src_uv[0] * ATLAS_SIZE as f32).round() as i32;
                let y = (id.src_uv[1] * ATLAS_SIZE as f32).round() as i32;
                let w = ((id.src_uv[2] - id.src_uv[0]) * ATLAS_SIZE as f32).round() as i32;
                let h = ((id.src_uv[3] - id.src_uv[1]) * ATLAS_SIZE as f32).round() as i32;
                let zeros = vec![0u8; (w * h * 4) as usize];
                ctx.texture_update_part(atlas_texture, x, y, w, h, &zeros);
            }
            self.free_id.push(id);
        }

        let pending = std::mem::take(&mut self.pending_clears);
        for id in pending {
            if let Some(&atlas_texture) = self.atlas.get(id.src_texture_index) {
                let x = (id.src_uv[0] * ATLAS_SIZE as f32).round() as i32;
                let y = (id.src_uv[1] * ATLAS_SIZE as f32).round() as i32;
                let w = ((id.src_uv[2] - id.src_uv[0]) * ATLAS_SIZE as f32).round() as i32;
                let h = ((id.src_uv[3] - id.src_uv[1]) * ATLAS_SIZE as f32).round() as i32;
                let zeros = vec![0u8; (w * h * 4) as usize];
                ctx.texture_update_part(atlas_texture, x, y, w, h, &zeros);
            }
        }




        let batches = self.cashing_batches.clone();

        for batch in &mut self.cashing_batches {
            batch.clear();
        }

        batches
    }

    pub fn get_drawing_batches(&mut self) -> Vec<RenderBatch> {
        let batches = self.drawing_batches.clone();
        self.drawing_batches.clear();

        batches
    }
}
