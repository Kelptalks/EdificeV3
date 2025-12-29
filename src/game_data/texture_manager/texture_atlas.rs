use std::{collections::HashMap, ops::Index};

use crate::game_data::{log_init, texture_manager::{block_sheet::BlockTextureManager, shader_sheet::ShaderTextureManager, text_sheet::TextTextureManager, ui_sheet::UITextureManager}, types::{BlockShaderType, BlockTriangle, BlockType, CharType, ShaderTriangle, UITextures}};
use image::{ImageBuffer, RgbaImage};
use miniquad::*;

pub struct TextureAtlas {
    // Atlas Texture
    pub texture_id: TextureId,
    pub atlas_dimensions: u32,

    // Tools for managing sections texture atlas
    pub block_texture_manager: BlockTextureManager,
    pre_calculated_block_uvs: Vec<[[f32; 4]; 6]>,

    pub shader_texture_manager: ShaderTextureManager,
    pre_calculated_shader_uvs: Vec<[[f32; 4]; 14]>,

    text_texture_manager: TextTextureManager,
    pre_calculated_font_uvs: HashMap<String, Vec<[f32; 4]>>,

    ui_texture_manager: UITextureManager,
    pre_calculated_ui_uvs: Vec<[f32; 4]>,

}

impl TextureAtlas {
    pub fn new(ctx: &mut GlContext) -> Self {
        log_init("Creating Texture atlas");
        let atlas_dimensions = 8192;

        // Create image for texture atlas
        let mut atlas_image: RgbaImage = ImageBuffer::new(atlas_dimensions, atlas_dimensions);

        // Pass image through atlas section managers
        
        // Block Textures
        let block_texture_manager = BlockTextureManager::new([0.0, 0.0], atlas_dimensions as f32);
        block_texture_manager.splice_textures(&mut atlas_image);
        let pre_calculated_block_uvs = block_texture_manager.create_pre_calculated_block_uvs(atlas_dimensions as f32);
        
        // Shader Textures
        let start_shader_y_cor = block_texture_manager.get_end_cords()[1] + 50.0;
        let shader_texture_manager = ShaderTextureManager::new([0.0, start_shader_y_cor]);
        shader_texture_manager.splice_textures(&mut atlas_image);
        let pre_calculated_shader_uvs = shader_texture_manager.create_pre_calculated_shader_uvs(atlas_dimensions as f32);

        // Text Textures
        let start_text_y_cor = shader_texture_manager.end_cords[1] + 50.0;
        let mut text_texture_manager = TextTextureManager::new([0.0, start_text_y_cor]);
        text_texture_manager.splice_fonts_to_atlas(&mut atlas_image);
        let pre_calculated_font_uvs = text_texture_manager.create_pre_calculated_fonts_uvs(atlas_dimensions as f32);

        // UI Textures
        let start_ui_y_cor = text_texture_manager.get_end_cords()[1] + 50.0;
        let ui_texture_manager = UITextureManager::new([0.0, start_ui_y_cor]);
        ui_texture_manager.splice_textures(&mut atlas_image);
        let pre_calculated_ui_uvs = ui_texture_manager.create_pre_calculated_ui_uvs(atlas_dimensions as f32);


        // Convert image to Texture
        // Create miniquad texture
        let rgba_bytes: Vec<u8> = atlas_image.clone().into_raw();
        let atlas_texture: TextureId = ctx.new_texture_from_rgba8(
            atlas_dimensions as u16,
            atlas_dimensions as u16,
            &rgba_bytes,
        );

        // Set both min and mag filter modes for pixel art
        ctx.texture_set_min_filter(atlas_texture, FilterMode::Nearest, MipmapFilterMode::None);
        ctx.texture_set_mag_filter(atlas_texture, FilterMode::Nearest);

        Self {
            // Atlas data
            texture_id: atlas_texture,
            atlas_dimensions: atlas_dimensions,

            // Atlas section managers
            block_texture_manager: block_texture_manager,
            pre_calculated_block_uvs: pre_calculated_block_uvs,

            shader_texture_manager: shader_texture_manager,
            pre_calculated_shader_uvs: pre_calculated_shader_uvs,

            text_texture_manager: text_texture_manager,
            pre_calculated_font_uvs: pre_calculated_font_uvs,

            ui_texture_manager: ui_texture_manager,
            pre_calculated_ui_uvs: pre_calculated_ui_uvs,
        }
    }

    pub fn get_atlas_texture_id(&self) -> TextureId {
        return self.texture_id;
    }

    pub fn get_precalculated_block_triangle_uv(&self, triangle: BlockTriangle, block: BlockType) -> [f32; 4] {
        return self.pre_calculated_block_uvs[block.id_as_usize()][triangle.id_as_usize()];
    }

    pub fn get_precalculated_shader_triangle_uv(&self, triangle: ShaderTriangle, shader: BlockShaderType) -> [f32; 4] {
        return self.pre_calculated_shader_uvs[shader.id_as_usize()][triangle.id_as_usize()];
    }

    pub fn get_precalculated_ui_uv(&self, ui_texture: UITextures) -> [f32; 4] {
        return self.pre_calculated_ui_uvs[ui_texture.get_id() as usize];
    }

    pub fn get_precalculated_font_uv(&self, font: String, char: CharType) -> [f32; 4] {
        let char_uvs = self.pre_calculated_font_uvs.get(&font);

        if let Some(char_uvs) = char_uvs {
            return char_uvs[char.get_id() as usize];

        } else {
            println!("Failed to index font");
            return [0.0, 0.0, 0.0, 0.0];
        }
    }
}
