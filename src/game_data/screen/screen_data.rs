use miniquad::{GlContext, MouseButton, RenderingBackend};

use crate::game_data::screen::input_data::{Input, InputManager};

#[derive(Copy, Clone, PartialEq)]
pub enum CurrentMenu {
    MainMenu,
    WorldCreationMenu,
    SettingsMenu,
    MapView,
    PlayView,
}

pub struct ScreenData {
    input_manager: InputManager,
    
    // Menu
    current_menu: CurrentMenu,
    debug_visible: bool,

    // Screen Data
    screen_rez: [f32; 2],
    viewport_rez: [f32; 2],
    viewport_offset: [f32; 2],
    ui_scale: f32,

    // Mouse Cords
    mouse_pixel_cords: [i32; 2],
    mouse_ndc_cords: [f32; 2],

    // Last mouse cords
    last_mouse_ndc_cords: [f32; 2],

    // Button held states
    middle_mouse_held: bool,
    left_mouse_held: bool,
    right_mouse_held: bool,
    was_left_released: bool,
    was_right_released: bool,
    was_left_pressed: bool,
    was_right_pressed: bool,


    starting_mouse_ndc_on_middle_down: [f32; 2],
    ending_mouse_ndc_on_middle_down: [f32; 2],

    // Quit
    quit_game : bool,
}

impl ScreenData {
    pub fn new() -> ScreenData {
        ScreenData {
            // Input Data, 
            input_manager: InputManager::new(),

            // Menu
            current_menu: CurrentMenu::MainMenu,
            debug_visible: false,

            // Screen Data
            screen_rez: [0.0, 0.0],
            viewport_rez: [0.0, 0.0],
            viewport_offset: [0.0, 0.0],
            ui_scale: 1.0,

            // Mouse Cords
            mouse_pixel_cords: [0, 0],
            mouse_ndc_cords: [0.0, 0.0],

            // Last mouse cords
            last_mouse_ndc_cords: [0.0, 0.0],

            // Mouse
            middle_mouse_held: false,
            left_mouse_held: false,
            right_mouse_held: false,
            was_left_released: false,
            was_right_released: false,
            was_left_pressed: false,
            was_right_pressed: false,

            starting_mouse_ndc_on_middle_down: [0.0, 0.0],
            ending_mouse_ndc_on_middle_down: [0.0, 0.0],

            // Quit
            quit_game: false,
        }
    }

    //=====================================
    // Inputs
    //=====================================
    
    pub fn add_input(&mut self, input: Input) {
        self.input_manager.add_input(input);
    }

    pub fn update_inputs(&mut self) {
        for input in self.input_manager.get_inputs() {
            match input {
                Input::MouseButtonDown(mouse_button) => {
                    if *mouse_button == MouseButton::Left {
                        self.left_mouse_held = true;
                        self.was_left_pressed = true;
                    } 
                    else if *mouse_button == MouseButton::Right {
                        self.right_mouse_held = true;
                        self.was_right_pressed = true;
                    }
                },
                Input::MouseButtonUp(mouse_button) => {
                    if *mouse_button == MouseButton::Left {
                        self.left_mouse_held = false;
                        self.was_left_released = true;
                        
                    } 
                    else if *mouse_button == MouseButton::Right {
                        self.right_mouse_held = false;
                        self.was_right_released = true;
                    }
                },
                _ => {

                }
            }
        }
    }

    pub fn clear_inputs(&mut self) {
        self.input_manager.clear_inputs();
        self.was_left_released = false;
        self.was_right_released = false;
        self.was_left_pressed = false;
        self.was_right_pressed = false;
    }

    pub fn get_inputs(&self) -> &Vec<Input> {
       return &self.input_manager.get_inputs();
    } 

    pub fn get_input_manager(&self) -> &InputManager {
        return &self.input_manager;
    }

    //=====================================
    // Menu
    //=====================================

    pub fn set_current_menu(&mut self, menu: CurrentMenu) {
        self.current_menu = menu;
    }

    pub fn get_current_menu(&self) -> CurrentMenu {
        return self.current_menu;
    }

