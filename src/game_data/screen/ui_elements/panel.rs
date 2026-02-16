use crate::game_data::{TextureManager, types::UITextures};

pub struct Panel {
    // Panel Rendering Data
    panel_ndc_scale: [f32; 2],
    panel_ndc_cords: [f32; 2],


    // Tile rendering data
    tile_texture_ndc_scale: f32,
    tile_dimensions: [u32; 2],
}


impl Panel {

    //=====================================
    // Constructors
    //=====================================

    pub fn new_blank() -> Panel{
        Panel {
            // Panel Rendering data
            panel_ndc_scale: [0.0, 0.0], 
            panel_ndc_cords: [0.0, 0.0],

            // Tile rendering data
            tile_texture_ndc_scale: 0.0,
            tile_dimensions: [0, 0]
        }
    }

    //=====================================
    // Getters and setters
    //=====================================

    pub fn set_tile_scale(&mut self, scale: f32) {
        self.tile_texture_ndc_scale = scale;

        self.tile_dimensions = [
            (self.panel_ndc_scale[0] / scale) as u32,
            (self.panel_ndc_scale[1] / scale) as u32,
        ];
    }

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.panel_ndc_cords = ndc;
    }

    pub fn set_scale(&mut self, scale: [f32; 2]) {
        self.panel_ndc_scale = scale;
    }

    pub fn get_ending_ndc(&self) -> [f32; 2] {
        return [
            self.panel_ndc_cords[0] + self.panel_ndc_scale[0],
            self.panel_ndc_cords[1] + self.panel_ndc_scale[1],
        ];
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

        for x in 0..=self.tile_dimensions[0] {
            for y in 0..=self.tile_dimensions[1] as usize {
                let ndc_cords = [
                    self.panel_ndc_cords[0] + (x as f32 * self.tile_texture_ndc_scale),
                    self.panel_ndc_cords[1] + (y as f32 * self.tile_texture_ndc_scale),
                ];

                let texture = self.tile_cords_to_texture([x as i32, y as i32]);


                texture_manager.render_ui_element(texture, ndc_cords, self.tile_texture_ndc_scale);
            
            }
        }


    }
    
}