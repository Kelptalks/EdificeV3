use std::process::id;

use miniquad::MouseButton;

use crate::game_data::{World, screen::{Button, ScreenData, screen_data::CurrentMenu, ui_elements::BarButton}, types::UITextures};



/*
###############
## Main menu ##
###############
Main menu screen with navigation buttons for sandbox mode, levels, options, and exit.

Buttons are automatically centered and spaced using ndc cords

*/
pub struct MainMenu {
    // Main Menu Data
    button_sandbox: BarButton,
    button_levels: BarButton,
    button_options: BarButton,
    button_exit: BarButton,

}

impl MainMenu {
    pub fn new() -> Self {
        // Init bar buttons    
        let button_sandbox = BarButton::new_blank();
        let button_levels = BarButton::new_blank();
        let button_options = BarButton::new_blank();
        let button_exit = BarButton::new_blank();


        Self {
            button_sandbox,
            button_levels,
            button_options,
            button_exit,
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    /// Get an array of all the bar buttons within the main menu
    fn get_all_bar_buttons(&mut self) -> [&mut BarButton; 4] {
        return [
            &mut self.button_sandbox,
            &mut self.button_levels,
            &mut self.button_options,
            &mut self.button_exit,
        ]
    }

    //=====================================
    // Rendering
    //=====================================

    /// Resizes and sets cords of the buttons based off screen data
    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        let buttons = self.get_all_bar_buttons();
        
        // Caclulate Centered Menu Button Cords
        let button_scale = 0.1;
        let button_length = 5;
        let button_y_spacing = button_scale * 1.2;

        let start_x_centering_offset = -(button_length as f32 * button_scale / 2.0);
        let mut button_y_start_offset = -(buttons.len() as f32 * button_y_spacing) / 2.0;

        // Set locations
        for button in buttons {
            // Size
            button.set_scale(button_scale);
            button.set_length(button_length);
            
            // NDC
            button.set_ndc([start_x_centering_offset, button_y_start_offset]);
            button_y_start_offset += button_y_spacing;
        }

        // Set button text
        self.button_sandbox.set_text("Sandbox".to_string());
        self.button_levels.set_text("Levels".to_string());
        self.button_options.set_text("Options".to_string());
        self.button_exit.set_text("Exit".to_string());


    }

    /// Render the main menu to the
    pub fn render_main_menu(&mut self, texture_manager: &mut crate::game_data::TextureManager, screen_data: &ScreenData) {
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::FaceBackground, screen_data.get_viewport_uv());
        

        // Render all buttons
        let buttons = self.get_all_bar_buttons();
        for button in buttons {
            button.render_button(texture_manager);
        }
        
    }

    //=====================================
    // Controls
    //=====================================
    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) {
        // Handle mouse motion for all buttons
        let buttons = self.get_all_bar_buttons();
        for button in buttons {
            button.handle_mouse_motion_input(screen_data);
        }
    }

    pub fn handle_mouse_button_down(&mut self, screen_data: &mut ScreenData, mouse_button: MouseButton) {
        if mouse_button == MouseButton::Left {
            if self.button_sandbox.is_mouse_on_button() {
                screen_data.set_current_menu(CurrentMenu::MainMenuWorldCreation);
            }

            if self.button_exit.is_mouse_on_button() {
                screen_data.quit();
            }
        }

    }

}