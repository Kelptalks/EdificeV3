use crate::game_data::{TextureManager, screen::{ScreenData}, types::CharType};

pub fn render_string(screen_data: &ScreenData, texture_manager: &mut TextureManager, string : String, font : String, scale: f32, pixel_cords: [f32; 2]){
    let mut ndi_cords = screen_data.pixel_cords_to_ndc_cords(pixel_cords);



    let chars = string.chars().collect::<Vec<char>>();
    for c in chars.iter() {
        let char_type = CharType::from_char(*c);
        texture_manager.render_char(font.clone(), char_type, ndi_cords, scale);
        ndi_cords[0] += scale + (scale * 0.05); // Advance position for next character
    }
}

pub fn render_string_at_ndi_cords(texture_manager: &mut TextureManager, string: String, font: String, scale: f32, ndi_cords: [f32; 2]) {
    let mut current_ndi_cords = ndi_cords;

    let chars = string.chars().collect::<Vec<char>>();
    for c in chars.iter() {
        let char_type = CharType::from_char(*c);
        texture_manager.render_char(font.clone(), char_type, current_ndi_cords, scale);
        current_ndi_cords[0] += scale + (scale * 0.05); // Advance position for next character
    }
}

pub fn render_centered_string_at_ndi_cords(texture_manager: &mut TextureManager, string: String, font: String, scale: f32, ndi_cords: [f32; 2]) {
    // Calculate the total width of the string
    let char_count = string.chars().count();
    let total_width = (char_count as f32) * (scale + (scale * 0.05)) - (scale * 0.05); // Subtract the last spacing
    
    // Calculate the starting position to center the string
    let start_x = ndi_cords[0] - (total_width / 2.0);
    let start_y = ndi_cords[1] - scale / 2.0; // Center vertically
    let mut current_ndi_cords = [start_x, start_y];

    let chars = string.chars().collect::<Vec<char>>();
    for c in chars.iter() {
        let char_type = CharType::from_char(*c);
        texture_manager.render_char(font.clone(), char_type, current_ndi_cords, scale);
        current_ndi_cords[0] += scale + (scale * 0.05); // Advance position for next character
    }
}
