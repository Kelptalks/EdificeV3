use miniquad::ShaderType;

use crate::game_data::{texture_manager, TextureManager, types::{BlockShaderType, BlockTriangle, BlockTexture, ShaderTriangle}};

#[derive(Clone)]
pub struct CastedTriangle {
    // Textures for the casted triangle
    block_types : Vec<BlockTexture>, 
    triangle_types : Vec<BlockTriangle>,

    shader_triangle : ShaderTriangle,
    shader_type: BlockShaderType,

    // Where the casted ray struck
    translucent_block_struck_cor : [i32; 3],
    translucent_struck : bool,

    solid_block_struck : [i32; 3],
}

impl CastedTriangle {
    pub fn new() -> Self {
        Self {
            block_types : Vec::new(),
            triangle_types : Vec::new(),

            shader_triangle : ShaderTriangle::LeftTop,
            shader_type: BlockShaderType::None,


            translucent_block_struck_cor : [0, 0, 0],
            translucent_struck : false,

            solid_block_struck : [0, 0, 0],
        }
    }

    pub fn clear_triangle(&mut self) {
        self.block_types.clear();
        self.triangle_types.clear();
        self.translucent_struck = false;
        self.shader_type = BlockShaderType::None;
    }

    pub fn add_texture(&mut self, block : BlockTexture, triangle : BlockTriangle) {
        self.block_types.insert(0, block);
        self.triangle_types.insert(0, triangle);
    }

    pub fn has_struck_translucent(&self) -> bool {
        return self.translucent_struck;
    }
    
    pub fn struck_translucent(&mut self, cords : [i32; 3]) {
        self.translucent_block_struck_cor = cords;
        self.translucent_struck = true;
    }
    
    pub fn struck_solid(&mut self, cords : [i32; 3]) {
        self.solid_block_struck = cords;
    }

    pub fn set_shader(&mut self, shader_triangle : ShaderTriangle, shader_type: BlockShaderType,) {
        self.shader_triangle = shader_triangle;
        self.shader_type = shader_type;
    }

    pub fn get_solid_struck_cords(&self) -> [i32; 3] {
        return self.solid_block_struck;
    }

    pub fn get_last_texture(&self) -> BlockTriangle {
        if self.triangle_types.len() > 0 {
            return self.triangle_types[0];
        }
        return BlockTriangle::TopLeft; // Default fallback
    }

    pub fn get_shader_triangle(&self) -> ShaderTriangle {
        return self.shader_triangle;
    }

    pub fn render_casted_triangle(&self, texture_manager : &mut TextureManager, draw_cords : [f32; 2], scale : f32) {
        for i in 0..self.block_types.len() {
            texture_manager.render_block_triangle(
                self.block_types[i],
                self.triangle_types[i],
                draw_cords,
                scale
            );
        }
        if self.shader_type.id() != 0 {
            texture_manager.render_shader_triangle(self.shader_type, self.shader_triangle, draw_cords, scale);
        }
    }
}
