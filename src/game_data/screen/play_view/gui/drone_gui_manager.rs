use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndc, ui_elements::panel::Panel}, types::FontType};

pub struct DroneGUIManager {
    panel: Panel,

    panal_padding_ndc_scale: f32,
    gui_scale: [f32; 2],
    ndc_pos: [f32; 4],
}

impl DroneGUIManager {
    pub fn new() -> DroneGUIManager {
        DroneGUIManager {
            panel: Panel::new_blank(),

            panal_padding_ndc_scale: 0.025,
            gui_scale: [0.0, 0.0],
            ndc_pos: [0.0, 0.0, 0.0, 0.0],

        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn get_gui_ndc_scale(&self) -> [f32; 2] {
        return self.gui_scale;
    }

    pub fn get_gui_pos(&self) -> [f32; 4] {
        return self.ndc_pos;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        // Panel
        self.panel.scale_to_fill_screen_left(screen_data, self.panal_padding_ndc_scale, 0.4);
        self.panel.set_title("Drone UI".to_string());
        let panel_ndc_scale = self.panel.get_ndc_scale();

        // Set GUI values
        self.gui_scale = [
            (self.panal_padding_ndc_scale * 2.0) + panel_ndc_scale[0],
            (self.panal_padding_ndc_scale * 2.0) + panel_ndc_scale[1],
        ];

        self.ndc_pos = [
            screen_start_ndc[0], 
            screen_start_ndc[1],
            screen_start_ndc[0] + self.gui_scale[0],
            screen_start_ndc[1] + self.gui_scale[1],
        ]
        
    }

    pub fn render(&mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager,
    ) {
        // Render background
        self.panel.render(texture_manager);


    }

}
