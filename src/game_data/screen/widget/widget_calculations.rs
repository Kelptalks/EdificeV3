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

pub fn is_pos_contained_within_pos(box_pos: [f32; 4], internal_pos: [f32; 4]) -> bool {
    if box_pos[0] > internal_pos[0] {
        return false;
    }
    else if box_pos[1] > internal_pos[1] {
        return false;
    }
    else if box_pos[2] < internal_pos[2] {
        return false;
    }
    else if box_pos[3] < internal_pos[3] {
        return false;
    }

    return true;
}

pub fn is_pos_overlapping_pos(a: [f32; 4], b: [f32; 4]) -> bool {
    a[0] < b[2] && a[2] > b[0] && a[1] < b[3] && a[3] > b[1]
}

//=====================================
// Button values
//=====================================
pub fn get_button_scale() -> f32 {
    return 0.06;
}

pub fn get_panel_spacing_scale() -> f32 {
    return 0.005;
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
    ExtraExtraSmall,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl TextSize {
    pub fn get_scale(&self) -> f32 {
        match self {
            TextSize::ExtraExtraSmall => 0.01,
            TextSize::ExtraSmall => 0.015,
            TextSize::Small      => 0.022,
            TextSize::Medium     => 0.03,
            TextSize::Large      => 0.05,
            TextSize::ExtraLarge => 0.1,
        }
    }
}

pub fn get_button_text_scale() -> f32 {
    return TextSize::ExtraSmall.get_scale();
}