#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UITextureData {
    pub id: u32,
    pub name: &'static str,
    pub rect: [u32; 4],  // [x, y, width, height]
    pub pressed_variant: Option<u32>,  // ID of pressed version, if any
}

// All UI textures defined in one place
pub const UI_TEXTURES: &[UITextureData] = &[
    // Arrow buttons
    UITextureData { id: 0, name: "ButtonLeftArrow", rect: [0, 0, 16, 16], pressed_variant: Some(1) },
    UITextureData { id: 1, name: "ButtonLeftArrow_Down", rect: [0, 16, 16, 16], pressed_variant: None },
    UITextureData { id: 2, name: "ButtonRightArrow", rect: [16, 0, 16, 16], pressed_variant: Some(3) },
    UITextureData { id: 3, name: "ButtonRightArrow_Down", rect: [16, 16, 16, 16], pressed_variant: None },
    
    // X Button
    UITextureData { id: 4, name: "ButtonX", rect: [32, 0, 16, 16], pressed_variant: Some(5) },
    UITextureData { id: 5, name: "ButtonX_Down", rect: [32, 16, 16, 16], pressed_variant: None },
    
    // Check Button
    UITextureData { id: 6, name: "ButtonCheck", rect: [48, 0, 16, 16], pressed_variant: Some(7) },
    UITextureData { id: 7, name: "ButtonCheck_Down", rect: [48, 16, 16, 16], pressed_variant: None },
    
    // Slider Button
    UITextureData { id: 8, name: "ButtonSlider", rect: [64, 0, 16, 16], pressed_variant: Some(9) },
    UITextureData { id: 9, name: "ButtonSlider_Down", rect: [64, 16, 16, 16], pressed_variant: None },
    
    // Bar buttons
    UITextureData { id: 10, name: "BarButtonLeft", rect: [0, 32, 16, 16], pressed_variant: Some(13) },
    UITextureData { id: 11, name: "BarButtonCenter", rect: [16, 32, 16, 16], pressed_variant: Some(14) },
    UITextureData { id: 12, name: "BarButtonRight", rect: [32, 32, 16, 16], pressed_variant: Some(15) },
    UITextureData { id: 13, name: "BarButtonLeft_Down", rect: [0, 48, 16, 16], pressed_variant: None },
    UITextureData { id: 14, name: "BarButtonCenter_Down", rect: [16, 48, 16, 16], pressed_variant: None },
    UITextureData { id: 15, name: "BarButtonRight_Down", rect: [32, 48, 16, 16], pressed_variant: None },
    
    // Dropdown arrow
    UITextureData { id: 16, name: "DropdownArrow", rect: [48, 32, 16, 16], pressed_variant: Some(17) },
    UITextureData { id: 17, name: "DropdownArrow_Down", rect: [48, 48, 16, 16], pressed_variant: None },
    
    // Circle button
    UITextureData { id: 18, name: "ButtonCircle", rect: [0, 64, 32, 32], pressed_variant: Some(19) },
    UITextureData { id: 19, name: "ButtonCircle_Down", rect: [0, 96, 32, 32], pressed_variant: None },
    
    // Speed button
    UITextureData { id: 20, name: "SpeedButton", rect: [80, 16, 16, 16], pressed_variant: Some(21) },
    UITextureData { id: 21, name: "SpeedButton_Down", rect: [80, 0, 16, 16], pressed_variant: None },
    UITextureData { id: 22, name: "Pause", rect: [80, 32, 16, 16], pressed_variant: None },
    UITextureData { id: 23, name: "Play", rect: [80, 48, 16, 16], pressed_variant: None },
    
    // Backgrounds
    UITextureData { id: 24, name: "VoidBackground", rect: [96, 0, 64, 64], pressed_variant: None },
    UITextureData { id: 25, name: "FaceBackground", rect: [160, 0, 320, 176], pressed_variant: None },
    UITextureData { id: 26, name: "MirrorBackground", rect: [480, 0, 240, 128], pressed_variant: None },

    // Panels
    // Row 1
    UITextureData { id: 27, name: "PanelTopLeft", rect: [672, 128, 16, 16], pressed_variant: None },
    UITextureData { id: 28, name: "PanelTopCenter", rect: [688, 128, 16, 16], pressed_variant: None },
    UITextureData { id: 29, name: "PanelTopRight", rect: [704, 128, 16, 16], pressed_variant: None },

    // Row 2
    UITextureData { id: 30, name: "PanelMidLeft", rect: [672, 144, 16, 16], pressed_variant: None },
    UITextureData { id: 31, name: "PanelMidCenter", rect: [688, 144, 16, 16], pressed_variant: None },
    UITextureData { id: 32, name: "PanelMidRight", rect: [704, 144, 16, 16], pressed_variant: None },

    // Row 3
    UITextureData { id: 33, name: "PanelBotLeft", rect: [672, 160, 16, 16], pressed_variant: None },
    UITextureData { id: 34, name: "PanelBotCenter", rect: [688, 160, 16, 16], pressed_variant: None },
    UITextureData { id: 35, name: "PanelBotRight", rect: [704, 160, 16, 16], pressed_variant: None },

    // Dark Panels
    // Row 1
    UITextureData { id: 36, name: "PanelTopLeftDark", rect: [624, 128, 16, 16], pressed_variant: None },
    UITextureData { id: 37, name: "PanelTopCenterDark", rect: [640, 128, 16, 16], pressed_variant: None },
    UITextureData { id: 38, name: "PanelTopRightDark", rect: [656, 128, 16, 16], pressed_variant: None },

    // Row 2
    UITextureData { id: 39, name: "PanelMidLeftDark", rect: [624, 144, 16, 16], pressed_variant: None },
    UITextureData { id: 40, name: "PanelMidCenterDark", rect: [640, 144, 16, 16], pressed_variant: None },
    UITextureData { id: 41, name: "PanelMidRightDark", rect: [656, 144, 16, 16], pressed_variant: None },

    // Row 3
    UITextureData { id: 42, name: "PanelBotLeftDark", rect: [624, 160, 16, 16], pressed_variant: None },
    UITextureData { id: 43, name: "PanelBotCenterDark", rect: [640, 160, 16, 16], pressed_variant: None },
    UITextureData { id: 44, name: "PanelBotRightDark", rect: [656, 160, 16, 16], pressed_variant: None },

];

