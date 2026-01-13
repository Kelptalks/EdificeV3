#[derive(Clone, Copy)]
pub enum UITextures {
    // Left Arrow Button
    ButtonLeftArrow = 0,
    ButtonLeftArrow_Down = 1,

    // Right Arrow Button
    ButtonRightArrow = 2,
    ButtonRightArrow_Down =3,
    
    // X Button
    ButtonX = 4,
    ButtonX_Down = 5,    
    
    // Check Button
    ButtonCheck = 6,
    ButtonCheck_Down = 7,
    
    // Slider Button
    ButtonSlider = 8,
    ButtonSlider_Down = 9,
    // Bar button split for resizing
    BarButtonLeft = 10,
    BarButtonCenter = 11,
    BarButtonRight = 12,
    
    BarButtonLeft_Down = 13,
    BarButtonCenter_Down = 14,
    BarButtonRight_Down = 15,
    
    // Dropdown arrow
    DropdownArrow = 16,
    DropdownArrow_Down = 17,

    // circle button
    ButtonCircle = 18,
    ButtonCircle_Down = 19,

    // Backgrounds
    VoidBackground = 20,
    FaceBackground = 21,
    MirrorBackground = 22,

}

impl UITextures {
    /// Get the ID of this UI texture
    pub fn get_id(&self) -> u32 {
        *self as u32
    }

    /// Get the total number of UI elements
    pub fn get_total_UI_elements() -> u32 {
        UITextures::MirrorBackground.get_id() + 1 // ButtonCircle_Down is the last element at index 19, so total is 20
    }

    /// Get the pressed (down) variant of this button
    /// If the button is already pressed, returns itself
    pub fn get_pressed_variant(&self) -> UITextures {
        match self {
            UITextures::ButtonLeftArrow => UITextures::ButtonLeftArrow_Down,
            UITextures::ButtonRightArrow => UITextures::ButtonRightArrow_Down,
            UITextures::ButtonX => UITextures::ButtonX_Down,
            UITextures::ButtonCheck => UITextures::ButtonCheck_Down,
            UITextures::ButtonSlider => UITextures::ButtonSlider_Down,
            UITextures::BarButtonLeft => UITextures::BarButtonLeft_Down,
            UITextures::BarButtonCenter => UITextures::BarButtonCenter_Down,
            UITextures::BarButtonRight => UITextures::BarButtonRight_Down,
            UITextures::DropdownArrow => UITextures::DropdownArrow_Down,
            UITextures::ButtonCircle => UITextures::ButtonCircle_Down,
            // If already pressed, return itself
            _ => *self,
        }
    }

    /// Create a UITextures from an ID
    pub fn from_id(id: u32) -> Option<UITextures> {
        match id {
            0 => Some(UITextures::ButtonLeftArrow),
            1 => Some(UITextures::ButtonLeftArrow_Down),
            2 => Some(UITextures::ButtonRightArrow),
            3 => Some(UITextures::ButtonRightArrow_Down),
            4 => Some(UITextures::ButtonX),
            5 => Some(UITextures::ButtonX_Down),
            6 => Some(UITextures::ButtonCheck),
            7 => Some(UITextures::ButtonCheck_Down),
            8 => Some(UITextures::ButtonSlider),
            9 => Some(UITextures::ButtonSlider_Down),
            10 => Some(UITextures::BarButtonLeft),
            11 => Some(UITextures::BarButtonCenter),
            12 => Some(UITextures::BarButtonRight),
            13 => Some(UITextures::BarButtonLeft_Down),
            14 => Some(UITextures::BarButtonCenter_Down),
            15 => Some(UITextures::BarButtonRight_Down),
            16 => Some(UITextures::DropdownArrow),
            17 => Some(UITextures::DropdownArrow_Down),
            18 => Some(UITextures::ButtonCircle),
            19 => Some(UITextures::ButtonCircle_Down),
            20 => Some(UITextures::VoidBackground),
            21 => Some(UITextures::FaceBackground),
            22 => Some(UITextures::MirrorBackground),
            _ => None,
        }
    }

    pub fn ui_texture_to_sprite_sheet_src_rect(&self) -> [u32; 4] {
        match self {
            UITextures::ButtonLeftArrow => [0, 0, 16, 16],
            UITextures::ButtonLeftArrow_Down => [0, 16, 16, 16],

            UITextures::ButtonRightArrow => [16, 0, 16, 16],
            UITextures::ButtonRightArrow_Down => [16, 16, 16, 16],

            UITextures::ButtonX => [32, 0, 16, 16],
            UITextures::ButtonX_Down => [32, 16, 16, 16],

            UITextures::ButtonCheck => [48, 0, 16, 16],
            UITextures::ButtonCheck_Down => [48, 16, 16, 16],

            UITextures::ButtonSlider => [64, 0, 16, 16],
            UITextures::ButtonSlider_Down => [64, 16, 16, 16],

            UITextures::BarButtonLeft => [0, 32, 16, 16],
            UITextures::BarButtonCenter => [16, 32, 16, 16],
            UITextures::BarButtonRight => [32, 32, 16, 16],

            UITextures::BarButtonLeft_Down => [0, 48, 16, 16],
            UITextures::BarButtonCenter_Down => [16, 48, 16, 16],
            UITextures::BarButtonRight_Down => [32, 48, 16, 16],

            UITextures::DropdownArrow => [48, 32, 16, 16],
            UITextures::DropdownArrow_Down => [48, 48, 16, 16],

            UITextures::ButtonCircle => [0, 64, 32, 32],
            UITextures::ButtonCircle_Down => [0, 96, 32, 32],

            UITextures::VoidBackground => [96, 0, 64, 64],
            UITextures::FaceBackground => [160, 0, 320, 176],
            UITextures::MirrorBackground => [480, 0, 240, 128],
        }
    }

}


pub enum UIAnimation {
    LoadingBlue,
    LoadingRed,
}

impl UIAnimation {
    pub fn ui_animation_to_sprite_sheet_src_rect(&self, frame_number: u32) -> [u32; 4] {
        if frame_number >= self.get_total_frames() {
            panic!("Frame number {} exceeds total frames {}", frame_number, self.get_total_frames());
            return [0, 0, 0, 0];
        }
        
        match self {
            UIAnimation::LoadingBlue => {
                let x_cor = frame_number * 16;
                let y_cor = 64;
                [x_cor, y_cor, 16, 16]
            },
            UIAnimation::LoadingRed => {
                let x_cor = frame_number * 16;
                let y_cor = 80;
                [x_cor, y_cor, 16, 16]
            },
        }
    }

    pub fn get_total_frames(&self) -> u32 {
        match self {
            UIAnimation::LoadingBlue => 6,
            UIAnimation::LoadingRed => 9,
        }
    }
}
