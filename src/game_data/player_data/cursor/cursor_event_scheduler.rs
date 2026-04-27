use crate::game_data::{game_event_manager::prelude::{Event, EventManager, PlayerDataEvent}, player_data::{cursor::cursor::Cursor, player_data::{PlayerData, ViewMode}}};

#[derive(Clone)]
pub enum CursorEvent {
    ModCords([i32; 3]),
    SetCords([i32; 3]),
    ModZoom(i32),
    SpawnDrone(),
}

impl CursorEvent {
    pub fn wrap_into_event(self) -> Event {
        PlayerDataEvent::CursorEvent(self).wrap_into_event()
    }

    pub fn execute_cursor_event(&self, player_data: &mut PlayerData) {
        
        
        match self {
            CursorEvent::ModCords(cords) => {
                let cursor = player_data.get_mut_cursor();
                cursor.mod_cords(cords)
            },
            CursorEvent::SetCords(cords) => {
                let cursor = player_data.get_mut_cursor();
                cursor.set_cords(cords)
            }
            CursorEvent::ModZoom(amount) => {
                let cursor = player_data.get_mut_cursor();
                cursor.mod_zoom(amount)
            },
            CursorEvent::SpawnDrone() => {
                let cords = player_data.get_cursor().get_cords();
                let id = player_data.get_mut_drone_manager().create_drone_at_cords(cords);

                player_data.set_view_mode(&Some(ViewMode::Drone(id)));
            }
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

    //=====================================
    // Event constructors
    //=====================================

    pub fn mod_cords(&mut self, cord_mod: [i32; 3]) {
        self.cursor_clone.mod_cords(&cord_mod);
        self.events.push(CursorEvent::ModCords(cord_mod));
    }

    pub fn set_cords(&mut self, new_cord: [i32; 3]) {
        self.cursor_clone.set_cords(&new_cord);

        self.events.push(CursorEvent::SetCords(new_cord));
    }

    pub fn zoom_in(&mut self) {
        self.cursor_clone.mod_zoom(&-1);
        self.events.push(CursorEvent::ModZoom(-1));
    }

    pub fn zoom_out(&mut self) {
        self.cursor_clone.mod_zoom(&1);
        self.events.push(CursorEvent::ModZoom(1));
    }

    pub fn spawn_drone(&mut self) {
        self.events.push(CursorEvent::SpawnDrone());
    }

    //=====================================
    // Event Sceduling
    //=====================================

    pub fn schedul_events(mut self, event_manager: &mut EventManager) {
        while let Some(cursor_event) = self.events.pop() {
            event_manager.add_event(cursor_event.wrap_into_event());
        }
    }

    

    
}