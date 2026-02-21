use image::{RgbaImage, imageops};

use crate::game_data::types::BlockTexture;

static BLOCK_PIXLE_REZ: u32 = 64;
static BLOCK_TEXTURES_PER_ROW: u32 = 20;

pub struct BlockTextureManager {
    pub start_cords: [f32; 2],
    pub end_cords: [f32; 2],
   
    pub buffer_space: u32,


}

impl BlockTextureManager {

    pub fn new(start_cords: [f32; 2]) -> BlockTextureManager {
        BlockTextureManager {
            start_cords: start_cords,
            end_cords: [start_cords[0], start_cords[1]],

            buffer_space: 32,
        }
    }


    //=====================================
    // Init
    //=====================================
    pub fn splice_textures(&mut self, atlas_image: &mut RgbaImage) {
        
        // Load UI textures image
        let block_image = image::open("Assets/blocks.png").unwrap().to_rgba8(); 
        
        let block_spacing = self.buffer_space + BLOCK_PIXLE_REZ;
        // Splice every block with buffer space
        for block_id in 0..BlockTexture::get_total_blocks() {
            let block_row_index= block_id % BLOCK_TEXTURES_PER_ROW;
            let block_collumn_index = block_id / BLOCK_TEXTURES_PER_ROW;
            
            let src_pixel_start_cords = [
                BLOCK_PIXLE_REZ * block_row_index, 
                BLOCK_PIXLE_REZ * block_collumn_index,
            ];

            let dest_pixel_start_cords = [
                self.start_cords[0] as u32 + (block_spacing * block_row_index), 
                self.start_cords[1] as u32 + (block_spacing * block_collumn_index),
            ];

            for x in 0..BLOCK_PIXLE_REZ {
                for y in 0..BLOCK_PIXLE_REZ {
                    let src_cords= [
                        src_pixel_start_cords[0] + x,
                        src_pixel_start_cords[1] + y
                    ];

                    let dest_cords = [
                        dest_pixel_start_cords[0] + x,
                        dest_pixel_start_cords[1] + y
                    ];


                    atlas_image.put_pixel(
                        dest_cords[0], dest_cords[1],
                        *block_image.get_pixel(src_cords[0], src_cords[1])
                    );

                }                
            }
        }   


        // Calculate end cords
        self.end_cords[0] = (block_spacing * BLOCK_TEXTURES_PER_ROW) as f32 + self.start_cords[0];

        let total_collumns = (BlockTexture::get_total_blocks() / BLOCK_TEXTURES_PER_ROW) + 1;
        self.end_cords[1] = (block_spacing * total_collumns) as f32 + self.start_cords[1];
        println!("End Cords: {:?}", self.end_cords);
    }

    //=====================================
    // Pre calculated UVs
    //=====================================

    pub fn create_pre_calculated_block_uvs(&self, atlas_dimensions: f32) -> Vec<[f32; 4]> {
        let mut pre_calculated_block_uvs: Vec<[f32; 4]> = Vec::new();

        let block_spacing = self.buffer_space + BLOCK_PIXLE_REZ;
        for block_id in 0..BlockTexture::get_total_blocks() {
            let block_row_index= block_id % BLOCK_TEXTURES_PER_ROW;
            let block_collumn_index = block_id / BLOCK_TEXTURES_PER_ROW;
            let start_pixel_cords = [
                self.start_cords[0] + (block_row_index * block_spacing) as f32,
                self.start_cords[1] + (block_collumn_index * block_spacing) as f32,
            ];

            let end_pixel_cords = [
                start_pixel_cords[0] + BLOCK_PIXLE_REZ as f32,
                start_pixel_cords[1] + BLOCK_PIXLE_REZ as f32
            ];

            let uv = [
                start_pixel_cords[0] / atlas_dimensions,
                start_pixel_cords[1] / atlas_dimensions,

                end_pixel_cords[0] / atlas_dimensions,
                end_pixel_cords[1] / atlas_dimensions,
            ];

            pre_calculated_block_uvs.push(uv);
        }

        return pre_calculated_block_uvs;
    }

    pub fn get_end_cords(&self) -> [f32; 2] {
        return self.end_cords;
    }

}