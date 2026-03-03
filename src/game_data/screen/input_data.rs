use miniquad::{KeyCode, MouseButton};

pub enum Input {
    MouseMotion(f32, f32),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    MouseWheel(f32, f32),
    KeyDown(KeyCode),
    KeyUp(KeyCode),
}

pub struct InputData {
    inputs: Vec<Input>,
}

impl InputData {
    pub fn new() -> InputData {
        InputData {
            inputs: Vec::new(),
        }
    }

    //=====================================
    // Inputs
    //=====================================

    pub fn get_inputs(&self) -> &Vec<Input> {
        return &self.inputs;
    }

    pub fn add_input(&mut self, input: Input) {
        self.inputs.push(input);
    }

    pub fn clear_inputs(&mut self) {
        self.inputs.clear();
    }
}
