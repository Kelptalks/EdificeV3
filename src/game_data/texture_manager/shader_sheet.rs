use crate::game_data::types::{BlockShaderType, ShaderTriangle};
use image::{ImageBuffer, Rgba, RgbaImage};

static BLOCK_PIXLE_REZ: u32 = 64;
static BLOCK_TEXTURES_PER_ROW: u32 = 20;

pub struct ShaderTextureManager {
    pub start_cords: [f32; 2],
    pub end_cords: [f32; 2],

    // Texture alignment  
    buffer_space: f32,
    total_shaders: u32,
    triangles_per_shader: u32,
    sprite_pixel_scale: [f32; 2],
}

impl ShaderTextureManager {
    pub fn new(start_cords: [f32; 2]) -> Self {
        // Texture alignment  
        let buffer_space = 8.0;
        let total_shaders = 4.0 as f32;
        let triangles_per_shader = 14.0;
        let sprite_pixel_scale = [32.0, 32.0];

        // Calculate end cords
        let end_cords = [
            start_cords[0] + ((buffer_space + sprite_pixel_scale[0]) * total_shaders),
            start_cords[1] + ((buffer_space + sprite_pixel_scale[1]) * triangles_per_shader)
        ];

        Self { 
            start_cords: start_cords,
            end_cords: end_cords,
            buffer_space: buffer_space,
            total_shaders: total_shaders as u32,
            triangles_per_shader: triangles_per_shader as u32,
            sprite_pixel_scale: sprite_pixel_scale
        }
    }

