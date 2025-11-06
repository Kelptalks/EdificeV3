pub struct screen_cord_tool {
    viewport_rez: [u32; 2],
    screen_rez : [u32; 2],


}

impl screen_cord_tool {
    pub fn new(viewport_rez: [u32; 2], screen_rez: [u32; 2]) -> Self {
        Self {
            viewport_rez: viewport_rez,
            screen_rez : screen_rez,
        }
    }

    //=====================================
    // Screen update functions
    //=====================================

    pub fn update_screen_rez(&mut self, new_screen_rez: [u32; 2]) {
        self.screen_rez = new_screen_rez;
    }

    //=====================================
    // Conversion Functions
    //=====================================

    //pub fn ndi_to_screen_cords(&self, ndc_cords: [f32; 2]) -> [u32; 2] {
        // I need to factor in that the viewport size to calculate ndc cord the screen starts
        
    
}
