use std::process::id;

use miniquad::MouseButton;

use crate::game_data::{World, screen::{Button, ScreenData, screen_data::CurrentMenu, ui_elements::BarButton}, types::UITextures};


pub struct MainMenu {
    // Main Menu Data
    start_button: BarButton,
    options_button: BarButton,
    exit_button: BarButton,

}

impl MainMenu {
    pub fn new() -> Self {
        // Caclulate Centered Menu Button Cords
        let total_buttons = 3.0;
        let button_scale = 0.1;
        let button_length = 5;
        let button_y_spacing = button_scale * 1.5;

        let start_x_centering_offset = -(button_length as f32 * button_scale / 2.0);
        let mut button_y_start_offset = -(total_buttons * button_y_spacing) / 2.0;

        let start_button = BarButton::new_with_text(
            [start_x_centering_offset, button_y_start_offset],
            button_scale,
            button_length,
            "Start".to_string()
        );

        button_y_start_offset += button_y_spacing;
        let option_button = BarButton::new_with_text(
            [start_x_centering_offset, button_y_start_offset],
            button_scale,
            button_length,
            "Options".to_string()
        );

        button_y_start_offset += button_y_spacing;
        let exit_button = BarButton::new_with_text(
            [start_x_centering_offset, button_y_start_offset],
            button_scale,
            button_length,
            "Exit".to_string()
        );

        Self {
            start_button: start_button,
            options_button: option_button,
            exit_button: exit_button,
        }
    }

    pub fn render_main_menu(&mut self, texture_manager: &mut crate::game_data::TextureManager, screen_data: &ScreenData) {
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::FaceBackground, screen_data.get_viewport_uv());
        

        // Render all buttons
        self.start_button.render_button(texture_manager);
        self.options_button.render_button(texture_manager);
        self.exit_button.render_button(texture_manager);
        
    }

    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) {
        // Handle mouse motion for all buttons
        
        self.start_button.handle_mouse_motion_input(screen_data);
        self.options_button.handle_mouse_motion_input(screen_data);
        self.exit_button.handle_mouse_motion_input(screen_data);
        
    }

    pub fn handle_mouse_button_down(&mut self, screen_data: &mut ScreenData, mouse_button: MouseButton) {
        
        if mouse_button == MouseButton::Left {
            if self.start_button.is_mouse_on_button() {
                screen_data.set_current_menu(CurrentMenu::MainMenuWorldCreation);
            }

            if self.exit_button.is_mouse_on_button() {
                screen_data.quit();
            }
        }

    }

}