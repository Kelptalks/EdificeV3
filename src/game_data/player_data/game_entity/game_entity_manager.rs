use core::fmt;

use crate::game_data::{World, game_event_manager::{event_manager::{Event, EventManager}, player_data_event_manager::player_event_manager::PlayerDataEvent, render_event_manager::window_manager_event::WindowManagerEvent}, player_data::{drones::drone_manager::DroneId, game_entity::{block_entity_manager::{self, block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId, BlockEntityManager}}, components::{entity_components::EntityComponent, powered_component::PoweredComponentEvent}, dynamic_entity_manager::{self, dynamic_entity_manager::{DynamicEntity, DynamicEntityId, DynamicEntityManager}}}, player_data::PlayerData}, screen::widget::widget::WidgetType, tik_manager::game_time::GameTime};


/*
####################
## Game Entity ID ##
####################
*/

#[derive(Clone, Copy)]
pub enum GameEntityId {
    Drone(DroneId),
    BlockEntity(BlockEntityId),
    DynamicEntity(DynamicEntityId),
}

impl GameEntityId {
    pub fn get_component_event(&self, component_event: PoweredComponentEvent) -> Option<Event> {
        match self {
            GameEntityId::BlockEntity(block_entity_id) => {
                block_entity_id.get_component_event(component_event)
            },
            _ => {
                None
            }
        }
    }
}

impl fmt::Display for GameEntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameEntityId::Drone(_drone_id) => write!(f, "Drone"),
            GameEntityId::BlockEntity(block_entity_id) => write!(f, "{}", block_entity_id),
            GameEntityId::DynamicEntity(dynamic_entity_id) => write!(f, "{}", dynamic_entity_id),
        }
    }
}

/*
#################
## Game Entity ##
#################
*/

#[derive(Clone)]
pub enum GameEntity {
    BlockEntity(BlockEntity),
    DynamicEntity(DynamicEntity),
}

impl GameEntity {
    pub fn get_components(self) -> Vec<EntityComponent> {
        match self {
            GameEntity::BlockEntity(block_entity) => block_entity.get_components(),
            GameEntity::DynamicEntity(dynamic_entity) => dynamic_entity.get_components(),
        }
    }

    pub fn get_window(self) -> Option<WidgetType> {
        match self {
            GameEntity::BlockEntity(block_entity) => {
                block_entity.get_window()
            },
            GameEntity::DynamicEntity(dynamic_entity) => {
                dynamic_entity.get_window()
            },
        }
    }
}

pub struct GameEntityManager {
    block_entity_manager: BlockEntityManager,
    dynamic_entity_manager: DynamicEntityManager,
}

impl GameEntityManager {
    pub fn new() -> GameEntityManager {
        GameEntityManager {
            block_entity_manager: BlockEntityManager::new(),
            dynamic_entity_manager: DynamicEntityManager::new(),
        }
    }

    pub fn new_game_entity(&mut self, new_entity: GameEntity) {
        match new_entity {
            GameEntity::BlockEntity(block_entity) => {
                self.block_entity_manager.add_entity(block_entity);
            },
            GameEntity::DynamicEntity(dynamic_entity) => {
                self.dynamic_entity_manager.add_entity(dynamic_entity)
            }
        }
    }

    pub fn clone_game_entity(&self, entity_id: GameEntityId) -> Option<GameEntity> {
        match entity_id {
            GameEntityId::BlockEntity(block_entity_id) => {
                if let Some(block_entity) = self.block_entity_manager.clone_block_entity(block_entity_id) {
                    Some(block_entity.wrap_into_game_entity())
                }
                else {
                    None
                }
            },
            _ => {
                None
            }
        }
    }

    pub fn tik_game_entities(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        self.block_entity_manager.tik(time, world, event_manager);
    }

    pub fn open_entity_window(&self, event_manager: &mut EventManager, id: GameEntityId) {
        event_manager.add_event(WindowManagerEvent::OpenEntityWindow(id).wrap_into_event());
        println!("opening window")
    }
}


#[derive(Clone)]
pub enum GameEntityEvent {
    BlockEntityEvent(BlockEntityEvent)
}

impl GameEntityEvent {
    pub fn wrap_into_event(self) -> Event {
        PlayerDataEvent::GameEntityEvent(self).wrap_into_event()
    }

    pub fn execute(self, game_entity_manager: &mut GameEntityManager) {
        match self {
            GameEntityEvent::BlockEntityEvent(block_entity_event) => {
                block_entity_event.execute(&mut game_entity_manager.block_entity_manager);
            },
        }
    }
}
