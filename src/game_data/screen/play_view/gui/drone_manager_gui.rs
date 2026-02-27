use crate::game_data::{TextureManager, screen::{ScreenData, play_view::play_view_data::PlayViewData, render_centered_string_at_ndc, ui_elements::panel::Panel}, types::FontType};

pub struct DroneGUIManager {
    panel: Panel,

    gui_scale: [f32; 2],
    ndc_pos: [f32; 4],
    is_mouse_on: bool,
}

impl DroneGUIManager {
    pub fn new() -> DroneGUIManager {
        DroneGUIManager {
            panel: Panel::new_blank(),

            gui_scale: [0.0, 0.0],
            ndc_pos: [0.0, 0.0, 0.0, 0.0],
            is_mouse_on: false,
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_gui_ndc_scale(&self) -> [f32; 2] {
        return self.gui_scale;
    }

    pub fn get_gui_pos(&self) -> [f32; 4] {
        return self.ndc_pos;
    }

    pub fn get_is_mouse_on(&self) -> bool {
        return self.is_mouse_on;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData, play_view_data: &PlayViewData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        let panel_padding_scale = play_view_data.get_panel_padding_scale();

        // Panel
        self.panel.scale_to_fill_screen_left(screen_data, panel_padding_scale, 0.4);
        self.panel.set_tile_ndc_scale(play_view_data.get_panel_tile_scale());
        
        self.panel.set_title("Drone Manager".to_string());
        let panel_ndc_scale = self.panel.get_ndc_scale();

        // Set GUI values
        self.gui_scale = [
            (panel_padding_scale * 2.0) + panel_ndc_scale[0],
            (panel_padding_scale * 2.0) + panel_ndc_scale[1],
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
        self.is_mouse_on = screen_data.mouse_on_ndc_pos(self.ndc_pos);

        // Render background
        self.panel.render(texture_manager);


    }

}
