use crate::game_data::{World, game_event_manager::{event_manager::EventManager, render_event_manager::window_manager_event::WindowManagerEvent}, player_data::{drones::drone_manager::DroneId, game_object::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityId, BlockEntityManager}, traits::game_object_trait_manager::GameObjectTrait}, nature_manager::nature_manager::NatureObject, player_data::PlayerData}, screen::widget::widget::WidgetType, tik_manager::game_time::GameTime};


#[derive(Clone, Copy)]
pub enum GameObjectId {
    Drone(DroneId),
    BlockEntity(BlockEntityId)
}

#[derive(Clone)]
pub enum GameObject {
    BlockEntity(BlockEntity),
}

impl GameObject {
    pub fn get_traits(self) -> Vec<GameObjectTrait> {
        match self {
            GameObject::BlockEntity(block_entity) => block_entity.get_traits(),
        }
    }
}

pub struct GameObjectManager {
    block_entity_manager: BlockEntityManager,
}

impl GameObjectManager {
    pub fn new() -> GameObjectManager {
        GameObjectManager {
            block_entity_manager: BlockEntityManager::new(),
        }
    }


    pub fn new_game_oject(&mut self, new_object: GameObject) {
        match new_object {
            GameObject::BlockEntity(block_entity) => {
                self.block_entity_manager.add_object(block_entity);
            },
        }
    }

    pub fn clone_game_object(&self, object_id: GameObjectId) -> Option<GameObject> {
        match object_id {
            GameObjectId::BlockEntity(block_entity_id) => {
                if let Some(block_entity) = self.block_entity_manager.get_block_entity(block_entity_id) {
                    Some(block_entity.wrap_into_game_object())
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

    pub fn tik_game_objects(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        self.block_entity_manager.tik(time, world, event_manager);
    }


    pub fn open_object_window(&self, event_manager: &mut EventManager, id: GameObjectId) {
        event_manager.add_event(WindowManagerEvent::OpenObjectWindow(id).wrap_into_event());
        println!("opening window")
    }

}