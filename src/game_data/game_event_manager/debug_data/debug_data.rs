use crate::game_data::game_event_manager::debug_data::window_debug_data::WindowDebugData;

pub struct DebugData {
    window_debug_data: Vec<WindowDebugData>,

}

impl DebugData {
    pub fn new() -> DebugData {
        DebugData {
            window_debug_data: Vec::new(),
        }
    }


    pub fn add_window_debug_data(&mut self, data: WindowDebugData) {
        self.window_debug_data.push(data);
    }

    pub fn get_window_debug_data(&self) -> &Vec<WindowDebugData> {
        return &self.window_debug_data;
    }

    pub fn clear_window_debug_data(&mut self) {
        self.window_debug_data.clear();
    }
}