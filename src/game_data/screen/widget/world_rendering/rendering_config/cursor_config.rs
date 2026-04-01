use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, EventManager, LocationEvent, PlayerDataEvent}, locations::{world_area::WorldArea, world_area_side::WorldAreaSide, world_point::WorldPoint}, player_data::{drone_programming::var::{game_vars::{game_var_type::GameVar}, var_type::Var}, locations::location::WorldLocation, player_data::PlayerData}};

#[derive(Clone)]
pub enum CursorMode {
    Free(), // Do not restric movment of cursor
    LockedToVar(Rc<RefCell<Var>>), // Prevent the cursor from leaveing the locations bounds

}

pub struct CursorConfig {
    location: Rc<RefCell<WorldLocation>>,
    cursor_mode: CursorMode,


}

impl CursorConfig {

    pub fn new(player_data: &mut PlayerData) -> CursorConfig {
        CursorConfig {
            location: player_data.get_mut_location_manager().create_location("Cursor".to_string(), WorldArea::new_blank()),
            
            cursor_mode: CursorMode::Free(),
        }
    }

    //=====================================
    // Data Getters
    //=====================================

    pub fn get_location_ref(&self) -> &Rc<RefCell<WorldLocation>> {
        return &self.location;
    }

    fn get_point(&self) -> WorldPoint {
        return self.location.borrow().get_area().get_world_point(0);
    }

    

    //=====================================
    // Cursor Movment Events
    //=====================================

    pub fn set_cursor_mode(&mut self, new_cursor_mode: CursorMode) {
        self.cursor_mode = new_cursor_mode;
    }

    // Construct a location shifting event based off the shift cords
    fn construct_shift_event(&self, shift: [i32; 3]) -> Event {
        let shift_event = 
            PlayerDataEvent::LocationEvent(
                self.location.clone(), 
                LocationEvent::ShiftLocation(shift)
            ).wrap_into_event();
    
        return shift_event;
    }
    
    // Construct shifting events but exclude specific sides
    fn construct_shift_event_excluding_sides(&self, excluded_sides: Vec<WorldAreaSide>, shift: [i32; 3]) -> Vec<Event> {
        let mut events = Vec::new();
        
        // Do not create shift event if side is excluded
        let shift_mod_side = WorldAreaSide::area_shift_mod_to_side(shift);
        if let Some(side) = shift_mod_side {
            for exluded_side in excluded_sides {
                if side == exluded_side {
                    return events;
                }
            }
        }
        events.push(self.construct_shift_event(shift));
        return events;
    }

    pub fn get_move_cursor_event_with_shift_mod(&self, shift: [i32; 3]) -> Vec<Event> {
        let mut events = Vec::new();

        let cursor_mode = self.cursor_mode.clone();
        match cursor_mode {
            CursorMode::Free() => {
                events.push(self.construct_shift_event(shift));
            },
            CursorMode::LockedToVar(var_ref) => {
                let borrowed = var_ref.borrow();
                if let Var::Game(GameVar::Dynamic(dynamic_var)) = &*borrowed {
                    if let Some(area) = dynamic_var.get_area() {
                        let sides_of_cursor_on_location = WorldAreaSide::get_sides_of_point_in_area(
                            &area, &self.get_point()
                        );
                        events.append(&mut self.construct_shift_event_excluding_sides(sides_of_cursor_on_location, shift));
                    }
                }
                else {
                    println!("Cannot Lock cursor to var({})", var_ref.borrow().get_name());
                }
            },
        }

        return events;
    }

    // Add a shift event directly to game event manager
    pub fn add_move_cursor_event_with_shift_mod(&self, event_manager: &mut EventManager, shift: [i32; 3]) {
        event_manager.add_events(&self.get_move_cursor_event_with_shift_mod(shift));
    }

    
}


