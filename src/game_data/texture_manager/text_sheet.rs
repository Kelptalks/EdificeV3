use std::hash::Hash;
use std::str::Chars;
use std::collections::HashMap;

use image::RgbaImage;
use miniquad::gl::GL_TEXTURE_BASE_LEVEL;
use rand::distr::Map;

use crate::game_data::types::CharType;

/*
################
## Text Sheet ##
################
This file is responsable for managing the splicing of the text textures from the
"Assets/Text.png" image to the texture atlas. It also manages fonts locations on
the texture atlass for the generation of pre calculated Texture UVs.



*/

pub struct Font {
    name: String,
    atlas_start_cords: [f32; 2],    
    splicing_start_cords: [f32; 2],
    buffer_space: f32,
    font_pixel_scale: [f32; 2],

}

impl Font {
    pub fn new(name: &str, atlas_start_cords: [f32; 2], splicing_start_cords: [f32; 2], font_pixel_scale: [f32; 2], buffer_space: f32) -> Self {
        Self {
            name: name.to_string(),
            atlas_start_cords: atlas_start_cords,
            splicing_start_cords: splicing_start_cords,
            buffer_space: buffer_space,
            font_pixel_scale: font_pixel_scale,
        }
    }

    // Splice the font from the texture sheet to the texture atlass
    pub fn splice_font_to_atlas(&self, atlas_image: &mut RgbaImage, fonts_image: &mut RgbaImage) {

        // Upack and set up spacing values
        let splicing_start_cords = self.splicing_start_cords;
        let atlas_start_cords = self.atlas_start_cords;
        let buffered_spacing = self.font_pixel_scale[0] + self.buffer_space;

        // Loop through each letter
        for current_char in 0..CharType::get_total_chars() {
            
            // Calculate cords to splice from fonts_image
            let char_x_splicing_offset = ((current_char as f32 * (self.font_pixel_scale[0])) + splicing_start_cords[0]) as u32;
            let char_y_splicing_offset = (splicing_start_cords[1]) as u32;
        
            // Calculate atlas draw location with buffered spacing
            let char_x_atlas_offset = (current_char as f32 * (buffered_spacing) + atlas_start_cords[0]) as u32;
            let char_y_atlas_offset = (atlas_start_cords[1]) as u32;



            // Loop through each pixel in char
            for x_char_pixel_index in 0..self.font_pixel_scale[0] as u32 {
                for y_char_pixel_index in 0..self.font_pixel_scale[1] as u32 {
                    
                    // Calculate Exact pixel to copy from
                    let char_x_splicing_cor = x_char_pixel_index + char_x_splicing_offset;
                    let char_y_splicing_cor = y_char_pixel_index + char_y_splicing_offset;
                    
                    // Calculate Exact pixel to paste too
                    let char_x_atlas_cor = x_char_pixel_index + char_x_atlas_offset;
                    let char_y_atlas_cor = y_char_pixel_index + char_y_atlas_offset;

                    // Update the pixel on the atlas
                    let spliced_pixel = fonts_image.get_pixel(char_x_splicing_cor, char_y_splicing_cor);
                    atlas_image.put_pixel(char_x_atlas_cor, char_y_atlas_cor, *spliced_pixel);
                }
            }
        }
    }


    pub fn get_char_src_rect(&mut self, char: CharType) -> [f32; 4] {
        let char_index = char.get_id();
        let atlas_start_cords = self.atlas_start_cords;

        let start_x = atlas_start_cords[0] + (char_index as f32 * (self.font_pixel_scale[0] + self.buffer_space));
        let start_y = atlas_start_cords[1];
    
        let end_x = start_x + self.font_pixel_scale[0];
        let end_y = atlas_start_cords[1] + self.font_pixel_scale[1];


        return [start_x, start_y, end_x, end_y];
    }

    pub fn get_char_uv(&mut self, char: CharType, atlas_dimensions: f32) -> [f32; 4] {
        let src_rect = self.get_char_src_rect(char);
        let mut uv = [0.0, 0.0, 0.0, 0.0];

        // Create uv
        uv[0] = src_rect[0] / atlas_dimensions;
        uv[1] = src_rect[1] / atlas_dimensions;
        uv[2] = src_rect[2] / atlas_dimensions;
        uv[3] = src_rect[3] / atlas_dimensions;

        return uv;
    }


    pub fn create_pre_calculated_chars_uvs(&mut self, atlas_dimensions: f32) -> Vec<[f32; 4]>{
        let mut char_uvs = Vec::new();
        
        // Loop through all character types and calculate their UVs
        for char_id in 0..CharType::get_total_chars() {
            if let Some(char_type) = CharType::from_id(char_id) {
                let uv = self.get_char_uv(char_type, atlas_dimensions);
                char_uvs.push(uv);
            }
        }
        
        char_uvs
    }




}

pub struct TextTextureManager {
    // Texter sheet cords
    pub start_cords: [f32; 2],
    pub end_cords: [f32; 2],
    
    // Fonts
    fonts: HashMap<String, Font>,

}

impl TextTextureManager {
    // Create the Text Manager in initilize Font values
    pub fn new(start_cords : [f32; 2]) -> Self {
        let mut fonts: HashMap<String, Font> = HashMap::new();
        
        let mut current_font_starting_cor = start_cords.clone();
        let font_buffer_spacing = 8.0;

        // Add Basic font
        fonts.insert(
            "Basic".to_string(),
            Font::new("Basic",
            current_font_starting_cor,
            [0.0, 0.0],
            [16.0, 16.0],
            8.0,
            )
        );

        // Add Mini font
        current_font_starting_cor[1] += 16.0 + font_buffer_spacing; // Calculate cords for next font 
        fonts.insert(
            "Mini".to_string(),
            Font::new("Mini",
            current_font_starting_cor,
            [0.0, 26.0],
            [6.0, 6.0],
            4.0
            )
        );

        // Use for calulating the end cords of the font sheet on the atlas
        current_font_starting_cor[1] += 6.0 + font_buffer_spacing;
        current_font_starting_cor[0] += 16.0 * CharType::get_total_chars() as f32; // calcualte end cords based off total

        Self {
            start_cords : start_cords,
            end_cords: current_font_starting_cor,
            fonts: fonts,
        }
    }

    // Get a font by name
    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    // Splice all the fonts added during the creation of TextTextureManager to the atlas
    pub fn splice_fonts_to_atlas(&self, atlas_image: &mut RgbaImage) {
        let mut fonts_image = image::open("Assets/Text.png").unwrap().to_rgba8();
        for (_name, font) in &self.fonts {
            font.splice_font_to_atlas(atlas_image, &mut fonts_image);
        }
    }

    pub fn create_pre_calculated_fonts_uvs(&mut self, atlas_dimensions: f32) -> HashMap<String, Vec<[f32; 4]>>{
        let mut fonts_uvs = HashMap::new();
        
        // Loop through all fonts and calculate pre-calculated UVs for each
        for (font_name, font) in &mut self.fonts {
            let char_uvs = font.create_pre_calculated_chars_uvs(atlas_dimensions);
            fonts_uvs.insert(font_name.clone(), char_uvs);
        }
        
        fonts_uvs
    }
}
