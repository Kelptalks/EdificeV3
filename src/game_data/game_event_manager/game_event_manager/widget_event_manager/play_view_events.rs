use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, GameEvent, GameEventManager, WidgetEvent}, screen::widget::{prelude::play_world_view_config::PlayViewRenderingConfig, world_rendering::rendering_config::cursor_config::CursorMode}};

#[derive(Clone)]
pub enum PlayViewEvent {
    // Cursor
    ShiftCursor(Rc<RefCell<PlayViewRenderingConfig>>, [i32; 3]),
    SetCursorMode(Rc<RefCell<PlayViewRenderingConfig>>, CursorMode),
}

impl PlayViewEvent {
    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::WidgetEvent(WidgetEvent::PlayViewEvent(self)));
    }

    pub fn execute_widget_event(&self, event_tools: &mut GameEventManager) -> Vec<Event> {
        let mut events = Vec::new();
        match self {
            PlayViewEvent::ShiftCursor(rendering_config, shift) =>{
                events.append(&mut rendering_config.borrow().get_cursor_config().get_move_cursor_event_with_shift_mod(*shift));
            },
            PlayViewEvent::SetCursorMode(rendering_config, cursor_mode) => {
                rendering_config.borrow_mut().get_mut_cursor_config().set_cursor_mode(cursor_mode.clone());
            }
        }

        return events;
    }
}