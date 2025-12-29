use miniquad::MouseButton;

use crate::game_data::{World, screen::{Button, ScreenData, main_menu::world_creation::WorldCreationMenu, ui_manager::BarButton}};

#[derive(PartialEq)]
enum MainMenuMenu {
    Main,
    WorldCreationMenu,
}

pub struct MainMenu {
    current_main_menu: MainMenuMenu,

    // Main Menu Data
    create_world_button: BarButton,
    world_creation_menu: WorldCreationMenu,

}

impl MainMenu {
    pub fn new() -> Self {
        let world_creation_menu = WorldCreationMenu::new();

        Self {
            current_main_menu: MainMenuMenu::Main,
            create_world_button: BarButton::new([0.0, 0.0], 0.1, 5),
            world_creation_menu,
        }
    }

    pub fn render_main_menu(&mut self, texture_manager: &mut crate::game_data::TextureManager) {
        // Render all buttons
        if self.current_main_menu == MainMenuMenu::Main {
            self.create_world_button.render_button(texture_manager);
        }
        else if self.current_main_menu == MainMenuMenu::WorldCreationMenu {
            self.world_creation_menu.render(texture_manager);
        }
    }

    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) {
        // Handle mouse motion for all buttons
        if self.current_main_menu == MainMenuMenu::Main {
            self.create_world_button.handle_mouse_motion_input(screen_data);
        }
        else if self.current_main_menu == MainMenuMenu::WorldCreationMenu {
            self.world_creation_menu.handle_mouse_motion_input(screen_data);
        }
    }

    pub fn handle_mouse_button_down(&mut self, screen_data: &mut ScreenData, mouse_button: MouseButton) {
        
        if self.current_main_menu == MainMenuMenu::Main {
            if mouse_button == MouseButton::Left {
                if self.create_world_button.is_mouse_on_button() {
                    self.current_main_menu = MainMenuMenu::WorldCreationMenu;
                }
            }
        }
        else if self.current_main_menu == MainMenuMenu::WorldCreationMenu {
            self.world_creation_menu.handle_mouse_button_down(screen_data, mouse_button);
        }

    }

}