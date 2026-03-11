pub fn pos_to_scale(pos: [f32; 4]) -> [f32; 2]{
    return [
        pos[2] - pos[0],
        pos[3] - pos[1],
    ];
}

pub fn buffer_pos(pos: [f32; 4], buffer: [f32; 4]) -> [f32; 4] {
    return [
        pos[0] + buffer[0],
        pos[1] + buffer[1],
        pos[2] - buffer[2],
        pos[3] - buffer[3],
    ];
}

//=====================================
// Button values
//=====================================
pub fn get_button_scale() -> f32 {
    return 0.05;
}

//=====================================
// Panle Values
//=====================================
pub fn get_panel_tile_scale() -> f32 {
    return 0.01;
}

//=====================================
// Text values
//=====================================
#[derive(Clone, Copy)]
pub enum TextSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl TextSize {
    pub fn get_scale(&self) -> f32 {
        match self {
            TextSize::ExtraSmall => 0.015,
            TextSize::Small      => 0.022,
            TextSize::Medium     => 0.03,
            TextSize::Large      => 0.07,
            TextSize::ExtraLarge => 0.1,
        }
    }
}

pub fn get_preset_text_scale(text_size: usize) -> f32 {
    match text_size {
        0 => TextSize::ExtraSmall.get_scale(),
        1 => TextSize::Small.get_scale(),
        2 => TextSize::Medium.get_scale(),
        3 => TextSize::Large.get_scale(),
        _ => TextSize::ExtraLarge.get_scale(),
    }
}

pub fn get_button_text_scale() -> f32 {
    return TextSize::ExtraSmall.get_scale();
}

pub fn get_header_text_scale() -> f32 {
    return TextSize::Medium.get_scale();
}