use crate::game_data::{TextureManager, screen::{Button, ScreenData, ui_elements::panel::Panel}, types::UITextures};

pub struct GUIManager {
    panel: Panel,
    button_map: Button,
}

impl GUIManager {
    pub fn new() -> GUIManager {
        GUIManager {
            panel: Panel::new_blank(),
            button_map: Button::new_blank(UITextures::ButtonCircle)
            
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_ending_ndc(&self) -> [f32; 2] {
        return self.panel.get_ending_ndc();
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        // Free Buttons


        // Panel
        let x_scale = (screen_end_ndc[0] - screen_start_ndc[1]) / 3.0;
        let y_scale = (screen_end_ndc[1] - screen_start_ndc[1]) * 0.9;

        let x_ndc_cor = screen_start_ndc[0] + (0.05 * y_scale);
        let y_ndc_cor = screen_start_ndc[1] + (0.05 * y_scale);

        self.panel.set_ndc([x_ndc_cor, y_ndc_cor]);
        self.panel.set_scale([x_scale, y_scale]);
        self.panel.set_tile_scale(0.025);

    }



    pub fn render_view(&mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager,
    ) {
        // Render background
        self.panel.render(texture_manager);


    }
}