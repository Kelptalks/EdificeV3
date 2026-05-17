use crate::game_data::texture_manager::texture::Texture;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UITextureData {
    pub id: u32,
    pub name: &'static str,
    pub rect: [u32; 4],  // [x, y, width, height]
    pub pressed_variant: Option<u32>,  // ID of pressed version, if any
}

const fn shift_src_rect_x(rect: [u32; 4], amount: u32) -> [u32; 4] {
    return [
        rect[0] + amount,
        rect[1],
        rect[2],
        rect[3]
    ];
}

const ICON_START_SRC_RECT:[u32; 4] = [0, 256, 32, 32];
const fn get_icon_src_rect(index: u32) -> [u32; 4] {
    return self::shift_src_rect_x(ICON_START_SRC_RECT, index * ICON_START_SRC_RECT[2])
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

    // Icons 
    UITextureData { id: 45, name: "BluePrintIcon",      rect: self::get_icon_src_rect(0), pressed_variant: None },
    UITextureData { id: 46, name: "BlockSelectionIcon", rect: self::get_icon_src_rect(1), pressed_variant: None },
    UITextureData { id: 47, name: "AreaIcon",           rect: self::get_icon_src_rect(2), pressed_variant: None },
    UITextureData { id: 48, name: "MapIcon",            rect: self::get_icon_src_rect(3), pressed_variant: None },
    UITextureData { id: 49, name: "XIcon",              rect: self::get_icon_src_rect(4), pressed_variant: None },
    UITextureData { id: 50, name: "LocationIcon",       rect: self::get_icon_src_rect(5), pressed_variant: None },
    UITextureData { id: 51, name: "SettingsIcon",       rect: self::get_icon_src_rect(6), pressed_variant: None },
    UITextureData { id: 52, name: "LeftArrowIcon",      rect: self::get_icon_src_rect(7), pressed_variant: None },
    UITextureData { id: 53, name: "LocationIcon",       rect: self::get_icon_src_rect(8), pressed_variant: None },
    UITextureData { id: 54, name: "CameraIcon",       rect: self::get_icon_src_rect(9), pressed_variant: None },
    UITextureData { id: 55, name: "BlockVarIcon",       rect: self::get_icon_src_rect(10), pressed_variant: None },
    UITextureData { id: 56, name: "ItemVarIcon",       rect: self::get_icon_src_rect(11), pressed_variant: None },
    UITextureData { id: 57, name: "LocationVarIcon",       rect: self::get_icon_src_rect(12), pressed_variant: None },
    UITextureData { id: 58, name: "ModIcon",       rect: self::get_icon_src_rect(13), pressed_variant: None },
    
    // Scalling Icons (16x16) - 3x3 grid starting at (96, 96)
    // Row 1
    UITextureData { id: 59, name: "ScallingIconTopLeft",   rect: [96,  96,  16, 16], pressed_variant: None },
    UITextureData { id: 60, name: "ScallingIconTopCenter", rect: [112, 96,  16, 16], pressed_variant: None },
    UITextureData { id: 61, name: "ScallingIconTopRight",  rect: [128, 96,  16, 16], pressed_variant: None },
    // Row 2
    UITextureData { id: 62, name: "ScallingIconMidLeft",   rect: [96,  112, 16, 16], pressed_variant: None },
    UITextureData { id: 63, name: "ScallingIconMidCenter", rect: [112, 112, 16, 16], pressed_variant: None },
    UITextureData { id: 64, name: "ScallingIconMidRight",  rect: [128, 112, 16, 16], pressed_variant: None },
    // Row 3
    UITextureData { id: 65, name: "ScallingIconBotLeft",   rect: [96,  128, 16, 16], pressed_variant: None },
    UITextureData { id: 66, name: "ScallingIconBotCenter", rect: [112, 128, 16, 16], pressed_variant: None },
    UITextureData { id: 67, name: "ScallingIconBotRight",  rect: [128, 128, 16, 16], pressed_variant: None },

    UITextureData { id: 68, name: "AnyVarIcon",       rect: self::get_icon_src_rect(14), pressed_variant: None },
    UITextureData { id: 69, name: "BoolVarIcon",      rect: self::get_icon_src_rect(15), pressed_variant: None },
    UITextureData { id: 70, name: "RefIcon",          rect: self::get_icon_src_rect(16), pressed_variant: None },
    UITextureData { id: 71, name: "SourceIcon",       rect: self::get_icon_src_rect(17), pressed_variant: None },
    UITextureData { id: 72, name: "CordsIcon",        rect: self::get_icon_src_rect(18), pressed_variant: None },
    UITextureData { id: 73, name: "NumIcon",          rect: self::get_icon_src_rect(19), pressed_variant: None },
    UITextureData { id: 74, name: "ConditionIcon",         rect: self::get_icon_src_rect(20), pressed_variant: None },
    UITextureData { id: 75, name: "IfIcon",                rect: self::get_icon_src_rect(21), pressed_variant: None },
    UITextureData { id: 76, name: "LoopIcon",              rect: self::get_icon_src_rect(22), pressed_variant: None },
    UITextureData { id: 77, name: "DroneActionPlaceIcon",  rect: self::get_icon_src_rect(23), pressed_variant: None },
    UITextureData { id: 78, name: "DroneActionPathIcon",   rect: self::get_icon_src_rect(24), pressed_variant: None },
    UITextureData { id: 79, name: "DroneActionMineIcon",   rect: self::get_icon_src_rect(25), pressed_variant: None },
    UITextureData { id: 80, name: "DroneActionEquipIcon",  rect: self::get_icon_src_rect(26), pressed_variant: None },
    UITextureData { id: 81, name: "DroneActionBurnItemIcon", rect: self::get_icon_src_rect(27), pressed_variant: None },
    UITextureData { id: 82, name: "DroneActionCraftIcon",  rect: self::get_icon_src_rect(28), pressed_variant: None },
    UITextureData { id: 83, name: "DroneActionIsBusyIcon", rect: self::get_icon_src_rect(29), pressed_variant: None },
    UITextureData { id: 84, name: "DroneActionCordsIcon",  rect: self::get_icon_src_rect(30), pressed_variant: None },
    UITextureData { id: 85, name: "DroneActionFuelIcon",   rect: self::get_icon_src_rect(31), pressed_variant: None },
    UITextureData { id: 86, name: "FuelIcon",              rect: self::get_icon_src_rect(32), pressed_variant: None },
    UITextureData { id: 87, name: "DroneActionHealthIcon", rect: self::get_icon_src_rect(33), pressed_variant: None },

];

