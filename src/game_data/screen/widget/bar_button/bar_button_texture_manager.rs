use crate::game_data::{TextureManager, types::UITextures};

pub struct BarButtonTextureManager {
    left_pos:   [f32; 4],
    center_pos: [f32; 4],
    right_pos:  [f32; 4],
}

impl BarButtonTextureManager {
    pub fn new() -> BarButtonTextureManager {
        BarButtonTextureManager {
            left_pos:   [0.0; 4],
            center_pos: [0.0; 4],
            right_pos:  [0.0; 4],
        }
    }

    pub fn size(&mut self, pos: [f32; 4]) {
        let tile_size = pos[3] - pos[1];
        self.left_pos   = [pos[0],             pos[1], pos[0] + tile_size, pos[3]];
        self.right_pos  = [pos[2] - tile_size, pos[1], pos[2],             pos[3]];
        self.center_pos = [pos[0] + tile_size, pos[1], pos[2] - tile_size, pos[3]];
    }

    pub fn render(&self, texture_manager: &mut TextureManager, is_hovered: bool) {
        let (left, center, right) = if is_hovered {
            (UITextures::BarButtonLeftDown, UITextures::BarButtonCenterDown, UITextures::BarButtonRightDown)
        } else {
            (UITextures::BarButtonLeft, UITextures::BarButtonCenter, UITextures::BarButtonRight)
        };

        texture_manager.render_ui_element_with_pos(left,   self.left_pos);
        texture_manager.render_ui_element_with_pos(center, self.center_pos);
        texture_manager.render_ui_element_with_pos(right,  self.right_pos);
    }
}
