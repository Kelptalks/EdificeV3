use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::{game_event_manager::GameEventManager, player_data_event_manager::location_event::LocationEvent}, player_data_event_manager::{drone_event::DroneEvent, var_event_manager::var_events::VarEvents}, prelude::{Event, GameEvent}}, player_data::{drone_script::var::{game_vars::dynamic_var::DynamicVarType, var_type::VarType}, drones::drone::Drone, locations::{location::WorldLocation, location_config::WorldLocationConfig}, player_data::PlayerData}};

#[derive(Clone)]
pub enum PlayerDataEvent {
    CreateDrone([i32; 3]),
    DroneEvent(Rc<RefCell<Drone>>, DroneEvent),

    // Constructors
    CreateLocation(Rc<RefCell<WorldLocationConfig>>),
    CreateLocationInVar(Rc<RefCell<VarType>>, Rc<RefCell<WorldLocation>>),
    
    CreateDroneInVar(Rc<RefCell<VarType>>, Rc<RefCell<WorldLocation>>),
    
    LocationEvent(Rc<RefCell<WorldLocation>>, LocationEvent),

    VarEvent(VarEvents),
}


impl PlayerDataEvent {
    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::PlayerDataEvent(self));
    }

    pub fn execute_player_data_events(&self, event_tools: &mut GameEventManager, player_data: &mut PlayerData) {
        match self {
            PlayerDataEvent::CreateDrone(cords) => {
                player_data.get_mut_drone_manager().create_drone_at_cords(*cords);
            }
            PlayerDataEvent::DroneEvent(drone, drone_event) => {
                drone_event.execute(event_tools, drone);
            }
            PlayerDataEvent::LocationEvent(location_ref, location_event) => {
                location_event.execute(event_tools, location_ref.clone());
            },
            PlayerDataEvent::CreateLocation(location_config) => {
                location_config.borrow().create_location_in_manager(player_data.get_mut_location_manager());
            },
            PlayerDataEvent::CreateLocationInVar(var_ref, location_ref) => {
                let new_location = 
                    player_data.get_mut_location_manager().create_location(
                        "name".to_string(), 
                        *location_ref.borrow().get_area()
                    );
                *var_ref.borrow_mut() = DynamicVarType::Location(Some(new_location)).wrap_into_var();
            },
            PlayerDataEvent::VarEvent(var_event) => {
                var_event.execute();
            },
            PlayerDataEvent::CreateDroneInVar(var_ref, location_ref) => {
                let drone = player_data.get_mut_drone_manager().create_drone_at_cords(location_ref.borrow().get_area().get_point_1_cords());

                *var_ref.borrow_mut() = DynamicVarType::Drone(Some(drone)).wrap_into_var();
            },
        }
    }
}