use crate::game_data::{World, game_event_manager::{event_manager::{self, Event, EventManager}, world_event_manager::world_event_manager::WorldEvent}, player_data::game_object::{game_object_manager::GameObjectId, traits::game_object_trait_manager::GameObjectTrait}, types::BlockTexture};

#[derive(Clone)]
pub struct BlockTrait {
    pub block_type: BlockTexture,
    pub world_cords: [i32; 3],       
}

impl BlockTrait {
    pub fn wrap_into_trait(self) -> GameObjectTrait {
        GameObjectTrait::BlockTrait(self)
    }

    pub fn new(block_type: BlockTexture, cords: [i32; 3]) -> BlockTrait {
        BlockTrait {
            block_type: block_type,
            world_cords: cords
        }
    }


    pub fn tik(&mut self, time: u64, world: &World, event_manager: &mut EventManager) {

    }

    pub fn init(&mut self, id: GameObjectId, event_manager: &mut EventManager) {
        event_manager.add_world_event(
            WorldEvent::ModBlock(self.world_cords, self.block_type)
        );
        event_manager.add_world_event(
            WorldEvent::AddGameObjcet(self.world_cords, id)
        );
    }

}
