use crate::game_data::types::UITextures;

#[derive(Clone, PartialEq)]
pub enum PanelColor {
    Light,
    Dark,
    Clear,
}

impl PanelColor {
    pub fn get_panel_corner_textures(&self) -> [UITextures; 4] {
        match self {
            PanelColor::Light => [
                UITextures::PanelTopLeftLight,   // top_left
                UITextures::PanelTopRightLight,  // top_right
                UITextures::PanelBotLeftLight,   // bot_left
                UITextures::PanelBotRightLight,  // bot_right
            ],
            PanelColor::Dark => [
                UITextures::PanelTopLeftDark,    // top_left
                UITextures::PanelTopRightDark,   // top_right
                UITextures::PanelBotLeftDark,    // bot_left
                UITextures::PanelBotRightDark,   // bot_right
            ],
            PanelColor::Clear => [
                UITextures::PanelTopLeftDark,    // top_left
                UITextures::PanelTopRightDark,   // top_right
                UITextures::PanelBotLeftDark,    // bot_left
                UITextures::PanelBotRightDark,   // bot_right
            ],
        }
    }

    pub fn get_panel_side_textures(&self) -> [UITextures; 4] {
        match self {
            PanelColor::Light => [
                UITextures::PanelTopCenterLight,  // top
                UITextures::PanelBotCenterLight,  // bot
                UITextures::PanelMidLeftLight,    // left
                UITextures::PanelMidRightLight,   // right
            ],
            PanelColor::Dark => [
                UITextures::PanelTopCenterDark,   // top
                UITextures::PanelBotCenterDark,   // bot
                UITextures::PanelMidLeftDark,     // left
                UITextures::PanelMidRightDark,    // right
            ],
            PanelColor::Clear => [
                UITextures::PanelTopCenterDark,   // top
                UITextures::PanelBotCenterDark,   // bot
                UITextures::PanelMidLeftDark,     // left
                UITextures::PanelMidRightDark,    // right
            ],
        }
    }

    pub fn get_panel_center_texture(&self) -> UITextures {
        match self {
            PanelColor::Light => UITextures::PanelMidCenterLight,
            PanelColor::Dark  => UITextures::PanelMidCenterDark,
            PanelColor::Clear  => UITextures::PanelMidCenterDark,
        }
    }
}
