use crate::game_data::{TextureManager, screen::screen_mananager::{self, ScreenManager}, types::CharType};

pub fn render_string(screen_mananager: &ScreenManager, texture_manager: &mut TextureManager, string : String, font : String, scale: f32, ndi_cords: [f32; 2]){
    let mut ndi_cords = screen_mananager.get_control_manager().get_mouse_ndc_cords();

    let chars = string.chars().collect::<Vec<char>>();
    for c in chars.iter() {
        let char_type = CharType::from_char(*c);
        texture_manager.render_char(font.clone(), char_type, ndi_cords, scale);
        ndi_cords[0] += scale + (scale * 0.05); // Advance position for next character
    }
}