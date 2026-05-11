use crate::game_data::{game_event_manager::debug_data::{self, rendering_debug_data::RenderingDebugData, window_debug_data::WindowDebugData, world_debug_data::WorldDebugData}, screen::widget::widget_properties::WidgetId};


#[macro_export]
macro_rules! debug_fields {
    ($self:ident, $($field:ident),*) => {
        vec![$( format!("{}: {}", stringify!($field), $self.$field) ),*]
    };
}


pub struct DebugData {
    pub debug_data_tabs_names: Vec<String>,
    pub debug_data: Vec<Vec<String>>,
    pub debug_data_widget_ids: Vec<WidgetId>,
}

impl DebugData {
    pub fn new() -> DebugData {

        let mut debug_data_tabs_names = Vec::new();
        let mut debug_data = Vec::new();

        debug_data_tabs_names.push("window_debug_data".to_string());
        debug_data.push(Vec::new());

        debug_data_tabs_names.push("rendering_debug_data".to_string());
        debug_data.push(Vec::new());
        
        debug_data_tabs_names.push("world_debug_data".to_string());
        debug_data.push(Vec::new());

        debug_data_tabs_names.push("tik_debug_data".to_string());
        debug_data.push(Vec::new());

        DebugData {

            debug_data_tabs_names,
            debug_data: debug_data,
            debug_data_widget_ids: Vec::new()
        }
    }


    pub fn get_data(&self, index: usize) -> &Vec<String> {
        if index < self.debug_data.len() {
            &self.debug_data[index]
        }
        else {
            &self.debug_data[0]
        }
    }

    pub fn clear_window_data(&mut self) {
        self.debug_data[0].clear();
    }
    pub fn add_window_data(&mut self, string: String) {
        self.debug_data[0].push(string);
    }

    pub fn clear_rendering_data(&mut self) {
        self.debug_data[1].clear();
    }
    pub fn add_rendering_data(&mut self, string: String) {
        self.debug_data[1].push(string);
    }

    pub fn clear_world_data(&mut self) {
        self.debug_data[2].clear();
    }
    pub fn add_world_data(&mut self, string: String) {
        self.debug_data[2].push(string);
    }

    pub fn clear_tik_data(&mut self) {
        self.debug_data[3].clear();
    }
    pub fn add_to_tik_data(&mut self, string: String) {
        self.debug_data[3].push(string);
    }

}