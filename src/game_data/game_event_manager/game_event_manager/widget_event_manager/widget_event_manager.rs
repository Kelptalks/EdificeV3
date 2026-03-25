use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::game_event_manager::GameEventManager, prelude::{Event, GameEvent}, widget_event_manager::prim_events::prim_event_manager::PrimEvent}, types::BlockTexture};

#[derive(Clone)]
pub enum WidgetEvent {
    ToggleBoolEvent(Rc<RefCell<bool>>),
    
    // Prim
    PrimEvent(PrimEvent),
    SetUsizeEvent(Rc<RefCell<usize>>, usize),
    
    
    Modf32Event(Rc<RefCell<f32>>, f32),
    Modi32Event(Rc<RefCell<i32>>, i32),

    // Game type
    SetBlockRef(Rc<RefCell<BlockTexture>>, Rc<RefCell<BlockTexture>>)
}


impl WidgetEvent {
    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        return vec![Event::GameEvent(GameEvent::WidgetEvent(self))];
    } 
    
    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::WidgetEvent(self));
    }

    pub fn execute_widget_event(&self, event_tools: &mut GameEventManager) {
        match self {
            WidgetEvent::PrimEvent(event) => {
                event.execute();
            }
            WidgetEvent::ToggleBoolEvent(toggle_button_event) => {
                let current_state = *toggle_button_event.borrow();
                *toggle_button_event.borrow_mut() = !current_state;
            },
            WidgetEvent::SetUsizeEvent(ref_cell, value) => {
                *ref_cell.borrow_mut() = *value;
            },
            WidgetEvent::Modf32Event(ref_cell, value) => {
                *ref_cell.borrow_mut() += *value;
            },
            WidgetEvent::Modi32Event(ref_cell, value) => {
                *ref_cell.borrow_mut() += *value;
            }
            WidgetEvent::SetBlockRef(current_block, new_block) => {
                *current_block.borrow_mut() = *new_block.borrow();
            },
        }
    }
}