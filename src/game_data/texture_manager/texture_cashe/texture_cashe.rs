use std::sync::atomic::{AtomicU32, Ordering};

use miniquad::{GlContext, RenderingBackend, TextureId};
use rodio::cpal::InputStreamTimestamp;

use crate::game_data::{TextureManager, screen::widget::widget_calculations, texture_manager::rendering_managager::rendering_batch::RenderBatch};

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
    

    textures_init: bool,
    atlas: Vec<TextureId>,


    cashing_batches: Vec<RenderBatch>,
    drawing_batches: Vec<RenderBatch>,



    atlas_count: usize,
    scale: usize,    
}

impl TextureCashe {
    pub fn new() -> TextureCashe {
        let atlas_count = 8;
        let scale = 8;
        

        
        TextureCashe {
            free_id: Vec::new(),
            
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

        let render_batch = self.get_batch_cashing_batch(cashed_texture);
        if let Some(render_batch) = render_batch {

        

            let scaled_pos = [
                pos[0] * cell_size - cashe_pos[0],
                pos[1] * cell_size - cashe_pos[1],
                pos[2] * cell_size - cashe_pos[0],
                pos[3] * cell_size - cashe_pos[1],
            ];

            render_batch.add_quad(scaled_pos, uv);
        }
        else {
            eprintln!("Missing render batch for texture cashe id({})", cashed_texture.id)
        }
        
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
            let width = 8192u16;
            let height = 8192u16;
            let bytes = vec![10u8; (width as usize * height as usize * 4) as usize];
            let new_texture = ctx.new_texture_from_rgba8(width, height, &bytes);

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


    pub fn get_cashing_batches(&mut self, ctx: &mut GlContext) -> Vec<RenderBatch> {
        if !self.textures_init {
            self.init(ctx);
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
