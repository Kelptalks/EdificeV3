use miniquad::MouseButton;

use crate::game_data::{TextureManager, game_event_manager::{game_event_manager::GameEventManager, render_event_manager::render_event_manager::RenderEvent, world_event_manager::world_event_manager::WorldEvent}, level_manager::level, screen::{Button, ScreenData, render_centered_string_at_ndc, screen_data::CurrentMenu}, types::UITextures};



/*
#######################
## Level Select Menu ##
#######################
Menu for displaying and selecting diffrent levels

*/
pub struct LevelSelectMenu {
    // Buttons
    buttons_levels: Vec<Button>,
    button_back: Button,
}

impl LevelSelectMenu {
    pub fn new() -> LevelSelectMenu {
        LevelSelectMenu {
            buttons_levels: Vec::new(),
            button_back: Button::new_blank(UITextures::ButtonLeftArrow)
        }
    }

    //=====================================
    // Init
    //=====================================
    pub fn init(&mut self, event_manager: &mut GameEventManager) {
        let level_manager = event_manager.get_event_tools().get_level_manager();

        for i in 0..level_manager.get_total_levels() {
            self.buttons_levels.push(level_manager.get_level_at_index(i).get_level_select_button());
        }

    }

    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        // Viewport data
        let viewport_starting_ndc = screen_data.get_viewport_starting_ndc();
        let viewport_ending_ndc = screen_data.get_viewport_ending_ndc();

        // scaling
        let buttonscale = 0.1 * screen_data.get_ui_scale();
        let button_spacing = buttonscale / 2.0;
        let total_button_spacing = buttonscale + button_spacing;

        
        // Back button
        let back_button_ndc = [
            viewport_starting_ndc[0] + button_spacing,
            viewport_ending_ndc[1] - total_button_spacing,
        ];

        self.button_back.set_ndc(back_button_ndc);
        self.button_back.set_scale(buttonscale);
        self.button_back.set_text("Back".to_string());


        // Level select buttons
        let buttons_per_row = 12.0;
        let x_start_cor = -(buttons_per_row / 2.0) * total_button_spacing;
        let y_start_cor = viewport_starting_ndc[1] + (total_button_spacing * 2.0);

        let mut index = 0;
        for button in &mut self.buttons_levels {
            button.set_scale(buttonscale);
            let ndc = [
                x_start_cor + (total_button_spacing * (index % buttons_per_row as i32) as f32),
                y_start_cor + (total_button_spacing * (index / buttons_per_row as i32) as f32)
            ];
            
            button.set_ndc(ndc);
            index += 1;
        }
    }

    /// Render the main menu to the
    pub fn render_menu(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::MirrorBackground, screen_data.get_viewport_uv());
        

        // Render text
        let viewport_starting_ndc = screen_data.get_viewport_starting_ndc();
        render_centered_string_at_ndc(texture_manager, 
            "Level Select".to_string(), 
            crate::game_data::types::FontType::Basic, 
            0.1, 
            [0.0, viewport_starting_ndc[1] + 0.1]
        );



        self.button_back.render_button(texture_manager);
        for button in &mut self.buttons_levels {
            button.render_button(texture_manager);
        }
    }

    //=====================================
    // Controls
    //=====================================
    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) {
        self.button_back.handle_mouse_motion_input(screen_data);


        for button in &mut self.buttons_levels {
            button.handle_mouse_motion_input(screen_data);
        }
    }

    pub fn handle_mouse_button_down(&mut self, event_manager: &mut GameEventManager, screen_data: &mut ScreenData, mouse_button: MouseButton) {
        if mouse_button == MouseButton::Left {
            if self.button_back.is_mouse_on_button() {
                event_manager.add_render_event(RenderEvent::ChangeMenu(CurrentMenu::MainMenu));
            }

            let mut level_index = 0;
            for button in &mut self.buttons_levels {
                
                if button.is_mouse_on_button() {
                    event_manager.add_world_event(WorldEvent::GenLevel(level_index));
                    event_manager.add_render_event(RenderEvent::InitWorldRender(15));
                    event_manager.add_render_event(RenderEvent::ChangeMenu(CurrentMenu::Camera));
                }
                level_index += 1;
            }
        }
    }

}