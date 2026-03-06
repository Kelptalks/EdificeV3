use crate::game_data::{TextureManager, screen::{ScreenData, widget::widget_trait::Widget}, texture_manager, types::UITextures};


#[derive(Clone)]
pub enum PanelColor {
    Light,
    Dark,
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
        }
    }

    pub fn get_panel_center_texture(&self) -> UITextures {
        match self {
            PanelColor::Light => UITextures::PanelMidCenterLight,
            PanelColor::Dark  => UITextures::PanelMidCenterDark,
        }
    }
}



pub struct Panel {
    // Panel
    buffers: [f32; 4],
    pos: [f32; 4],
    ndc_scale: [f32; 2],
    

    // Panel Tile Rendering
    tile_ndc_scale: f32,
    tile_corner_pos: [[f32; 4]; 4],
    tile_side_pos: [[f32; 4]; 4],
    tile_center_pos: [f32; 4],

    // Aperence
    color: PanelColor,
}

impl Panel {

    fn calculate_tile_pos(&mut self) {
        let s = self.tile_ndc_scale;
        let [x1, y1] = [self.pos[0], self.pos[1]];
        let x2 = x1 + self.ndc_scale[0];
        let y2 = y1 + self.ndc_scale[1];
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
}

impl Widget for Panel {

    fn new(parent_pos: [f32; 4], side_buffers: [f32; 4]) -> Self {
        
        let pos = [
            parent_pos[0] + side_buffers[0],
            parent_pos[1] + side_buffers[1],
            parent_pos[2] - side_buffers[2],
            parent_pos[3] - side_buffers[3],
        ];

        let ndc_scale = [
            pos[2] - pos[0],
            pos[3] - pos[1],
        ];

        let mut panel = Panel {
            buffers: side_buffers,
            pos: pos,
            ndc_scale: ndc_scale,


            // Tile rendering data
            tile_ndc_scale: 0.025,
            tile_corner_pos: [[0.0; 4]; 4],
            tile_side_pos: [[0.0; 4]; 4],
            tile_center_pos: [0.0; 4],

            // Aperence
            color: PanelColor::Light,
        };

        panel.calculate_tile_pos();

        return panel;
    }

    // Getters
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }
    fn get_scale(&self) -> [f32; 2] {
        return self.ndc_scale;
    }

    fn render(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
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

        println!("test");

    }
}