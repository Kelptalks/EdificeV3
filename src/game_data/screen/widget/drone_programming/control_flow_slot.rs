use crate::game_data::screen::widget::{panel::panel::Panel, widget::Widget, widget_properties::WidgetProperties};

pub struct ControlFlow {
    panel: Panel,
}


impl Widget for ControlFlow {
    fn get_widget_properties(&self) -> &WidgetProperties {
        self.panel.get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        self.panel.get_mut_widget_properties()
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
    ) {
        todo!()
    }
}
