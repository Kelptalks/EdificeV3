use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_object::{block_entity_manager::{natural::flour::BlockEntityFlour, player_created::radar::BlockEntityRadar}, game_object_manager::{GameObject, GameObjectId}}};

#[derive(Clone)]
pub enum BlockEntity {
    // player
    Radar(BlockEntityRadar),

    // natural
    Flour(BlockEntityFlour)
}

#[derive(Clone)]
pub enum BlockEntityId {
    // Player
    RadarID(u64),

    // Natural
    FlourID(u64),
}

impl BlockEntityId {
    pub fn wrap_into_game_object_id(self) -> GameObjectId {
        GameObjectId::BlockEntity(self)
    }
}

impl BlockEntity {
    pub fn wrap_into_game_object(self) -> GameObject {
        GameObject::BlockEntity(self)
    }

    

    pub fn tik(&mut self, time: u64, world: &World, event_manager: &mut EventManager) {
        match self {
            BlockEntity::Radar(block_entity_radar) => {
                block_entity_radar.tik(time, world, event_manager);
            },
            BlockEntity::Flour(block_entity_flour) => {
                block_entity_flour.tik(time, world, event_manager)
            },
        }
    } 
}

pub struct BlockEntityManager {
    block_entitys: Vec<BlockEntity>,
}

impl BlockEntityManager {
    pub fn new() -> BlockEntityManager {
        BlockEntityManager {
            block_entitys: Vec::new(),
        }
    }

    pub fn add_object(&mut self, new_block_entity: BlockEntity) {
        self.block_entitys.push(new_block_entity);
    }

    pub fn tik(&mut self, time: u64, world: &World, event_manager: &mut EventManager) {
        for entity in &mut self.block_entitys {
            entity.tik(time, world, event_manager);
        }
    }
}