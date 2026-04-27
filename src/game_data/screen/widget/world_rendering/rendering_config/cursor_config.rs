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
    cursor_cords: [i32 ;3],
    cursor_mode: CursorMode,


}

impl CursorConfig {

    pub fn new() -> CursorConfig {
        CursorConfig {
            cursor_cords: [0; 3],
            
            cursor_mode: CursorMode::Free(),
        }
    }

    //=====================================
    // Data Getters
    //=====================================

    pub fn get_cords(&self) -> [i32; 3] {
        return self.cursor_cords;
    }

    

    //=====================================
    // Cursor Movment Events
    //=====================================

    pub fn set_cursor_mode(&mut self, new_cursor_mode: CursorMode) {
        self.cursor_mode = new_cursor_mode;
    }


    pub fn get_move_cursor_event_with_shift_mod(&self, shift: [i32; 3]) -> Vec<Event> {
        let mut events = Vec::new();

        let cursor_mode = self.cursor_mode.clone();
        match cursor_mode {
            CursorMode::Free() => {
                
            },
            CursorMode::LockedToVar(location_ref) => {
               
               
               

            },
            CursorMode::Expand(location_ref) => {
                let var_type_borrowed = location_ref.borrow_mut();
                if let VarType::Game(GameVarType::Dynamic(DynamicVarType::Location(Some(location_ref)))) = &*var_type_borrowed {
                    let mut location_borrowed = location_ref.borrow_mut();

                    let cursor_cords = self.get_cords();
                    location_borrowed.get_mut_area().expand_to_fit_point(cursor_cords);
                } 
            },

            CursorMode::Shrink(location_ref) => {
                let var_type_borrowed = location_ref.borrow_mut();
                if let VarType::Game(GameVarType::Dynamic(DynamicVarType::Location(Some(location_ref)))) = &*var_type_borrowed {
                    let mut location_borrowed = location_ref.borrow_mut();

                    
                    let cursor_cords = self.get_cords();
                    location_borrowed.get_mut_area().shrink_to_avoid_point(cursor_cords);
                }
            }
        }


        return events;
    }

    // Add a shift event directly to game event manager
    pub fn add_move_cursor_event_with_shift_mod(&self, event_manager: &mut EventManager, shift: [i32; 3]) {
        event_manager.add_events(&self.get_move_cursor_event_with_shift_mod(shift));
    }

    
}


