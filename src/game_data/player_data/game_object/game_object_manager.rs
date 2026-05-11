use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::{drones::drone_manager::DroneId, game_object::block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityId, BlockEntityManager}, nature_manager::nature_manager::NatureObject, player_data::PlayerData}};


#[derive(Clone)]
pub enum GameObjectId {
    Drone(DroneId),
    BlockEntity(BlockEntityId)
}

#[derive(Clone)]
pub enum GameObject {
    BlockEntity(BlockEntity),
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

    pub fn tik_game_objects(&mut self, time: u64, world: &World, event_manager: &mut EventManager) {
        self.block_entity_manager.tik(time, world, event_manager);
    }

}