use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndc}, types::{FontType, UITextures}};

#[derive(Clone)]
pub enum PanelColor {
    Light,
    Dark,
}

impl PanelColor {
    pub fn get_panel_corner_textures(&self) -> [UITextures; 4] {
        match self {
            PanelColor::Light => [
                UITextures::PanelTopLeft,   // top_left
                UITextures::PanelTopRight,  // top_right
                UITextures::PanelBotLeft,   // bot_left
                UITextures::PanelBotRight,  // bot_right
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
                UITextures::PanelTopCenter,  // top
                UITextures::PanelBotCenter,  // bot
                UITextures::PanelMidLeft,    // left
                UITextures::PanelMidRight,   // right
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
            PanelColor::Light => UITextures::PanelMidCenter,
            PanelColor::Dark  => UITextures::PanelMidCenterDark,
        }
    }
}

#[derive(Clone)]
pub struct Panel {
    // Panel Rendering Data
    ndc_scale: [f32; 2],
    ndc: [f32; 2],
    ndc_center: [f32; 2],
    ndc_pos: [f32; 4],

    // Tile rendering data
    tile_ndc_scale: f32,
    tile_corner_pos: [[f32; 4]; 4],
    tile_side_pos: [[f32; 4]; 4],
    tile_center_pos: [f32; 4],

    // Text
    text: String,
    text_scale: f32,
    text_ndc: [f32; 2],

    // Color
    color: PanelColor,
}


impl Panel {

    //=====================================
    // Constructors
    //=====================================

    pub fn new_blank() -> Panel{
        Panel {
            // Panel Rendering data
            ndc_scale: [0.0, 0.0],
            ndc: [0.0, 0.0],
            ndc_center: [0.0, 0.0],
            ndc_pos: [0.0; 4],

            // Tile rendering data
            tile_ndc_scale: 0.0,
            tile_corner_pos: [[0.0; 4]; 4],
            tile_side_pos: [[0.0; 4]; 4],
            tile_center_pos: [0.0; 4],

            //
            text: "".to_string(),
            text_scale: 0.0,
            text_ndc: [0.0; 2],

            // Color
            color: PanelColor::Light,
        }
    }

    //=====================================
    // Getters and setters
    //=====================================

    pub fn recalulate_rendering_values(&mut self) {
        // ndc
        self.ndc_center = [
            self.ndc[0] + (self.ndc_scale[0] / 2.0),
            self.ndc[1] + (self.ndc_scale[1] / 2.0),
        ];

        // Text
        self.text_scale = (self.ndc_scale[0] / self.text.len() as f32) / 1.5;
        self.text_ndc = [
            self.ndc_center[0],
            self.ndc[1] + (self.text_scale * 1.5)
        ];

        self.ndc_pos = [
            self.ndc[0],
            self.ndc[1],
            self.ndc[0] + self.ndc_scale[0],
            self.ndc[1] + self.ndc_scale[1],
        ];

        // Tile rendering
        self.calculate_tile_pos();
    }

    // Tile
    pub fn get_tile_ndc_scale(&self) -> f32 {
        return self.tile_ndc_scale;
    }
    pub fn set_tile_ndc_scale(&mut self, scale: f32) {
        self.tile_ndc_scale = scale;
        self.recalulate_rendering_values();
    }

    // Ndc
    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
        self.recalulate_rendering_values();
    }
    pub fn get_ndc(&self) -> [f32; 2] {
        return self.ndc;
    }
    pub fn get_text_ending_ndc(&mut self) -> [f32; 2] {
        return [
            self.ndc[0],
            self.ndc[1] + self.text_scale * 1.5,
        ]
    }
    pub fn get_ending_ndc(&self) -> [f32; 2] {
        return [
            self.ndc[0] + self.ndc_scale[0],
            self.ndc[1] + self.ndc_scale[1],
        ];
    }
    pub fn get_panel_ndc_center(&self) -> [f32; 2] {
        self.ndc_center
    }
    pub fn get_ndc_pos(&self) -> [f32; 4] {
        self.ndc_pos
    }

    // Scale
    pub fn set_ndc_scale(&mut self, scale: [f32; 2]) {
        self.ndc_scale = scale;
        self.recalulate_rendering_values();
    }
    pub fn get_ndc_scale(&self) -> [f32; 2] {
        return self.ndc_scale;
    }
    pub fn set_title(&mut self, title: String) {
        self.text = title;
        self.recalulate_rendering_values();
    }

    // Color
    pub fn set_color(&mut self, color: PanelColor) {
        self.color = color;
    }

    //=====================================
    // Sizing
    //=====================================

    /// Calculate the texture pos to avoid and cache to avoid recalulation each frame
    fn calculate_tile_pos(&mut self) {
        let s = self.tile_ndc_scale;
        let [x1, y1] = self.ndc;
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

    pub fn scale_to_fill_screen_left(&mut self, screen_data: &ScreenData, padding: f32, scale: f32) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        // Panel
        let panel_x_scale = (screen_end_ndc[0] - screen_start_ndc[1]) * scale;
        let panel_y_scale = (screen_end_ndc[1] - screen_start_ndc[1]) - (padding * 2.0);

        let panel_x_ndc_cor = screen_start_ndc[0] + (padding);
        let panel_y_ndc_cor = screen_start_ndc[1] + (padding);

        self.set_ndc([panel_x_ndc_cor, panel_y_ndc_cor]);
        self.set_ndc_scale([panel_x_scale, panel_y_scale]);
        self.set_tile_ndc_scale(0.025);
    }

    //=====================================
    // Rendering
    //=====================================


    pub fn render(&self, texture_manager: &mut TextureManager) {
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

        render_centered_string_at_ndc(
            texture_manager,
            self.text.clone(),
            FontType::Basic,
            self.text_scale,
            self.text_ndc,
        );

    }

}
