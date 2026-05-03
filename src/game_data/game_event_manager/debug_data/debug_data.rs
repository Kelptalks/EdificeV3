use crate::game_data::game_event_manager::debug_data::{rendering_debug_data::RenderingDebugData, window_debug_data::WindowDebugData};


#[macro_export]
macro_rules! debug_fields {
    ($self:ident, $($field:ident),*) => {
        vec![$( format!("{}: {}", stringify!($field), $self.$field) ),*]
    };
}


pub struct DebugData {
    window_debug_data: WindowDebugData,
    rendering_debug_data: RenderingDebugData,
}

impl DebugData {
    pub fn new() -> DebugData {
        DebugData {
            window_debug_data: WindowDebugData::new(),
            rendering_debug_data: RenderingDebugData::new(),
        }
    }


    pub fn get_window_debug_data(&mut self) -> &mut WindowDebugData {
        &mut self.window_debug_data
    }

    pub fn get_rendering_debug_data(&mut self) -> &mut RenderingDebugData {
        &mut self.rendering_debug_data
    }
}