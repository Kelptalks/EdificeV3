use crate::game_data::{log_init, Types::{BlockTriangle, BlockType, BlockShaderType, ShaderTriangle}, texture_manager::{block_sheet::BlockTextureManager, shader_sheet::ShaderTextureManager}};
use image::{ImageBuffer, RgbaImage};
use miniquad::*;

pub struct TextureAtlas {
    // Atlas Texture
    pub texture_id: TextureId,
    atlas_dimensions: u32,

    // Tools for managing sections texture atlas
    block_texture_manager: BlockTextureManager,
    pre_calculated_block_uvs: Vec<[[f32; 4]; 6]>,

    shader_texture_manager: ShaderTextureManager,
    pre_calculated_shader_uvs: Vec<[[f32; 4]; 14]>,
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
        let shader_texture_manager = ShaderTextureManager::new([0.0, 500.0]);
        shader_texture_manager.splice_textures(&mut atlas_image);
        let pre_calculated_shader_uvs = shader_texture_manager.create_pre_calculated_shader_uvs(atlas_dimensions as f32);

        println!("Shader Texture sheet cords ({}, {})", shader_texture_manager.start_cords[0], shader_texture_manager.start_cords[1]);
        
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
        }
    }

    pub fn get_precalculated_block_triangle_uv(&self, triangle: BlockTriangle, block: BlockType) -> [f32; 4] {
        return self.pre_calculated_block_uvs[block.id_as_usize()][triangle.id_as_usize()];
    }

    pub fn get_precalculated_shader_triangle_uv(&self, triangle: ShaderTriangle, shader: BlockShaderType) -> [f32; 4] {
        return self.pre_calculated_shader_uvs[shader.id_as_usize()][triangle.id_as_usize()];
    }
}
