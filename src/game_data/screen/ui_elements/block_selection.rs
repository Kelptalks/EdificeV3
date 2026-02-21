use crate::game_data::screen::ui_elements::panel::Panel;

pub struct BlockSelection {
    ndc: [f32; 2],
    scale: [f32; 2],
    ndc_end_cords: [f32; 2],

    panel: Panel,


}


impl BlockSelection {
    pub fn new() -> BlockSelection {
        BlockSelection {
            ndc: [0.0, 0.0],
            scale: [0.0, 0.0],
            ndc_end_cords: [0.0, 0.0],

            panel: Panel::new_blank(),
        }
    }

    //=====================================
    // Updates
    //=====================================

    fn re_calculate_ndc_end_cords(&mut self) {
        self.ndc_end_cords = [
            self.ndc[0] + self.scale[0],
            self.ndc[1] + self.scale[1],
        ];
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
        self.panel.set_ndc(ndc);
        self.re_calculate_ndc_end_cords();
    }

    pub fn set_scale(&mut self, scale: [f32; 2]) {
        self.scale = scale;
        self.panel.set_ndc_scale(scale);
        self.panel.set_tile_ndc_scale(0.025);
        self.re_calculate_ndc_end_cords();
    }

    pub fn get_ndc(&self) -> [f32; 2] {
        return self.ndc;
    }

    pub fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    pub fn get_ndc_end_cords(&self) -> [f32; 2] {
        return self.ndc_end_cords;
    }

    pub fn get_panel(&self) -> &Panel {
        return &self.panel;
    }


    //=====================================
    // Renderer
    //=====================================

    

}

