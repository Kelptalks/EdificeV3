use std::{cell::RefCell, rc::Rc};


use crate::game_data::{game_event_manager::prelude::{Event, GameEvent, PlayerDataEvent}, locations::world_area_side::WorldAreaSide, player_data::{drone_programming::{function::function::Function, var::{self, game_vars::{dynamic_var::{self, DynamicVar}, game_var_type::GameVar}, var_properties::VarPropRequest, var_type::Var}}, drones::{drone::Drone, drone_actions::{drone_actions::DroneAction, prim_actions::{drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}}}, locations::location::{self, WorldLocation}}};


//=====================================
// Helper
//=====================================

pub fn unpack_var_into_locaton(var_ref: &Rc<RefCell<Var>>) -> Option<Rc<RefCell<WorldLocation>>> {
    let borrow = var_ref.borrow();
    if let Var::Game(GameVar::Dynamic(DynamicVar::Location(location_option_ref))) = &*borrow {
        return location_option_ref.clone();
    }
    else {
        eprintln!("Cannot unpack to location on var type: {}", borrow.get_name());
        return None;
    }
}

//=====================================
// Var Event
//=====================================
#[derive(Clone)]
pub enum VarEvents {
    RequestEvent(Rc<RefCell<Var>>, VarPropRequest),
    
    LocationVarEvent(Rc<RefCell<Var>>, LocationVarEvent),
    DroneVarEvent(Rc<RefCell<Var>>, DroneVarEvent),
    DynamicVarEvent(Rc<RefCell<Var>>, DynamicVarEvent),


}


impl VarEvents {
    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::PlayerDataEvent(PlayerDataEvent::VarEvent(self)));
    }

    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        return vec![self.wrap_into_event()];
    }

    pub fn execute(&self) {
        match self {
            VarEvents::RequestEvent(ref_cell, request) => {
                ref_cell.borrow_mut().request_prop(request.clone());
            },
            VarEvents::LocationVarEvent(var_ref, location_var_event) => {
                let location_option_ref = unpack_var_into_locaton(var_ref);
                if let Some(location) = location_option_ref {
                    location_var_event.execute(&location);
                }
                else {
                    eprintln!("Cannot execute Location Var Event on NULL location");
                }
            }
            VarEvents::DynamicVarEvent(var_ref, dynamic_var_event) => {
                let mut borrow = var_ref.borrow_mut();
                if let Var::Game(GameVar::Dynamic(dynamic_var)) = &mut *borrow {
                    dynamic_var_event.execute(dynamic_var);
                }
                else {
                    eprintln!("Cannot execute Dymamic Var Event on var type: {}", borrow.get_name());
                }
            },
            VarEvents::DroneVarEvent(var_ref, drone_var_event) => {
                let borrow = var_ref.borrow();
                if let Var::Game(GameVar::Dynamic(DynamicVar::Drone(drone_option_ref))) = &*borrow {
                    if let Some(drone) = drone_option_ref {
                        drone_var_event.execute(drone);
                    }
                    else {
                        eprintln!("Cannot execute Location Var Event on NULL location");
                    }
                }
                else {
                    eprintln!("Cannot execute Location event on var type: {}", borrow.get_name());
                }
            },
        }
    }
}


//=====================================
// DynamicVarEvent
//=====================================
#[derive(Clone)]
pub enum DynamicVarEvent {
    Rename(Rc<RefCell<String>>),
}

impl DynamicVarEvent {
    pub fn execute(&self, dynamic_var: &mut DynamicVar) { 
        match self {
            DynamicVarEvent::Rename(string_ref) => {
                dynamic_var.rename(string_ref.borrow().clone());
            },
        }
    }
}


//=====================================
// LocationVarEvent
//=====================================
#[derive(Clone)]
pub enum LocationVarEvent {
    ExpandLocationSide(WorldAreaSide),
    ShrinkLocationSide(WorldAreaSide),
}

impl LocationVarEvent {
    pub fn execute(&self, location: &Rc<RefCell<WorldLocation>>) { 
        match self {
            LocationVarEvent::ExpandLocationSide(world_area_side) => {
                location.borrow_mut().get_mut_area().expand(world_area_side);
            },
            LocationVarEvent::ShrinkLocationSide(world_area_side) => {
                location.borrow_mut().get_mut_area().shrink(world_area_side);
            },
        }
    }
}

//=====================================
// DroneVarEvent
//=====================================

#[derive(Clone)]
pub enum DroneVarEvent {
    ExecuteActionEvent(Rc<RefCell<Function>>)
}

impl DroneVarEvent {
    pub fn execute(&self, drone: &Rc<RefCell<Drone>>) { 
        match self {
            DroneVarEvent::ExecuteActionEvent(function_ref) => {
                let constructed_action = function_ref.borrow_mut().into_drone_action();
                println!("Adding Action ({}) To Drone ({})", constructed_action.get_name(), drone.borrow().get_name());


                drone.borrow_mut().add_action(constructed_action);
            }
        }
    }
}