// Keep enum for type safety if you want
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum UITextures {
    ButtonLeftArrow = 0,
    ButtonLeftArrow_Down = 1,
    ButtonRightArrow = 2,
    ButtonRightArrow_Down = 3,
    ButtonX = 4,
    ButtonX_Down = 5,
    ButtonCheck = 6,
    ButtonCheck_Down = 7,
    ButtonSlider = 8,
    ButtonSlider_Down = 9,
    BarButtonLeft = 10,
    BarButtonCenter = 11,
    BarButtonRight = 12,
    BarButtonLeft_Down = 13,
    BarButtonCenter_Down = 14,
    BarButtonRight_Down = 15,
    DropdownArrow = 16,
    DropdownArrow_Down = 17,
    ButtonCircle = 18,
    ButtonCircle_Down = 19,
    Speed = 20,
    Speed_Down = 21,
    Pause = 22,
    Play = 23,

    // Backgrounds
    VoidBackground = 24,
    FaceBackground = 25,
    MirrorBackground = 26,

    // Panels
    PanelTopLeftLight = 27,
    PanelTopCenterLight = 28,
    PanelTopRightLight = 29,
    PanelMidLeftLight = 30,
    PanelMidCenterLight = 31,
    PanelMidRightLight = 32,
    PanelBotLeftLight = 33,
    PanelBotCenterLight = 34,
    PanelBotRightLight = 35,

    // Dark Panels
    PanelTopLeftDark = 36,
    PanelTopCenterDark = 37,
    PanelTopRightDark = 38,
    PanelMidLeftDark = 39,
    PanelMidCenterDark = 40,
    PanelMidRightDark = 41,
    PanelBotLeftDark = 42,
    PanelBotCenterDark = 43,
    PanelBotRightDark = 44,
}

impl UITextures {
    #[inline]
    pub fn get_id(&self) -> u32 {
        *self as u32
    }

    #[inline]
    pub fn get_data(&self) -> &'static UITextureData {
        &UI_TEXTURES[self.get_id() as usize]
    }

    #[inline]
    pub fn get_rect(&self) -> [u32; 4] {
        self.get_data().rect
    }

    #[inline]
    pub fn get_pressed_variant(&self) -> UITextures {
        if let Some(pressed_id) = self.get_data().pressed_variant {
            UITextures::from_id(pressed_id).unwrap()
        } else {
            *self
        }
    }

    pub fn from_id(id: u32) -> Option<UITextures> {
        if (id as usize) < UI_TEXTURES.len() {
            // Safe because we checked bounds and enum is repr(u32)
            Some(unsafe { std::mem::transmute(id) })
        } else {
            None
        }
    }

    pub fn get_total_ui_elements() -> u32 {
        UI_TEXTURES.len() as u32
    }
}