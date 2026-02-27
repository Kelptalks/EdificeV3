use miniquad::MouseButton;

use crate::game_data::{TextureManager, game_event_manager::game_event_manager::GameEventManager, locations::world_area::WorldArea, screen::{Button, ScreenData, play_view::play_view_data::{self, PlayViewData}, ui_elements::panel::Panel}, types::UITextures};



pub struct LocationManagerGUI {
    // Location Data
    area: WorldArea,
    
    // UI
    panel: Panel,

    // Rendering
    ndc: [f32; 2],
    gui_scale: [f32; 2],
    ndc_pos: [f32; 4],

}

impl LocationManagerGUI {
    pub fn new() -> LocationManagerGUI{


        LocationManagerGUI {
            // Location data
            area: WorldArea::new_blank(),
            
            // UI
            panel: Panel::new_blank(),            

            // Rendering
            ndc: [0.0, 0.0],
            gui_scale: [0.0, 0.0],
            ndc_pos: [0.0, 0.0, 0.0, 0.0],
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    fn resize_world_points(&mut self) {
        // Update ndc pos
        self.ndc_pos = [
            self.ndc[0],
            self.ndc[1],
            self.ndc[0] + self.gui_scale[0],
            self.ndc[1] + self.gui_scale[1],
        ];
        

    }

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
        self.resize_world_points();
    }

    pub fn set_x_scale(&mut self, scale: f32) {
        self.gui_scale[0] = scale;
        self.gui_scale[1] = scale / 4.0;
        self.resize_world_points();
    }

    pub fn get_world_area(&self) -> &WorldArea {
        return &self.area;
    }

    pub fn get_gui_pos(&self) -> [f32; 4] {
        return self.ndc_pos;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData, play_view_data: &PlayViewData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        let panel_padding_scale = play_view_data.get_panel_padding_scale();

        // Panel
        self.panel.scale_to_fill_screen_left(screen_data, panel_padding_scale, 0.5);
        self.panel.set_tile_ndc_scale(play_view_data.get_panel_tile_scale());

        self.panel.set_title("Location Manager".to_string());
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

    pub fn render(
        &mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager,
    ) {
        self.panel.render(texture_manager);
    }


    pub fn mouse_button_down_event(&mut self, 
        event_manager: &mut GameEventManager, 
        play_view_data: &mut PlayViewData, 
        screen_data: &ScreenData, 
        button: MouseButton
    ) {
        if MouseButton::Left == button {

        }
    }
}
