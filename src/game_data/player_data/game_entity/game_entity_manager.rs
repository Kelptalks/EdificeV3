use core::fmt;

use crate::game_data::{World, game_event_manager::{event_manager::{Event, EventManager}, game_event_manager::game_event_manager::GameEventManager, player_data_event_manager::player_event_manager::PlayerDataEvent, render_event_manager::window_manager_event::WindowManagerEvent}, player_data::{drones::drone_manager::DroneId, game_entity::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId, BlockEntityManager}, components::entity_components::{EntityComponent, EntityComponentEvent}, dynamic_entity_manager::dynamic_entity_manager::{DynamicEntity, DynamicEntityId, DynamicEntityManager}, lair_block_entity_manager::lair_block_entity_manager::{LairBlockEntity, LairBlockEntityEvent, LairBlockEntityId, LairBlockEntityManager}}}, screen::{ScreenData, widget::{widget::WidgetType, world_rendering::{area_rendering_manager::block_lair_manager::lair_block::LairBlockMod, world_view_data::WorldViewData}}}, tik_manager::game_time::GameTime};


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
    LairBlockEntity(LairBlockEntityId),
}

impl GameEntityId {
    pub fn get_component_event(&self, component_event: EntityComponentEvent) -> Option<Event> {
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
            GameEntityId::Drone(_drone_id)               => write!(f, "Drone"),
            GameEntityId::BlockEntity(id)                => write!(f, "{}", id),
            GameEntityId::DynamicEntity(id)              => write!(f, "{}", id),
            GameEntityId::LairBlockEntity(id)            => write!(f, "{}", id),
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
    LairBlockEntity(LairBlockEntity),
}

impl GameEntity {
    pub fn get_components(self) -> Vec<EntityComponent> {
        match self {
            GameEntity::BlockEntity(block_entity)   => block_entity.get_components(),
            GameEntity::DynamicEntity(dynamic_entity) => dynamic_entity.get_components(),
            GameEntity::LairBlockEntity(_)          => vec![],
        }
    }

    pub fn get_window(self) -> Option<WidgetType> {
        match self {
            GameEntity::BlockEntity(block_entity)   => block_entity.get_window(),
            GameEntity::DynamicEntity(dynamic_entity) => dynamic_entity.get_window(),
            GameEntity::LairBlockEntity(_)          => None,
        }
    }

    /// Runs this entity's spectate-view logic for the current frame and
    /// returns the overlay widget to display while spectating it.
    pub fn play_view(&self, world_view_data: &WorldViewData, screen_data: &ScreenData, world: &World, event_manager: &mut EventManager) -> WidgetType {
        match self {
            GameEntity::BlockEntity(block_entity)   => block_entity.play_view(world_view_data, screen_data, world, event_manager),
            GameEntity::DynamicEntity(dynamic_entity) => dynamic_entity.play_view(world_view_data, screen_data, world, event_manager),
            GameEntity::LairBlockEntity(_)          => crate::game_data::screen::widget::panel::panel::Panel::new_blank().wrap_into_widget(),
        }
    }
}

pub struct GameEntityManager {
    block_entity_manager: BlockEntityManager,
    dynamic_entity_manager: DynamicEntityManager,
    lair_block_entity_manager: LairBlockEntityManager,
}

impl GameEntityManager {
    pub fn new() -> GameEntityManager {
        GameEntityManager {
            block_entity_manager: BlockEntityManager::new(),
            dynamic_entity_manager: DynamicEntityManager::new(),
            lair_block_entity_manager: LairBlockEntityManager::new(),
        }
    }

    pub fn new_game_entity(&mut self, new_entity: GameEntity) {
        match new_entity {
            GameEntity::BlockEntity(block_entity) => {
                self.block_entity_manager.add_entity(block_entity);
            },
            GameEntity::DynamicEntity(dynamic_entity) => {
                self.dynamic_entity_manager.add_entity(dynamic_entity)
            },
            GameEntity::LairBlockEntity(lair_entity) => {
                self.lair_block_entity_manager.add_entity(lair_entity);
            }
        }
    }

    pub fn get_lair_block_mod(&self, cords: &[i32; 3], entity_id: LairBlockEntityId) -> Option<LairBlockMod> {
        self.lair_block_entity_manager.get_mod(cords, entity_id)
    }

    pub fn clone_game_entity(&self, entity_id: GameEntityId) -> Option<GameEntity> {
        match entity_id {
            GameEntityId::BlockEntity(block_entity_id) => {
                self.block_entity_manager.clone_block_entity(block_entity_id).map(|e| e.wrap_into_game_entity())
            },
            GameEntityId::DynamicEntity(dynamic_entity_id) => {
                self.dynamic_entity_manager.clone_entity(dynamic_entity_id).map(|e| e.wrap_into_game_entity())
            },
            _ => None,
        }
    }

    pub fn tik_game_entities(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        self.block_entity_manager.tik(time, world, event_manager);
        self.dynamic_entity_manager.tik(time, world, event_manager);
        self.lair_block_entity_manager.tik(time, event_manager);
    }

    pub fn iter_dynamic_world_pos_and_texture(&self) -> impl Iterator<Item = ([f32; 3], crate::game_data::texture_manager::texture::Texture)> + '_ {
        self.dynamic_entity_manager.iter_world_pos_and_texture()
    }

    pub fn open_entity_window(&self, event_manager: &mut EventManager, id: GameEntityId) {
        event_manager.add_event(WindowManagerEvent::OpenEntityWindow(id).wrap_into_event());
        println!("opening window")
    }
}


#[derive(Clone)]
pub enum GameEntityEvent {
    BlockEntityEvent(BlockEntityEvent),
    LairBlockEntityEvent(LairBlockEntityEvent),
}

impl GameEntityEvent {
    pub fn wrap_into_event(self) -> Event {
        PlayerDataEvent::GameEntityEvent(self).wrap_into_event()
    }

    pub fn execute(self, game_entity_manager: &mut GameEntityManager, gem: &mut GameEventManager) {
        match self {
            GameEntityEvent::BlockEntityEvent(block_entity_event) => {
                block_entity_event.execute(&mut game_entity_manager.block_entity_manager);
            },
            GameEntityEvent::LairBlockEntityEvent(lair_event) => {
                lair_event.execute(&mut game_entity_manager.lair_block_entity_manager, gem);
            },
        }
    }
}
