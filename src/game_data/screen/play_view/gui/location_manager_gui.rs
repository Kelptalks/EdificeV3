use miniquad::{KeyCode, MouseButton};

use crate::game_data::{TextureManager, game_event_manager::game_event_manager::GameEventManager, locations::world_area::WorldArea, screen::{Button, ScreenData, play_view::{play_view_data::{self, PlayViewData}}, ui_elements::{panel::Panel, text_bar::TextBar}}, types::UITextures};



pub struct LocationManagerGUI {
    // Location Data
    area: WorldArea,
    
    // UI
    panel: Panel,
    text_bar: TextBar,

    // Rendering
    ndc: [f32; 2],
    gui_scale: [f32; 2],
    ndc_pos: [f32; 4],
    is_mouse_on: bool,
}

impl LocationManagerGUI {
    pub fn new() -> LocationManagerGUI{


        LocationManagerGUI {
            // Location data
            area: WorldArea::new_blank(),
            
            // UI
            panel: Panel::new_blank(),
            text_bar: TextBar::new_blank(),

            // Rendering
            ndc: [0.0, 0.0],
            gui_scale: [0.0, 0.0],
            ndc_pos: [0.0, 0.0, 0.0, 0.0],
            is_mouse_on: false,
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
        ];

        // Text bar
        self.text_bar.set_ndc_y_scale(0.05);
        let panel_center = self.panel.get_panel_ndc_center();
        let bar_scale = self.text_bar.get_ndc_scale();
        self.text_bar.set_ndc([
            panel_center[0] - bar_scale[0] / 2.0,
            panel_center[1] - bar_scale[1] / 2.0,
        ]);

    }

    pub fn render(
        &mut self,
        screen_data: &ScreenData,
        texture_manager: &mut TextureManager,
    ) {
        self.is_mouse_on = screen_data.mouse_on_ndc_pos(self.ndc_pos);

        self.panel.render(texture_manager);
        self.text_bar.render(texture_manager, screen_data);
    }


    pub fn key_down_event(&mut self, keycode: KeyCode) {
        self.text_bar.handle_keydown(keycode);
    }

    pub fn mouse_button_down_event(&mut self, 
        event_manager: &mut GameEventManager, 
        play_view_data: &mut PlayViewData, 
        screen_data: &ScreenData, 
        button: MouseButton
    ) {
        self.text_bar.handle_mouse_button_down(button, screen_data);
    }
}