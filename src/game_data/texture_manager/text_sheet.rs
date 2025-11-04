use image::RgbaImage;

pub struct font {
    start_cords: [f32; 2],    
    buffer_space: f32,
    font_pixel_scale: [f32; 2],
    total_chars: u32,

}

impl font {
    pub fn new(start_cords: [f32; 2], font_pixel_scale: [f32; 2], total_chars: u32) -> Self {
        Self {
            start_cords: start_cords,
            buffer_space: 8.0,
            font_pixel_scale: font_pixel_scale,
            total_chars: total_chars,
        }
    }

    // Splice the font from the texture sheet to the texture atlass
    pub fn splice_font_to_atlas(&self, atlas_image: &mut RgbaImage, fonts_image: &mut RgbaImage, font_texture_start_cords: [f32; 2]) {
        
    }
}

pub struct TextTextureManager {
    // Texter sheet cords
    pub start_cords: [f32; 2],
    pub end_cords: [f32; 2],

    // Texture alignment  
    buffer_space: f32,
    total_fonts: u32,

}

impl TextTextureManager {
    pub fn new(start_cords : [f32; 2]) -> Self {
        let fonts_image = image::open("Assets/Text.png").unwrap().to_rgba8();

        Self {
            start_cords : start_cords,
            end_cords: [0.0, 0.0],
            buffer_space: 8.0,
            total_fonts: 2,
        }

    }
}
