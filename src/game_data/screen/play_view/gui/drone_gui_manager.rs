use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndc, ui_elements::panel::Panel}, types::FontType};

pub struct DroneGUIManager {
    panel: Panel,
}

impl DroneGUIManager {
    pub fn new() -> DroneGUIManager {
        DroneGUIManager {
            panel: Panel::new_blank(),
        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        // Panel
        self.panel.scale_to_fill_screen_left(screen_data, 0.025, 0.2);
        self.panel.set_title("Drone UI".to_string());

        
    }

    pub fn render(&mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager,
    ) {
        // Render background
        self.panel.render(texture_manager);


    }

}
