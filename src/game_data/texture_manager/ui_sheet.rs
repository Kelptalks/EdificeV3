use image::RgbaImage;

use crate::game_data::{screen::text, types::{DroneItemTexture, DroneUITexture, UITextures}};


/*
########################
## UI Texture manager ##
########################
Responsable for splicing the ui Sprite sheet, and generating
the pre calculated UVs for UITextures, DroneUITextures, and 
DroneItemTextures
*/

pub struct UITextureManager {
    pub start_cords: [f32; 2],
    pub end_cords: [f32; 2],
    
}

impl UITextureManager {
    pub fn new(start_cords: [f32; 2]) -> UITextureManager {
        UITextureManager {
            start_cords: start_cords,
            end_cords: [start_cords[0], start_cords[1]],
        }
    }

    //=====================================
    // Init
    //=====================================

    pub fn splice_textures(&mut self, atlas_image: &mut RgbaImage) {
        // Load UI textures image
        let mut ui_image = image::open("Assets/UI.png").unwrap().to_rgba8(); 

        self.end_cords[0] = ui_image.height() as f32;

        // Copy image to atlas
        image::imageops::replace(atlas_image, &mut ui_image, self.start_cords[0] as i64, self.start_cords[1] as i64);
    }

    //=====================================
    // Helper
    //=====================================

    fn create_uv_from_src_rect(&self, atlas_dimensions: f32, texture_src_rect: [u32; 4]) -> [f32; 4] {
        let uv_x_start = (texture_src_rect[0] as f32 + self.start_cords[0]) / atlas_dimensions;
        let uv_y_start = (texture_src_rect[1] as f32 + self.start_cords[1]) / atlas_dimensions;

        let uv_x_end = ((texture_src_rect[0] + texture_src_rect[2]) as f32 + self.start_cords[0]) / atlas_dimensions;
        let uv_y_end = ((texture_src_rect[1] + texture_src_rect[3]) as f32 + self.start_cords[1]) / atlas_dimensions;

        return [uv_x_start, uv_y_start, uv_x_end, uv_y_end];
    }

    //=====================================
    // Pre calculated UVs
    //=====================================

    pub fn create_pre_calculated_ui_uvs(&self, atlas_dimensions: f32) -> Vec<[f32; 4]> {
        let mut pre_calculated_ui_uvs: Vec<[f32; 4]> = Vec::new();

        // Loop through each UI texture
        for current_ui_texture in 0..UITextures::get_total_ui_elements() {
            // Calculate UVs
            let texture_src_rect = UITextures::from_id(current_ui_texture).unwrap().get_rect();
            let texture_uv = self.create_uv_from_src_rect(atlas_dimensions, texture_src_rect);
            pre_calculated_ui_uvs.push(texture_uv);
        }

        return pre_calculated_ui_uvs;
    }


    pub fn create_pre_calculated_drone_ui_uvs(&self, atlas_dimensions: f32) -> Vec<[f32; 4]> {
        let mut pre_calculated_uvs: Vec<[f32; 4]> = Vec::new();

        // Loop through each UI texture
        for current_ui_texture in 0..DroneUITexture::get_total_UI_elements() {
            // Calculate UVs
            let texture_src_rect = DroneUITexture::from_id(current_ui_texture).unwrap().ui_texture_to_sprite_sheet_src_rect();
            let texture_uv = self.create_uv_from_src_rect(atlas_dimensions, texture_src_rect);
            pre_calculated_uvs.push(texture_uv);
        }

        return pre_calculated_uvs;
    }


    pub fn create_pre_calculated_drone_item_uvs(&self, atlas_dimensions: f32) -> Vec<[f32; 4]> {
        let mut pre_calculated_uvs: Vec<[f32; 4]> = Vec::new();

        // Loop through each UI texture
        for current_ui_texture in 0..DroneItemTexture::get_total_UI_elements() {
            // Calculate UVs
            let texture_src_rect = DroneItemTexture::from_id(current_ui_texture).unwrap().ui_texture_to_sprite_sheet_src_rect();
            let texture_uv = self.create_uv_from_src_rect(atlas_dimensions, texture_src_rect);
            pre_calculated_uvs.push(texture_uv);
        }

        return pre_calculated_uvs;
    }
}