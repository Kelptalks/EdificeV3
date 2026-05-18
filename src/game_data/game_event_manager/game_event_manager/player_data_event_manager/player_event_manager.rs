use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::{game_event_manager::GameEventManager, player_data_event_manager::location_event::LocationEvent}, player_data_event_manager::{drone_event::DroneEvent, var_event_manager::var_events::VarEvents}, prelude::{Event, GameEvent}}, player_data::{cursor::cursor_event_scheduler::CursorEvent, drone_script::var::{game_vars::dynamic_var::DynamicVarType, var_type::VarType}, drones::{drone::Drone, drone_event_scheduler::NewDroneEvent, drone_manager::DroneId}, game_entity::game_entity_manager::{GameEntity, GameEntityEvent}, locations::{location::WorldLocation, location_config::WorldLocationConfig}, player_data::PlayerData}, screen::widget::world_rendering::view_mode::ViewMode};

#[derive(Clone)]
pub enum PlayerDataEvent {
    
    // New
    SetViewMode(Option<ViewMode>),
    NewGameEntity(GameEntity),

    GameEntityEvent(GameEntityEvent),


    CursorEvent(CursorEvent),
    DroneEventNew(DroneId, NewDroneEvent),
    

    // Old

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

    pub fn execute_player_data_events(self, event_tools: &mut GameEventManager, player_data: &mut PlayerData) {
        match self {
            PlayerDataEvent::NewGameEntity(entity) => {
                player_data.new_game_entity(entity);
            }
            PlayerDataEvent::GameEntityEvent(game_entity_event) => {
                let game_entity_manager = &mut player_data.game_entity_manager;
                game_entity_event.execute(game_entity_manager);
            },

            PlayerDataEvent::SetViewMode(mode) => {
                player_data.set_view_mode(&mode);
            },


            PlayerDataEvent::CursorEvent(cursor_event) => {
                cursor_event.execute_cursor_event(player_data);
            },
            PlayerDataEvent::DroneEventNew(drone_id, drone_event) => {
                drone_event.execute(&drone_id, player_data);
            }
            

  
            PlayerDataEvent::DroneEvent(drone, drone_event) => {
                drone_event.execute(event_tools, &drone);
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
                *var_ref.borrow_mut() = DynamicVarType::Location(Some(new_location)).wrap_into_var_type();
            },
            PlayerDataEvent::VarEvent(var_event) => {
                var_event.execute();
            },
            PlayerDataEvent::CreateDroneInVar(_var_ref, location_ref) => {
                let _drone = player_data.get_mut_drone_manager().create_drone_at_cords(location_ref.borrow().get_area().get_point_1_cords());
            },
        }
    }
}