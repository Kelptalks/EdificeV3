use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    game_event_manager::prelude::{
        Event, EventManager, LocationEvent, PlayerDataEvent
    }, 
    locations::{world_area::WorldArea, world_area_side::WorldAreaSide, world_point::WorldPoint}, 
    player_data::{
        drone_script::var::{game_vars::{dynamic_var::{DynamicVarType}, game_var_type::GameVarType}, var_type::VarType}, 
        locations::location::WorldLocation, player_data::PlayerData
    }
};

#[derive(Clone)]
pub enum CursorMode {
    Free(), // Do not restric movment of cursor
    
    LockedToVar(Rc<RefCell<VarType>>), // Prevent the cursor from leaveing the locations bounds
    Expand(Rc<RefCell<VarType>>),
    Shrink(Rc<RefCell<VarType>>),
}

pub struct CursorConfig {
    location: Rc<RefCell<WorldLocation>>,
    cursor_mode: CursorMode,


}

impl CursorConfig {

    pub fn new(player_data_ref: &Rc<RefCell<PlayerData>>) -> CursorConfig {
        
        let location = 
            player_data_ref.borrow_mut().
            get_mut_location_manager().
            create_location("Cursor".to_string(), WorldArea::new_blank());
        
        CursorConfig {
            location: location,
            
            cursor_mode: CursorMode::Free(),
        }
    }

    //=====================================
    // Data Getters
    //=====================================

    pub fn get_location_ref(&self) -> &Rc<RefCell<WorldLocation>> {
        return &self.location;
    }

    pub fn get_point(&self) -> WorldPoint {
        return self.location.borrow().get_area().get_world_point(0);
    }

    pub fn get_cords(&self) -> [i32; 3] {
        return self.location.borrow().get_area().get_point_1_cords();
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
            CursorMode::LockedToVar(location_ref) => {
               
               
               
               
               
                let location_borrowed = location_ref.borrow_mut();
    
                
                if let VarType::Game(GameVarType::Dynamic(dynamic_var)) = &*location_borrowed {
                    if let Some(area) = dynamic_var.get_area() {
                        

                        

                        // Create events 
                        let sides_of_cursor_on_location = WorldAreaSide::get_sides_of_point_in_area(
                            &area, &self.get_point()
                        );
                        events.append(
                            &mut self.construct_shift_event_excluding_sides(
                                sides_of_cursor_on_location.clone(), shift
                            )
                        );



                        // Move cursor back into bounds if it somehow escapes
                        if !area.cords_in_area(self.get_point().cords) {
                            for side in sides_of_cursor_on_location {
                                let dist_from_side = side.dist_from_side(&area, self.get_point().cords);
                                events.push(self.construct_shift_event(side.get_area_shift_mod(dist_from_side)));
                            }
                        }
                    }
                }
                else {
                    println!("Cannot Lock cursor to var({})", location_ref.borrow().get_name());
                }
            },
            CursorMode::Expand(location_ref) => {
                let var_type_borrowed = location_ref.borrow_mut();
                if let VarType::Game(GameVarType::Dynamic(DynamicVarType::Location(Some(location_ref)))) = &*var_type_borrowed {
                    let mut location_borrowed = location_ref.borrow_mut();

                    let cursor_cords = self.get_cords();
                    location_borrowed.get_mut_area().expand_to_fit_point(cursor_cords);
                }
                events.push(self.construct_shift_event(shift));   
            },

            CursorMode::Shrink(location_ref) => {
                let var_type_borrowed = location_ref.borrow_mut();
                if let VarType::Game(GameVarType::Dynamic(DynamicVarType::Location(Some(location_ref)))) = &*var_type_borrowed {
                    let mut location_borrowed = location_ref.borrow_mut();

                    
                    let cursor_cords = self.get_cords();
                    location_borrowed.get_mut_area().shrink_to_avoid_point(cursor_cords);
                }
                events.push(self.construct_shift_event(shift));
            }
        }


        return events;
    }

    // Add a shift event directly to game event manager
    pub fn add_move_cursor_event_with_shift_mod(&self, event_manager: &mut EventManager, shift: [i32; 3]) {
        event_manager.add_events(&self.get_move_cursor_event_with_shift_mod(shift));
    }

    
}


