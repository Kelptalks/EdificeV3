use crate::game_data::{game_event_manager::{event_manager::Event, render_event_manager::render_event_manager::RenderEvent}, player_data::game_object::game_object_manager::GameObjectId, screen::widget::{panel::panel::Panel, widget::WidgetType, window_manager::{widget_window_manager::WidgetWindowManager, windows::{game_object_win::GameObjectWindow, window_type::{Window, WindowType}}}}};

#[derive(Clone)]
pub enum WindowManagerEvent {
    OpenObjectWindow(GameObjectId),
}

impl WindowManagerEvent {
    pub fn wrap_into_event(self) -> Event {
        RenderEvent::WindowMangerEvent(self).wrap_into_event()
    }
    pub fn execute_event(self, window_manager: &mut WidgetWindowManager) {
        match self {
            WindowManagerEvent::OpenObjectWindow(game_object_id) => {
                let window = GameObjectWindow::new(game_object_id);
                window_manager.new_window(window.wrap_into_window_type(), "Game Object Window");

            },
        }

    }
}