    pub fn splice_textures(&self, atlas_image: &mut RgbaImage) {
        // Get the block sprite sheet and masking block
        let block_sprite_image = image::open("Assets/Shaders.png").unwrap().to_rgba8();
        let block_masks_image = image::open("Assets/masking_textures.png").unwrap().to_rgba8();

        println!("Block mask width : {}", block_masks_image.width());


        // Loop through all blocks and splice there textures
        // Loop through all block locations
        for shader_id in 0..self.total_shaders {
            let block_src_x_cor = BLOCK_PIXLE_REZ * (shader_id % BLOCK_TEXTURES_PER_ROW);
            let block_src_y_cor = BLOCK_PIXLE_REZ * (shader_id / BLOCK_TEXTURES_PER_ROW);

            let x_dest_cor = ((self.sprite_pixel_scale[0] + self.buffer_space) * shader_id as f32) as u32;
            let triangle_spacing = self.buffer_space + self.sprite_pixel_scale[1];

            // First masking block
            for y in 0..BLOCK_PIXLE_REZ {
                for x in 0..BLOCK_PIXLE_REZ {
                    // Get masking textures pixle color
                    let source_pixel = block_masks_image.get_pixel(x, y);
                    let [r, g, b, a] = source_pixel.0;  // Gets [u8; 4] array

                    let x_draw_cor = x + x_dest_cor + self.start_cords[0] as u32;
                    let y_draw_cor = self.start_cords[1] as u32;

                    //Top Left
                    if r == 243 && g == 255 && b == 0 {
                        let y_mod = (0.0 + ShaderTriangle::TopLeft.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Top Right
                    else if r == 7 && g == 255 && b == 0 {
                        let y_mod = (0.0 + ShaderTriangle::TopRight.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor - 32, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Left Top
                    else if r == 255 && g == 0 && b == 0 {
                        let y_mod = (-16.0 + ShaderTriangle::LeftTop.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Left Bot
                    else if r == 253 && g == 0 && b == 232 {
                        let y_mod = (-32.0 + ShaderTriangle::LeftBot.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Right Top
                    else if r == 0 && g == 193 && b == 255 {
                        let y_mod = (-16.0 + ShaderTriangle::RightTop.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor - 32, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Right Bot
                    else if r == 110 && g == 0 && b == 255 {
                        let y_mod = (-32.0 + ShaderTriangle::RightBot.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor - 32, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                }
            }


            
            for y in 0..BLOCK_PIXLE_REZ {
                for x in 0..BLOCK_PIXLE_REZ {
                    // Get masking textures pixle color
                    let source_pixel = block_masks_image.get_pixel(x + 64, y);
                    let [r, g, b, a] = source_pixel.0;  // Gets [u8; 4] array

                    let x_draw_cor = x + x_dest_cor + self.start_cords[0] as u32;
                    let y_draw_cor = self.start_cords[1] as u32;

                    //TOPTOPLEFT
                    if r == 255 && g == 0 && b == 0 {
                        let y_mod = (0.0 + ShaderTriangle::TopTopLeft.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //TOPBOTLEFT
                    else if r == 197 && g == 255 && b == 0 {
                        let y_mod = (0.0 + ShaderTriangle::TopBotLeft.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //TOPTOPRIGHT
                    else if r == 208 && g == 133 && b == 255 {
                        let y_mod = (0.0 + ShaderTriangle::TopTopRight.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor - 32, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //TOPBOTRIGHT
                    else if r == 0 && g == 255 && b == 65 {
                        let y_mod = (0.0 + ShaderTriangle::TopBotRight.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor - 32, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //LEFTCENTERLEFT
                    else if r == 7 && g == 255 && b == 0 {
                        let y_mod = (-16.0 + ShaderTriangle::LeftCenterLeft.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //LEFTCENTERTOP
                    else if r == 130 && g == 0 && b == 255 {
                        let y_mod = (-16.0 + ShaderTriangle::LeftCenterTop.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //LEFTCENTERRIGHT
                    else if r == 253 && g == 0 && b == 232 {
                        let y_mod = (-32.0 + ShaderTriangle::LeftCenterRight.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //LEFTCENTERBOT
                    else if r == 0 && g == 193 && b == 255 {
                        let y_mod = (-32.0 + ShaderTriangle::LeftCenterBot.get_shader_triangle_id() * triangle_spacing) as u32 + y_draw_cor;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    
                }
            }
        }
    }

    pub fn get_shader_triangle_src_rect(&self, triangle: ShaderTriangle, shader_type: BlockShaderType) -> [f32; 4] {
        let x_start_cor = self.start_cords[0] + ((self.buffer_space + self.sprite_pixel_scale[0]) * shader_type.id() as f32);
        
        let triangle_spacing = self.buffer_space + self.sprite_pixel_scale[1];
        let triangle_id = triangle.id();
        
        
        let y_start_cor = self.start_cords[1] + (triangle_id as f32 * triangle_spacing);

        let x_end_cor = x_start_cor + self.sprite_pixel_scale[0];
        let y_end_cor = y_start_cor + self.sprite_pixel_scale[1];

        return [x_start_cor, y_start_cor, x_end_cor, y_end_cor];
    }

    pub fn get_shader_triangle_uv(&self, triangle: ShaderTriangle, shader_type: BlockShaderType, atlas_dimensions: f32) -> [f32; 4] {
        let src_rect = self.get_shader_triangle_src_rect(triangle, shader_type);
        let mut uv = [0.0, 0.0, 0.0, 0.0];

        // Create uv
        uv[0] = src_rect[0] / atlas_dimensions;
        uv[1] = src_rect[1] / atlas_dimensions;
        uv[2] = src_rect[2] / atlas_dimensions;
        uv[3] = src_rect[3] / atlas_dimensions;

        return uv;
    }

    pub fn create_pre_calculated_shader_uvs(&self, atlas_dimensions: f32) -> Vec<[[f32; 4]; 14]> {
        let mut shaders: Vec<[[f32; 4]; 14]> = Vec::new();

        for current_block_shader in 0..self.total_shaders {
            // Create triangle uv array from block
            let mut block_triangles = [[0.0; 4]; 14];
            for current_triangle in 0..self.triangles_per_shader {
                let uv = self.get_shader_triangle_uv(
                    ShaderTriangle::from_id(current_triangle as u16),
                    BlockShaderType::from_id(current_block_shader as u16),
                    atlas_dimensions
                );
                block_triangles[current_triangle as usize] = uv;
            }

            // Add triangle array uv to block uv vector
            shaders.push(block_triangles);
        }

        return shaders;
    }
}
