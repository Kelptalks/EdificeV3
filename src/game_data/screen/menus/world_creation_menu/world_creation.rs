use miniquad::MouseButton;

use crate::game_data::{TextureManager, game_event_manager::{game_event_manager::GameEventManager, render_event_manager::render_event_manager::RenderEvent}, screen::{Button, ScreenData, menus::world_creation_menu::world_config::WorldConfig, render_centered_string_at_ndc, render_string, screen_data::CurrentMenu, ui_elements::{BarButton, stepper::Stepper}}, types::{FontType, UITextures}};


pub struct WorldCreationMenu {
    // Back button
    button_back: Button,
    
    // Create world
    button_create_world: BarButton,
    
    stepper_world_size: Stepper,
    
    // World folliage control


}

impl WorldCreationMenu {
    pub fn new() -> Self {
        let scale = 0.1;

        let world_size_gap = 0.35;
        
        let world_size_button_y = 0.0;
        let increase_world_cords = [world_size_gap, world_size_button_y];
        let decrease_world_cords = [-world_size_gap - scale, world_size_button_y];

        let create_world_button_length = 5;
        let bar_start_x = -scale * create_world_button_length as f32 / 2.0;
        let bar_start_y = 0.2;


        let mut create_world_button = BarButton::new([bar_start_x, bar_start_y], scale, create_world_button_length);
        create_world_button.set_text("Create World!".to_string());

        Self {
            button_back: Button::new_blank_with_texture(UITextures::ButtonLeftArrow),
            button_create_world: create_world_button,
            
            stepper_world_size: Stepper::new_blank(),
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


        // Stepper
        self.stepper_world_size.set_ndc([0.0, -0.5]);
        self.stepper_world_size.set_scale(buttonscale);
        self.stepper_world_size.set_text("World Size".to_string());
        self.stepper_world_size.set_value(250);
        self.stepper_world_size.set_max_value(1000);
        self.stepper_world_size.set_min_value(0);
        self.stepper_world_size.set_increment(25);


    }

    pub fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::MirrorBackground, screen_data.get_viewport_uv());
        
        // Render back button
        self.button_back.render_button(texture_manager, screen_data);

        self.stepper_world_size.render(texture_manager, screen_data);

        // Render create world button
        self.button_create_world.render_button(texture_manager);


    }

    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) {
        // Create world
        self.button_create_world.handle_mouse_motion_input(screen_data);
        
    }

    pub fn handle_mouse_button_down(&mut self, event_manager: &mut GameEventManager, screen_data: &mut ScreenData, mouse_button: MouseButton) {
        
        self.stepper_world_size.handle_mouse_button_down(mouse_button);
        if mouse_button == MouseButton::Left {
            if self.button_create_world.is_mouse_on_button() {
                event_manager.init_world(self.create_world_config());
                event_manager.add_render_event(RenderEvent::ChangeMenu(CurrentMenu::PlayView));
            }
            else if self.button_back.is_mouse_on_button() {
                event_manager.add_render_event(RenderEvent::ChangeMenu(CurrentMenu::MainMenu));
            }
        }
    }

    pub fn create_world_config(&self) -> WorldConfig {
        let mut world_config = WorldConfig::new();
        world_config.set_scale(self.stepper_world_size.get_value() as u32);
        return world_config;
    }

}
