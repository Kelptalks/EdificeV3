use crate::game_data::screen::widget::{panel::panel::Panel, widget::Widget};

pub struct ControlFlow {
    panel: Panel,
}


impl Widget for ControlFlow {
    fn get_pos(&self) -> [f32; 4] {
        todo!()
    }

    fn get_scale(&self) -> [f32; 2] {
        todo!()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        todo!()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        todo!()
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        todo!()
    }

    fn size(&mut self) {
        todo!()
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        bounds: Option<[f32; 4]>
    ) {
        todo!()
    }
}