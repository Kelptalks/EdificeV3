use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{Button, ScreenData, play_view::play_view_data::{self, PlayViewData}, ui_elements::panel::{self, Panel}}};

pub struct ButtonSlot {
    button: Option<Button>,
    panel: Panel,
}

impl ButtonSlot {
    pub fn new_blank(play_view_data: &mut PlayViewData) -> ButtonSlot {
        let mut panel = Panel::new_blank();
        panel.set_color(crate::game_data::screen::ui_elements::panel::PanelColor::Dark);
        panel.set_tile_ndc_scale(play_view_data.get_panel_tile_scale());

        ButtonSlot {
            button: None,
            panel: panel,
        }
    }

    //=====================================
    // Setters / Getters
    //=====================================

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.panel.set_ndc(ndc);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.panel.set_ndc_scale([scale, scale]);
    }

    pub fn mouse_on(&mut self, screen_data: &ScreenData) -> bool{
        return screen_data.mouse_on_ndc_pos(self.panel.get_ndc_pos());
    }

    pub fn get_button(&mut self) -> &mut Option<Button> {
        return &mut self.button;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render(
        &mut self, 
        texture_manager: &mut TextureManager, 
        screen_data: &ScreenData, 
        play_view_data: &mut PlayViewData
    ) {
        self.panel.render(texture_manager);
        if let Some(button) = &mut self.button {
            button.render_button(texture_manager, screen_data);
        }


        if self.mouse_on(screen_data) {
            let inputs = screen_data.get_inputs();
            for input in inputs {
                match input {
                    crate::game_data::screen::input_data::Input::MouseButtonDown(mouse_button) => {
                        if *mouse_button == MouseButton::Right {
                            let mut button = play_view_data.get_button_selected_clone();
                            
                            // Set button rendering values to the same as slot
                            if let Some(button) = &mut button {
                                button.set_ndc(self.panel.get_ndc());
                                button.set_scale(self.panel.get_ndc_scale()[0]);
                            }

                            self.button = button;
                            println!("test");
                        }
                    },
                    _ => {

                    }
                }
            }
        }
    }


}