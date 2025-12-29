use miniquad::MouseButton;

use crate::game_data::{screen::{Button, ScreenData, main_menu::world_config::WorldConfig, render_centered_string_at_ndi_cords, render_string, ui_manager::BarButton}, types::UITextures};

pub struct WorldCreationMenu {
    

    create_world_button: BarButton,
    
    // World size buttons
    increase_world_size_button: Button,
    decrease_world_size_button: Button,
    world_size: u32,

}

impl WorldCreationMenu {
    pub fn new() -> Self {
        let scale = 0.1;

        let world_size_gap = 0.5;
        
        let world_size_button_y = 0.0;
        let increase_world_cords = [world_size_gap, world_size_button_y];
        let decrease_world_cords = [-world_size_gap, world_size_button_y];

        let create_world_button_length = 5;
        let bar_start_x = -scale * create_world_button_length as f32 / 2.0;
        let bar_start_y = 0.2;


        Self {
            create_world_button: BarButton::new([bar_start_x, bar_start_y], scale, create_world_button_length),
            increase_world_size_button: Button::new(increase_world_cords, scale, UITextures::ButtonRightArrow),
            decrease_world_size_button: Button::new(decrease_world_cords, scale, UITextures::ButtonLeftArrow),
            world_size: 200,
        }
    }

    pub fn render(&mut self, texture_manager: &mut crate::game_data::TextureManager) {
        // Render create world button
        self.create_world_button.render_button(texture_manager);
        render_centered_string_at_ndi_cords(
            texture_manager,
            format!("Create World"),
            "Basic".to_string(),
            0.02,
            [self.create_world_button.get_pos()[0], self.create_world_button.get_pos()[1]]
        );

        // Render size customize
        self.increase_world_size_button.render_button(texture_manager);
        self.decrease_world_size_button.render_button(texture_manager);
        render_centered_string_at_ndi_cords(texture_manager,
            format!("World Size: {}", self.world_size),
            "Basic".to_string(),
            0.02,
            [0.0, 0.0]
        );
    }

    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) {
        // Handle mouse motion for all buttons
        self.create_world_button.handle_mouse_motion_input(screen_data);
        self.increase_world_size_button.handle_mouse_motion_input(screen_data);
        self.decrease_world_size_button.handle_mouse_motion_input(screen_data);
    }

    pub fn handle_mouse_button_down(&mut self, screen_data: &mut ScreenData, mouse_button: MouseButton) {
        if mouse_button == MouseButton::Left {
            if self.create_world_button.is_mouse_on_button() {
                screen_data.set_world_config(self.create_world_config());
                screen_data.set_current_menu(crate::game_data::screen::screen_data::CurrentMenu::Camera);
            }
            else if self.increase_world_size_button.is_mouse_on_button() {
                self.world_size += 10;
            }
            else if self.decrease_world_size_button.is_mouse_on_button() {
                if self.world_size > 0 {
                    self.world_size -= 10;
                }
            }
        }
    }

    pub fn create_world_config(&self) -> WorldConfig {
        let mut world_config = WorldConfig::new();
        world_config.set_scale(self.world_size);
        return world_config;
    }

}
