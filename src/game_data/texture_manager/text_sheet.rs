use std::str::Chars;

use image::RgbaImage;
use miniquad::gl::GL_TEXTURE_BASE_LEVEL;

use crate::game_data::Types::CharType;

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
        let splicing_start_cords = self.splicing_start_cords;
        let atlas_start_cords = self.atlas_start_cords;

        let buffered_spacing = (self.font_pixel_scale[0] + self.buffer_space);

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
                    let char_x_splicing_cor = x_char_pixel_index + char_x_splicing_offset;
                    let char_y_splicing_cor = y_char_pixel_index + char_y_splicing_offset;
                    
                    let char_x_atlas_cor = x_char_pixel_index + char_x_atlas_offset;
                    let char_y_atlas_cor = y_char_pixel_index + char_y_atlas_offset;

                    let spliced_pixel = fonts_image.get_pixel(char_x_splicing_cor, char_y_splicing_cor);
                    atlas_image.put_pixel(char_x_atlas_cor, char_y_atlas_cor, *spliced_pixel);
                }
            }
        }

    }
}

pub struct TextTextureManager {
    // Texter sheet cords
    pub start_cords: [f32; 2],
    pub end_cords: [f32; 2],
    
    // Fonts
    fonts: Vec<Font>,

}

impl TextTextureManager {
    pub fn new(start_cords : [f32; 2]) -> Self {
        let mut fonts: Vec<Font> = Vec::new();
        
        let mut current_font_starting_cor = start_cords.clone();
        let font_buffer_spacing = 8.0;

        // Add Basic font
        fonts.push(
            Font::new("Basic",
            current_font_starting_cor,
            [0.0, 0.0],
            [16.0, 16.0],
            8.0,
            )
        );

        // Add Mini font
        current_font_starting_cor[1] += 16.0 + font_buffer_spacing; // Calculate cords for next font 
        fonts.push(
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

    pub fn splice_fonts_to_atlas(&self, atlas_image: &mut RgbaImage) {
        let mut fonts_image = image::open("Assets/Text.png").unwrap().to_rgba8();
        for font in &self.fonts {
            font.splice_font_to_atlas(atlas_image, &mut fonts_image);
        }
    }
}
