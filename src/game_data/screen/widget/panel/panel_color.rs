use crate::game_data::{texture_manager::texture::Texture, types::UITextures};

#[derive(Clone, PartialEq)]
pub enum PanelColor {
    SuperLightUI,
    LightUI,
    DarkUI,
    SuperDarkUI,

    Orange,
    BrightOrange,

    Yellow,
    BrightYellow,

    Red,
    BrightRed,

    Green,
    BrightGreen,

    LimeGreen,

    Custom(u8, u8, u8),
    Clear,
}

impl PanelColor {

    fn rgb_to_float(r: u8, g: u8, b: u8) -> [f32; 3] {
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
    }

    fn to_tint(&self) -> [f32; 3] {
        match self {
            // UI Colors
            PanelColor::SuperLightUI => Self::rgb_to_float(140, 122, 255),
            PanelColor::LightUI => Self::rgb_to_float(117, 102, 217),
            PanelColor::DarkUI => Self::rgb_to_float(88, 76, 166),
            PanelColor::SuperDarkUI => Self::rgb_to_float(59, 50, 112),

            // Basic Colors
            PanelColor::Orange => Self::rgb_to_float(230, 100, 20),
            PanelColor::BrightOrange => Self::rgb_to_float(255, 130, 0),

            PanelColor::Yellow => Self::rgb_to_float(180, 145, 0),
            PanelColor::BrightYellow => Self::rgb_to_float(255, 225, 0),

            PanelColor::Red => Self::rgb_to_float(190, 30, 30),
            PanelColor::BrightRed => Self::rgb_to_float(255, 50, 50),

            PanelColor::Green => Self::rgb_to_float(50, 200, 50),
            PanelColor::BrightGreen => Self::rgb_to_float(80, 255, 80),

            PanelColor::LimeGreen => Self::rgb_to_float(160, 255, 0),

            // Other
            PanelColor::Clear => Self::rgb_to_float(0, 0, 0),
            PanelColor::Custom(r, g, b) => Self::rgb_to_float(*r, *g, *b),
        }
    }
        
    pub fn get_panel_corner_textures(&self) -> [Texture; 4] {
        let color= self.to_tint();
        [
            Texture::TintedUITexture(UITextures::PanelTopLeft, color),
            Texture::TintedUITexture(UITextures::PanelTopRight, color),
            Texture::TintedUITexture(UITextures::PanelBotLeft, color),
            Texture::TintedUITexture(UITextures::PanelBotRight, color),
        ]
    }

    pub fn get_panel_side_textures(&self) -> [Texture; 4] {
        let color= self.to_tint();
        [
            Texture::TintedUITexture(UITextures::PanelTopCenter, color),  // top
            Texture::TintedUITexture(UITextures::PanelBotCenter, color),  // bot
            Texture::TintedUITexture(UITextures::PanelMidLeft, color),    // left
            Texture::TintedUITexture(UITextures::PanelMidRight, color),   // right
        ]
    }

    pub fn get_panel_center_texture(&self) -> Texture {
        let color= self.to_tint();
        Texture::TintedUITexture(UITextures::PanelMidCenter, color)
        
    }
}
