use crate::game_data::{texture_manager::texture::Texture, types::UITextures};

#[derive(Clone, PartialEq)]
pub enum PanelColor {
    SuperLightBlue,
    LightBlue,
    DarkBlue,
    Custom(u8, u8, u8),
    Clear,
}

impl PanelColor {

    fn rgb_to_float(r: u8, g: u8, b: u8) -> [f32; 3] {
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
    }

    fn to_tint(&self) -> [f32; 3] {
        match self {
            PanelColor::SuperLightBlue => Self::rgb_to_float(2, 245, 215),
            PanelColor::LightBlue => Self::rgb_to_float(0, 213, 186),
            PanelColor::DarkBlue => Self::rgb_to_float(1, 122, 107),
            PanelColor::Clear => Self::rgb_to_float(0, 0, 0),
            PanelColor::Custom(r, g, b) => Self::rgb_to_float(*r, *g, *b)
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
