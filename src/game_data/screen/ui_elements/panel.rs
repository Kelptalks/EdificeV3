use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndc, screen_data}, types::{FontType, UITextures}};

pub struct Panel {
    // Panel Rendering Data
    ndc_scale: [f32; 2],
    ndc: [f32; 2],
    ndc_center: [f32; 2],


    // Tile rendering data
    tile_ndc_scale: f32,
    tile_dimensions: [u32; 2],

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
            tile_dimensions: [0, 0],

            // 
            text: "".to_string(),
            text_scale: 0.0,
            text_ndc: [0.0, 0.0],
        }
    }

    //=====================================
    // Getters and setters
    //=====================================

    pub fn get_tile_ndc_scale(&self) -> f32 {
        return self.tile_ndc_scale;
    }
    pub fn set_tile_ndc_scale(&mut self, scale: f32) {
        self.tile_ndc_scale = scale;

        self.tile_dimensions = [
            (self.ndc_scale[0] / scale) as u32,
            (self.ndc_scale[1] / scale) as u32,
        ];

        self.ndc_center = [
            self.ndc[0] + (self.ndc_scale[0] / 2.0),
            self.ndc[1] + (self.ndc_scale[1] / 2.0),
        ];
    }

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
    }

    pub fn set_ndc_scale(&mut self, scale: [f32; 2]) {
        self.ndc_scale = scale;
    }
    pub fn get_ndc_scale(&self) -> [f32; 2] {
        return self.ndc_scale;
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

    pub fn set_title(&mut self, title: String) {
        self.text = title;
        self.text_scale = (self.ndc_scale[0] / self.text.len() as f32) / 1.5;
        self.text_ndc = [
            self.ndc_center[0], 
            self.ndc[1] + (self.text_scale * 1.5)
        ];
    }

    //=====================================
    // Sizing
    //=====================================

    pub fn scale_to_fill_screen_left(&mut self, screen_data: &ScreenData, padding: f32, scale: f32) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        // Free Buttons
        

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
    pub fn tile_cords_to_texture(&self, cords: [i32; 2]) -> UITextures {
        let x_start = cords[0] == 0;
        let x_end = cords[0] == self.tile_dimensions[0] as i32;
        let y_start = cords[1] == 0;
        let y_end = cords[1] == self.tile_dimensions[1] as i32;

        match (x_start, x_end, y_start, y_end) {
            // Corners
            (true, _, true, _) => UITextures::PanelTopLeft,
            (_, true, true, _) => UITextures::PanelTopRight,
            (true, _, _, true) => UITextures::PanelBotLeft,
            (_, true, _, true) => UITextures::PanelBotRight,
            
            // Edges
            (true, _, _, _) => UITextures::PanelMidLeft,
            (_, true, _, _) => UITextures::PanelMidRight,
            (_, _, true, _) => UITextures::PanelTopCenter,
            (_, _, _, true) => UITextures::PanelBotCenter,
            
            // Center
            _ => UITextures::PanelMidCenter,
        }
    }


    pub fn render(&self, texture_manager: &mut TextureManager) {

        let s = self.tile_ndc_scale;
        let [x1, y1] = self.ndc;
        let x2 = x1 + self.ndc_scale[0];
        let y2 = y1 + self.ndc_scale[1];

        let corners = [
            [x1,     y1,     x1 + s, y1 + s],  // top_left
            [x2 - s, y1,     x2,     y1 + s],  // top_right
            [x1,     y2 - s, x1 + s, y2    ],  // bot_left
            [x2 - s, y2 - s, x2,     y2    ],  // bot_right
        ];

        let sides = [
            [x1 + s, y1,     x2 - s, y1 + s],  // top
            [x1 + s, y2 - s, x2 - s, y2    ],  // bot
            [x1,     y1 + s, x1 + s, y2 - s],  // left
            [x2 - s, y1 + s, x2,     y2 - s],  // right
        ];

        let center = [x1 + s, y1 + s, x2 - s, y2 - s];


        for x in 0..=self.tile_dimensions[0] {
            for y in 0..=self.tile_dimensions[1] as usize {
                let ndc_cords = [
                    self.ndc[0] + (x as f32 * self.tile_ndc_scale),
                    self.ndc[1] + (y as f32 * self.tile_ndc_scale),
                ];

                let texture = self.tile_cords_to_texture([x as i32, y as i32]);


                texture_manager.render_ui_element(texture, ndc_cords, self.tile_ndc_scale);
            }
        }

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