// Keep enum for type safety if you want
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum UITextures {
    ButtonLeftArrow = 0,
    ButtonLeftArrowDown = 1,
    ButtonRightArrow = 2,
    ButtonRightArrowDown = 3,
    ButtonX = 4,
    ButtonXDown = 5,
    ButtonCheck = 6,
    ButtonCheckDown = 7,
    ButtonSlider = 8,
    ButtonSliderDown = 9,
    BarButtonLeft = 10,
    BarButtonCenter = 11,
    BarButtonRight = 12,
    BarButtonLeftDown = 13,
    BarButtonCenterDown = 14,
    BarButtonRightDown = 15,
    DropdownArrow = 16,
    DropdownArrowDown = 17,
    ButtonCircle = 18,
    ButtonCircleDown = 19,
    Speed = 20,
    SpeedDown = 21,
    Pause = 22,
    Play = 23,

    // Backgrounds
    VoidBackground = 24,
    FaceBackground = 25,
    MirrorBackground = 26,

    // Panels
    PanelTopLeft = 27,
    PanelTopCenter = 28,
    PanelTopRight = 29,
    PanelMidLeft = 30,
    PanelMidCenter = 31,
    PanelMidRight = 32,
    PanelBotLeft = 33,
    PanelBotCenter = 34,
    PanelBotRight = 35,

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

    // Icons
    BluePrintIcon = 45,
    BlockSelectionIcon = 46,
    AreaIcon = 47,
    MapIcon = 48,
    XIcon = 49,
    LocationIcon = 50,
    SettingsIcon = 51,
    LeftArrowIcon = 52,
    RightArrowIcon = 53,
    CameraIcon = 54,
    BlockVarIcon = 55,
    ItemVarIcon = 56,
    LocationVarIcon = 57,
    ModIcon = 58,


    // Scalling Icons
    ScallingIconTopLeft = 59,
    ScallingIconTopMid = 60,
    ScallingIconTopRight = 61,
    ScallingIconMidLeft = 62,
    ScallingIconMidCenter = 63,
    ScallingIconMidRight = 64,
    ScallingIconBotLeft = 65,
    ScallingIconBotMid = 66,
    ScallingIconBotRight = 67,
    AnyVarIcon = 68,
    BoolVarIcon = 69,
    RefIcon = 70,
    SourceIcon = 71,
    CordsIcon = 72,
    NumVarIcon = 73,
    ControlFlowIcon = 74,
    IfIcon = 75,
    LoopIcon = 76,
    DroneActionPlaceIcon = 77,
    DroneActionPathIcon = 78,
    DroneActionMineIcon = 79,
    DroneActionEquipIcon = 80,
    DroneActionBurnItemIcon = 81,
    DroneActionCraftIcon = 82,
    DroneActionIsBusyIcon = 83,
    DroneActionCordsIcon = 84,
    DroneActionFuelIcon = 85,
    FuelIcon = 86,
    DroneActionHealthIcon = 87,
}

impl UITextures {
    pub fn wrap_into_texture(self) -> Texture {
        Texture::UITexture(self)
    }
    
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