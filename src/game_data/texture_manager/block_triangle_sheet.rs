use crate::game_data::types::{BlockTriangle, BlockTexture};
use image::{ImageBuffer, Rgba, RgbaImage};

static BLOCK_PIXLE_REZ: u32 = 64;
static BLOCK_TEXTURES_PER_ROW: u32 = 20;

/*
#################
## Block Sheet ##
#################
This file is responsable for managing the splicing of the block textures. This
file uses masking textures to create triangles from each block texture and splices
them to the atlas.


Go to "game_data/Types/Triangles" for more information on what bock triangles are


*/

pub struct BlockTriangleTextureManager {
    start_cords: [f32; 2],
    end_cords: [f32; 2],

    // Texture alignment
    buffer_space: f32,
    total_blocks: u32,
    triangles_per_block: u32,
    sprite_pixel_scale: [f32; 2],
    blocks_per_row: u32,
    row_height: f32,
}

impl BlockTriangleTextureManager {
    pub fn new(start_cords: [f32; 2], atlas_dimensions: f32) -> Self {
        // Texture alignment
        let buffer_space = 8.0;
        let total_blocks = BlockTexture::get_total_blocks() as f32;
        let triangles_per_block = 6.0;
        let sprite_pixel_scale = [32.0, 32.0];

        let blocks_per_row = ((atlas_dimensions - start_cords[0]) / (buffer_space + sprite_pixel_scale[0])).floor() as u32;
        let row_height = triangles_per_block * (sprite_pixel_scale[1] + buffer_space);
        let total_rows = ((total_blocks as u32) + blocks_per_row - 1) / blocks_per_row;

        // Calculate end cords
        let end_cords = [
            start_cords[0] + blocks_per_row as f32 * (buffer_space + sprite_pixel_scale[0]),
            start_cords[1] + total_rows as f32 * row_height,
        ];

        Self {
            start_cords,
            end_cords,
            buffer_space,
            total_blocks: total_blocks as u32,
            triangles_per_block: triangles_per_block as u32,
            sprite_pixel_scale,
            blocks_per_row,
            row_height,
        }
    }

    // Splice the textures from block sheet to sprite sheets location
    pub fn splice_textures(&self, atlas_image: &mut RgbaImage) {
        // Get the block sprite sheet and masking block
        let block_sprite_image = image::open("Assets/Blocks.png").unwrap().to_rgba8();
        let block_masks_image = image::open("Assets/masking_textures.png").unwrap().to_rgba8();

        // Loop through all blocks and splice there textures
        for block_id in 0..BlockTexture::get_total_blocks() {
            let block_src_x_cor = BLOCK_PIXLE_REZ * (block_id % BLOCK_TEXTURES_PER_ROW);
            let block_src_y_cor = BLOCK_PIXLE_REZ * (block_id / BLOCK_TEXTURES_PER_ROW);

            let block_col = block_id % self.blocks_per_row;
            let block_row = block_id / self.blocks_per_row;

            let x_dest_cor = self.start_cords[0] as u32 + (self.sprite_pixel_scale[0] as u32 + self.buffer_space as u32) * block_col;
            let y_row_base = self.start_cords[1] as u32 + block_row * self.row_height as u32;

            // Splice block texture useing RGB values of masking texture to identify what triangle the pixel belongs too.
            for y in 0..BLOCK_PIXLE_REZ {
                for x in 0..BLOCK_PIXLE_REZ {
                    // Get masking textures pixle color
                    let source_pixel = block_masks_image.get_pixel(x, y);
                    let [r, g, b, a] = source_pixel.0;  // Gets [u8; 4] array

                    let x_draw_cor = x + x_dest_cor;
                    let mut y_mod = y_row_base;
                    //Top Left
                    if r == 243 && g == 255 && b == 0 {
                        y_mod += (0.0 + 0.0 * self.buffer_space) as u32;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Top Right
                    else if r == 7 && g == 255 && b == 0 {
                        y_mod += (32.0 + 1.0 * self.buffer_space) as u32;
                        atlas_image.put_pixel(x_draw_cor - 32, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Left Top
                    else if r == 255 && g == 0 && b == 0 {
                        y_mod += (48.0 + 2.0 * self.buffer_space) as u32;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Left Bot
                    else if r == 253 && g == 0 && b == 232 {
                        y_mod += (64.0 + 3.0 * self.buffer_space) as u32;
                        atlas_image.put_pixel(x_draw_cor, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Right Top
                    else if r == 0 && g == 193 && b == 255 {
                        y_mod += (112.0 + 4.0 * self.buffer_space) as u32;
                        atlas_image.put_pixel(x_draw_cor - 32, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Right Bot
                    else if r == 110 && g == 0 && b == 255 {
                        y_mod += (128.0 + 5.0 * self.buffer_space) as u32;
                        atlas_image.put_pixel(x_draw_cor - 32, y + y_mod, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                }
            }
        }
    }

    // Get the pixel based SRC rect of a specific block triangle
    pub fn get_block_triangle_src_rect(&self, triangle: BlockTriangle, block: BlockTexture) -> [f32; 4] {
        let block_col = block.id() % self.blocks_per_row;
        let block_row = block.id() / self.blocks_per_row;

        let x_start_cor = self.start_cords[0] + (self.buffer_space + self.sprite_pixel_scale[0]) * block_col as f32;
        let y_start_cor = self.start_cords[1] + self.row_height * block_row as f32 + (self.buffer_space + self.sprite_pixel_scale[1]) * triangle.id() as f32;

        let x_end_cor = x_start_cor + self.sprite_pixel_scale[0];
        let y_end_cor = y_start_cor + self.sprite_pixel_scale[1];

        return [x_start_cor, y_start_cor, x_end_cor, y_end_cor];
    }

    // Get the UV based on atlas size of a specific block triangle
    pub fn get_block_triangle_uv(&self, triangle: BlockTriangle, block: BlockTexture, atlas_dimensions: f32) -> [f32; 4] {
        let src_rect = self.get_block_triangle_src_rect(triangle, block);
        let mut uv = [0.0, 0.0, 0.0, 0.0];

        // Create uv
        uv[0] = src_rect[0] / atlas_dimensions;
        uv[1] = src_rect[1] / atlas_dimensions;
        uv[2] = src_rect[2] / atlas_dimensions;
        uv[3] = src_rect[3] / atlas_dimensions;

        return uv;
    }

    // Create a pre calculate UV array of all block triangle UVs
    pub fn create_pre_calculated_block_uvs(&self, atlas_dimensions: f32) -> Vec<[[f32; 4]; 6]> {
        let mut blocks: Vec<[[f32; 4]; 6]> = Vec::new();

        for current_block in 0..self.total_blocks {
            // Create triangle uv array from block
            let mut block_triangles = [[0.0; 4]; 6];
            for current_triangle in 0..self.triangles_per_block {
                let uv = self.get_block_triangle_uv(
                    BlockTriangle::from_id(current_triangle as u16),
                    BlockTexture::from_id(current_block as u16),
                    atlas_dimensions
                );
                block_triangles[current_triangle as usize] = uv;
            }

            // Add triangle array uv to block uv vector
            blocks.push(block_triangles);
        }

        return blocks;
    }

    pub fn get_start_cords(&self) -> [f32; 2] {
        return self.start_cords;
    }

    pub fn get_end_cords(&self) -> [f32; 2] {
        return self.end_cords;
    }

}
