use std::collections::{HashMap, HashSet};

use miniquad::{KeyCode, MouseButton};
use rand::distr::Map;

pub enum Input {
    MouseMotion(f32, f32),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    MouseWheel(f32, f32),
    KeyDown(KeyCode),
    KeyUp(KeyCode),
}

pub struct InputManager {
    inputs: Vec<Input>,

    // Sorted Inputs
    key_down_inputs: HashSet<KeyCode>,

}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            inputs: Vec::new(),

            key_down_inputs: HashSet::new(),
        }
    }

    //=====================================
    // Inputs
    //=====================================

    pub fn get_inputs(&self) -> &Vec<Input> {
        return &self.inputs;
    }

    pub fn was_key_code_pressed(&self, key_code: KeyCode) -> bool {
        return self.key_down_inputs.contains(&key_code);
    }

    pub fn add_input(&mut self, input: Input) {
        
        match input {
            Input::KeyDown(key_code) => {
                self.key_down_inputs.insert(key_code);
            },
            _ => {

            }
        }

        self.inputs.push(input);

        
    }

    pub fn clear_inputs(&mut self) {
        self.inputs.clear();


        self.key_down_inputs.clear();
    }
}
