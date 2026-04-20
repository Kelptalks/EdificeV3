use crate::game_data::{texture_manager::texture::Texture, types::UITextures};

#[derive(Clone, PartialEq)]
pub enum PanelColor {
    Light,
    Dark,
    Custom(u8, u8, u8),
    Clear,
}

impl PanelColor {

    fn rgb_to_float(r: u8, g: u8, b: u8) -> [f32; 3] {
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
    }

    fn to_tint(&self) -> [f32; 3] {
        match self {
            PanelColor::Light => Self::rgb_to_float(55, 113, 219),
            PanelColor::Dark => Self::rgb_to_float(25, 71, 156),
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
