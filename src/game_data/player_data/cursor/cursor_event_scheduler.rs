use crate::game_data::{game_event_manager::prelude::{Event, EventManager, PlayerDataEvent}, player_data::{cursor::cursor::Cursor, player_data::PlayerData}};

#[derive(Clone)]
pub enum CursorEvent {
    ModCords([i32; 3]),
    ModZoom(i32),
}

impl CursorEvent {
    pub fn wrap_into_event(self) -> Event {
        PlayerDataEvent::CursorEvent(self).wrap_into_event()
    }

    pub fn execute_cursor_event(&self, player_data: &mut PlayerData) {
        let cursor = player_data.get_mut_cursor();
        match self {
            CursorEvent::ModCords(cords) => cursor.mod_cords(cords),
            CursorEvent::ModZoom(amount) => cursor.mod_zoom(amount),
        }
    }
}

pub struct CursorEventScheduler { 
    cursor_clone: Cursor,
    events: Vec<CursorEvent>,
}


impl CursorEventScheduler {
    pub fn new(cursor: Cursor) -> CursorEventScheduler {
        CursorEventScheduler {
            cursor_clone: cursor,
            events: Vec::new(),
        }
    }

    pub fn get_cursor(&self) -> &Cursor {
        return &self.cursor_clone;
    }

    pub fn mod_cords(&mut self, cord_mod: [i32; 3]) {
        self.cursor_clone.mod_cords(&cord_mod);
        
        self.events.push(CursorEvent::ModCords(cord_mod));
    }


    pub fn zoom_in(&mut self) {
        self.cursor_clone.mod_zoom(&-1);

        self.events.push(CursorEvent::ModZoom(-1));
    }

    pub fn zoom_out(&mut self) {
        self.cursor_clone.mod_zoom(&1);

        self.events.push(CursorEvent::ModZoom(1));
    }

    pub fn schedul_events(mut self, event_manager: &mut EventManager) {
        while let Some(cursor_event) = self.events.pop() {
            event_manager.add_event(cursor_event.wrap_into_event());
        }
    }

    

    
}