use crate::game_data::{TextureManager, screen::widget::panel::panel_color::PanelColor, texture_manager};

pub struct PanelTextureManager {
    // Apearence
    color: PanelColor,
    tile_ndc_scale: f32,
    
    // Rendering 
    tile_corner_pos: [[f32; 4]; 4],
    tile_side_pos: [[f32; 4]; 4],
    tile_center_pos: [f32; 4],
}


impl PanelTextureManager {

    pub fn new() -> PanelTextureManager {
        PanelTextureManager {
            // Apearence
            color: PanelColor::Light,
            tile_ndc_scale: 0.01,

            // Rendering 
            tile_corner_pos: [[0.0; 4]; 4],
            tile_side_pos: [[0.0; 4]; 4],
            tile_center_pos: [0.0; 4],


        }
    }

    pub fn set_color(&mut self, color: PanelColor) {
        self.color = color;
    }

    pub fn size(&mut self, pos: [f32; 4], scale: [f32; 2]) {
        let s = self.tile_ndc_scale;
        let [x1, y1] = [pos[0], pos[1]];
        let x2 = x1 + scale[0];
        let y2 = y1 + scale[1];
        self.tile_corner_pos = [
            [x1,     y1,     x1 + s, y1 + s],  // top_left
            [x2 - s, y1,     x2,     y1 + s],  // top_right
            [x1,     y2 - s, x1 + s, y2    ],  // bot_left
            [x2 - s, y2 - s, x2,     y2    ],  // bot_right
        ];


        self.tile_side_pos = [
            [x1 + s, y1,     x2 - s, y1 + s],  // top
            [x1 + s, y2 - s, x2 - s, y2    ],  // bot
            [x1,     y1 + s, x1 + s, y2 - s],  // left
            [x2 - s, y1 + s, x2,     y2 - s],  // right
        ];

        self.tile_center_pos = [x1 + s, y1 + s, x2 - s, y2 - s];
    }

    pub fn render(&self, texture_manager: &mut TextureManager) {
        if self.color != PanelColor::Clear {
            let corners = self.tile_corner_pos;
            let sides = self.tile_side_pos;
            let center = self.tile_center_pos;

            let corner_textures = self.color.get_panel_corner_textures();
            let side_textures = self.color.get_panel_side_textures();
            let center_texture = self.color.get_panel_center_texture();

            // Corners
            texture_manager.render_ui_element_with_pos(corner_textures[0], corners[0]);
            texture_manager.render_ui_element_with_pos(corner_textures[1], corners[1]);
            texture_manager.render_ui_element_with_pos(corner_textures[2], corners[2]);
            texture_manager.render_ui_element_with_pos(corner_textures[3], corners[3]);

            // Sides
            texture_manager.render_ui_element_with_pos(side_textures[0], sides[0]);
            texture_manager.render_ui_element_with_pos(side_textures[1], sides[1]);
            texture_manager.render_ui_element_with_pos(side_textures[2], sides[2]);
            texture_manager.render_ui_element_with_pos(side_textures[3], sides[3]);

            // Center
            texture_manager.render_ui_element_with_pos(center_texture, center);
        }
    }

}