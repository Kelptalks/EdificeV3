use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndc, screen_data}, types::{FontType, UITextures}};

pub struct Panel {
    // Panel Rendering Data
    ndc_scale: [f32; 2],
    ndc: [f32; 2],
    ndc_center: [f32; 2],


    // Tile rendering data
    tile_ndc_scale: f32,
    tile_corner_pos: [[f32; 4]; 4],
    tile_side_pos: [[f32; 4]; 4],
    tile_center_pos: [f32; 4],

    // Text
    text: String,
    text_scale: f32,
    text_ndc: [f32; 2],
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

            // Tile rendering data
            tile_ndc_scale: 0.0,
            tile_corner_pos: [[0.0; 4]; 4],
            tile_side_pos: [[0.0; 4]; 4],
            tile_center_pos: [0.0; 4],

            // 
            text: "".to_string(),
            text_scale: 0.0,
            text_ndc: [0.0; 2],
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
    pub fn get_ending_ndc(&self) -> [f32; 2] {
        return [
            self.ndc[0] + self.ndc_scale[0],
            self.ndc[1] + self.ndc_scale[1],
        ];
    }
    pub fn get_panel_ndc_center(&self) -> [f32; 2] {
        self.ndc_center
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
        let panel_y_scale = (screen_end_ndc[1] - screen_start_ndc[1]) - (padding * 3.0);

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
        let s = self.tile_ndc_scale;
        let [x1, y1] = self.ndc;
        let x2 = x1 + self.ndc_scale[0];
        let y2 = y1 + self.ndc_scale[1];

        // Corners
        let corners = self.tile_corner_pos;
        texture_manager.render_ui_element_with_pos(UITextures::PanelTopLeft, corners[0]);
        texture_manager.render_ui_element_with_pos(UITextures::PanelTopRight, corners[1]);
        texture_manager.render_ui_element_with_pos(UITextures::PanelBotLeft, corners[2]);
        texture_manager.render_ui_element_with_pos(UITextures::PanelBotRight, corners[3]);

        // Sides
        let sides = self.tile_side_pos;
        texture_manager.render_ui_element_with_pos(UITextures::PanelTopCenter, sides[0]);
        texture_manager.render_ui_element_with_pos(UITextures::PanelBotCenter, sides[1]);
        texture_manager.render_ui_element_with_pos(UITextures::PanelMidLeft, sides[2]);
        texture_manager.render_ui_element_with_pos(UITextures::PanelMidRight, sides[3]);

        let center = [x1 + s, y1 + s, x2 - s, y2 - s];
        texture_manager.render_ui_element_with_pos(UITextures::PanelMidCenter, center);
        

        let panel_center = self.get_panel_ndc_center();
        let panel_start_cords = self.get_ndc();
        render_centered_string_at_ndc(
            texture_manager, 
            self.text.clone(), 
            FontType::Basic, 
            self.text_scale, 
            self.text_ndc,
        );

    }
    
}