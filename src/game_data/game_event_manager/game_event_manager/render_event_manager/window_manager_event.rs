use crate::game_data::{game_event_manager::{event_manager::Event, render_event_manager::render_event_manager::RenderEvent}, player_data::game_entity::game_entity_manager::GameEntityId, screen::widget::window_manager::{widget_window_manager::WidgetWindowManager, windows::{game_entity_win::GameEntityWindow, window_type::Window}}};

#[derive(Clone)]
pub enum WindowManagerEvent {
    OpenEntityWindow(GameEntityId),
}

impl WindowManagerEvent {
    pub fn wrap_into_event(self) -> Event {
        RenderEvent::WindowMangerEvent(self).wrap_into_event()
    }
    pub fn execute_event(self, window_manager: &mut WidgetWindowManager) {
        match self {
            WindowManagerEvent::OpenEntityWindow(game_entity_id) => {
                let window = GameEntityWindow::new(game_entity_id);
                window_manager.new_window(window.wrap_into_window_type(), "Game Entity Window");

            },
        }

    }
}