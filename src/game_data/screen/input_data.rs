use core::f32;
use std::collections::HashSet;

use miniquad::{KeyCode, MouseButton};

pub enum Input {
    MouseMotion(f32, f32),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    MouseWheel(f32, f32),
    KeyDown(KeyCode),
    KeyUp(KeyCode),
}


pub struct MouseInputData {
    // Clicking
    left_mouse_button_down: bool,
    right_mouse_button_down: bool,
    
    left_mouse_button_up: bool,
    right_mouse_button_up: bool,

    // Mouse
    scrolled_up: bool,
    scrolled_down: bool,
}

impl MouseInputData {
    pub fn new() -> MouseInputData{
        MouseInputData {
            // Clicking
            left_mouse_button_down: false,
            right_mouse_button_down: false,

            left_mouse_button_up: false,
            right_mouse_button_up: false,

            // Scrolling
            scrolled_up: false,
            scrolled_down: false,
        }
    }

    //=====================================
    // Value checking
    //=====================================

    pub fn scrolled_up(&self) -> bool {
        return self.scrolled_up;
    }

    pub fn scrolled_down(&self) -> bool {
        return self.scrolled_down;
    }

    pub fn was_left_clicked(&self) -> bool {
        return self.left_mouse_button_down;
    }

    pub fn was_right_clicked(&self) -> bool {
        return self.right_mouse_button_down;
    }

    pub fn was_left_released(&self) -> bool {
        return self.left_mouse_button_up;
    }

    pub fn was_right_released(&self) -> bool {
        return self.right_mouse_button_up;
    }


    //=====================================
    // Value updating
    //=====================================

    pub fn update_scrolling_values(&mut self, x: f32, y: f32) {
        if y < 0.0 {
            self.scrolled_down = true;
        }
        else if y > 0.0 {
            self.scrolled_up = true;
        }
    }

    pub fn update_mouse_button_down(&mut self, mouse_button: MouseButton) {
        match mouse_button {
            MouseButton::Left => self.left_mouse_button_down = true,
            MouseButton::Right => self.right_mouse_button_down = true,
            _ => {},
        }
    }

    pub fn update_mouse_button_up(&mut self, mouse_button: MouseButton) {
        match mouse_button {
            MouseButton::Left => self.left_mouse_button_up = true,
            MouseButton::Right => self.right_mouse_button_up = true,
            _ => {},
        }
    }

    fn clear(&mut self) {
        self.scrolled_up = false;
        self.scrolled_down = false;

        self.left_mouse_button_down = false;
        self.right_mouse_button_down = false;

        self.left_mouse_button_up = false;
        self.right_mouse_button_up = false;

    }

}

pub struct InputManager {
    inputs: Vec<Input>,

    // Sorted Inputs
    key_down_inputs: HashSet<KeyCode>,

    mouse_input_data: MouseInputData

}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            inputs: Vec::new(),

            // Key
            key_down_inputs: HashSet::new(),

            // Mouse
            mouse_input_data : MouseInputData::new(),
        }
    }

    //=====================================
    // Inputs
    //=====================================

    pub fn get_inputs(&self) -> &Vec<Input> {
        return &self.inputs;
    }

    pub fn add_input(&mut self, input: Input) {
        
        match input {
            Input::KeyDown(key_code) => {
                self.key_down_inputs.insert(key_code);
            },
            Input::MouseWheel(x, y) => {
                self.mouse_input_data.update_scrolling_values(x, y)
            }
            Input::MouseButtonDown(mouse_button) => {
                self.mouse_input_data.update_mouse_button_down(mouse_button);
            }
            Input::MouseButtonUp(mouse_button) => {
                self.mouse_input_data.update_mouse_button_up(mouse_button);
            }
            _ => {

            }
        }

        self.inputs.push(input);

        
    }

    pub fn get_mouse_input_data(&self) -> &MouseInputData {
        return &self.mouse_input_data;
    }

    //=====================================
    // Key Checkers
    //=====================================

    pub fn was_key_code_pressed(&self, key_code: KeyCode) -> bool {
        return self.key_down_inputs.contains(&key_code);
    }

    //=====================================
    // Scroll Checkers
    //=====================================

    pub fn clear_inputs(&mut self) {
        self.inputs.clear();

        self.key_down_inputs.clear();


        self.mouse_input_data.clear();
    }
}
