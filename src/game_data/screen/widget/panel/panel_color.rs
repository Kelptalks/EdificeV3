use crate::game_data::{texture_manager::texture::Texture, types::UITextures};

#[derive(Clone, PartialEq)]
pub enum PanelColor {
    Light,
    Dark,
    Clear,
}

impl PanelColor {
    pub fn get_panel_corner_textures(&self) -> [Texture; 4] {
        match self {
            PanelColor::Light => [
                UITextures::PanelTopLeftLight.wrap_into_texture(),   // top_left
                UITextures::PanelTopRightLight.wrap_into_texture(),  // top_right
                UITextures::PanelBotLeftLight.wrap_into_texture(),   // bot_left
                UITextures::PanelBotRightLight.wrap_into_texture(),  // bot_right
            ],
            PanelColor::Dark => [
                UITextures::PanelTopLeftDark.wrap_into_texture(),    // top_left
                UITextures::PanelTopRightDark.wrap_into_texture(),   // top_right
                UITextures::PanelBotLeftDark.wrap_into_texture(),    // bot_left
                UITextures::PanelBotRightDark.wrap_into_texture(),   // bot_right
            ],
            PanelColor::Clear => [
                UITextures::PanelTopLeftDark.wrap_into_texture(),    // top_left
                UITextures::PanelTopRightDark.wrap_into_texture(),   // top_right
                UITextures::PanelBotLeftDark.wrap_into_texture(),    // bot_left
                UITextures::PanelBotRightDark.wrap_into_texture(),   // bot_right
            ],
        }
    }

    pub fn get_panel_side_textures(&self) -> [Texture; 4] {
        match self {
            PanelColor::Light => [
                UITextures::PanelTopCenterLight.wrap_into_texture(),  // top
                UITextures::PanelBotCenterLight.wrap_into_texture(),  // bot
                UITextures::PanelMidLeftLight.wrap_into_texture(),    // left
                UITextures::PanelMidRightLight.wrap_into_texture(),   // right
            ],
            PanelColor::Dark => [
                UITextures::PanelTopCenterDark.wrap_into_texture(),   // top
                UITextures::PanelBotCenterDark.wrap_into_texture(),   // bot
                UITextures::PanelMidLeftDark.wrap_into_texture(),     // left
                UITextures::PanelMidRightDark.wrap_into_texture(),    // right
            ],
            PanelColor::Clear => [
                UITextures::PanelTopCenterDark.wrap_into_texture(),   // top
                UITextures::PanelBotCenterDark.wrap_into_texture(),   // bot
                UITextures::PanelMidLeftDark.wrap_into_texture(),     // left
                UITextures::PanelMidRightDark.wrap_into_texture(),    // right
            ],
        }
    }

    pub fn get_panel_center_texture(&self) -> Texture {
        match self {
            PanelColor::Light => UITextures::PanelMidCenterLight.wrap_into_texture(),
            PanelColor::Dark  => UITextures::PanelMidCenterDark.wrap_into_texture(),
            PanelColor::Clear  => UITextures::PanelMidCenterDark.wrap_into_texture(),
        }
    }
}