    pub fn set_debug_visibility(&mut self, visibility: bool) {
        self.debug_visible = visibility;
    }
    pub fn get_debug_visiblity(&self) -> bool {
        return self.debug_visible;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn apply_port(&self, ctx : &mut GlContext) {
        ctx.apply_viewport(
            self.viewport_offset[0] as i32, 
            self.viewport_offset[1] as i32, 
            self.viewport_rez[0] as i32, 
            self.viewport_rez[1] as i32, 
        );
    }

    pub fn set_screen_rez(&mut self, screen_rez: [f32; 2], ctx : &mut GlContext) {
        // Calculate and setup viewport and set the correct values 
        let viewport_size: f32;

        // Set up square viewport to allow for consistant rendering
        if screen_rez[0] > screen_rez[1] {
            viewport_size = screen_rez[0];
            self.viewport_offset = [0.0, -(viewport_size - screen_rez[1]) / 2.0];
        } else {
            viewport_size = screen_rez[1];
            self.viewport_offset = [-(viewport_size - screen_rez[0]) / 2.0, 0.0];
        }

        self.screen_rez = screen_rez;
        self.viewport_rez = [viewport_size, viewport_size];

        // set the viewport using ctx
        ctx.apply_viewport(
            self.viewport_offset[0] as i32, 
            self.viewport_offset[1] as i32, 
            self.viewport_rez[0] as i32, 
            self.viewport_rez[1] as i32, 
        );
    }

    pub fn get_viewport_uv(&self) -> [f32; 4] {
        let starting_cords = self.pixel_cords_to_ndc_cords([0.0, 0.0]);
        let ending_cords = self.pixel_cords_to_ndc_cords(self.get_screen_rez());

        return [starting_cords[0], starting_cords[1], ending_cords[0], ending_cords[1]];
    }

    pub fn get_viewport_starting_ndc(&self) -> [f32; 2] {
        return self.pixel_cords_to_ndc_cords([0.0, 0.0]);
    }

    pub fn get_viewport_ending_ndc(&self) -> [f32; 2] {
        return self.pixel_cords_to_ndc_cords(self.get_screen_rez());
    }


    pub fn pixel_cords_to_ndc_cords(&self, pixel_cords: [f32; 2]) -> [f32; 2] {
        let centered_pixel_cords = [
            pixel_cords[0] - (self.screen_rez[0] / 2.0),
            pixel_cords[1] - (self.screen_rez[1] / 2.0),
        ];

        let ndc_cords = [
            (centered_pixel_cords[0]) / (self.viewport_rez[0] / 2.0),
            (centered_pixel_cords[1]) / (self.viewport_rez[1] / 2.0)
        ];

        return ndc_cords;
    }

    pub fn get_screen_rez(&self) -> [f32; 2] {
        return self.screen_rez;
    }

    pub fn get_viewport_rez(&self) -> [f32; 2] {
        return self.viewport_rez;
    }

    pub fn get_viewport_offset(&self) -> [f32; 2] {
        return self.viewport_offset;
    }

    pub fn get_ui_scale(&self) -> f32 {
        return self.ui_scale;
    }

    //=====================================
    // Mouse Cords
    //=====================================

    pub fn get_mouse_pixel_cords(&self) -> [i32; 2] {
        return self.mouse_pixel_cords;
    }

    pub fn set_mouse_pixel_cords(&mut self, cords: [i32; 2]) {
        self.mouse_pixel_cords = cords;
        self.last_mouse_ndc_cords = self.mouse_ndc_cords;
        let x = cords[0] as f32;
        let y = cords[1] as f32;
        let y_offset = (self.viewport_rez[1] - self.screen_rez[1]) / 2.0;
        self.mouse_ndc_cords = [
            (x / self.viewport_rez[0]) * 2.0 - 1.0,
            ((y + y_offset) / self.viewport_rez[1]) * 2.0 - 1.0,
        ];
    }

    pub fn get_mouse_ndc(&self) -> [f32; 2] {
        return self.mouse_ndc_cords;
    }

    pub fn mouse_on_ndc_pos(&self, pos: [f32; 4]) -> bool {
        return 
            self.mouse_ndc_cords[0] >= pos[0] && 
            self.mouse_ndc_cords[1] >= pos[1] &&
            self.mouse_ndc_cords[0] <= pos[2] &&
            self.mouse_ndc_cords[1] <= pos[3];
    }

    //=====================================
    // Mouse Holding
    //=====================================

    pub fn is_middle_mouse_held(&self) -> bool {
        return self.middle_mouse_held;
    }

    pub fn set_middle_mouse_held(&mut self, held: bool) {
        self.middle_mouse_held = held;
        if held {
            self.starting_mouse_ndc_on_middle_down = self.get_mouse_ndc();
        }
        else {
            self.ending_mouse_ndc_on_middle_down = self.get_mouse_ndc();
        }
    }

    pub fn get_total_middle_drag_ndc_from_start(&self) -> [f32; 2] {
        return [
            self.starting_mouse_ndc_on_middle_down[0] + self.mouse_ndc_cords[0],
            self.starting_mouse_ndc_on_middle_down[1] + self.mouse_ndc_cords[0],
        ];
    }

    pub fn get_change_in_mouse_ndc(&self) -> [f32; 2] {
        return [
            self.last_mouse_ndc_cords[0] - self.mouse_ndc_cords[0],
            self.last_mouse_ndc_cords[1] - self.mouse_ndc_cords[1],
        ]
    }

    pub fn is_left_mouse_held(&self) -> bool {
        return self.left_mouse_held;
    }

    pub fn is_right_mouse_held(&self) -> bool {
        return self.right_mouse_held;
    }

    pub fn was_left_released(&self) -> bool {
        return self.was_left_released;
    }

    pub fn was_right_released(&self) -> bool {
        return self.was_right_released;
    }

    pub fn was_left_pressed(&self) -> bool {
        return self.was_left_pressed;
    }

    pub fn was_right_pressed(&self) -> bool {
        return self.was_right_pressed;
    }

    pub fn get_mouse_centered_texture_rendering_pos(&self, scale: [f32; 2]) -> [f32; 4] {
        let half_scale = [scale[0] / 2.0, scale[1] / 2.0];
        return [
            self.mouse_ndc_cords[0] - half_scale[0],
            self.mouse_ndc_cords[1] - half_scale[1],
            self.mouse_ndc_cords[0] + half_scale[0],
            self.mouse_ndc_cords[1] + half_scale[1],
        ];
    }

    //=====================================
    // Quitting
    //=====================================

    pub fn quit(&mut self) {
        self.quit_game = true;
    }

    pub fn should_quit(&self) -> bool {
        return self.quit_game;
    }

}