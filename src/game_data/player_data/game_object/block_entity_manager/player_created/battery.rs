use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_object::traits::{game_object_trait_manager::GameObjectTrait, trait_block::BlockTrait, trait_powered::PoweredTrait}, tik_manager::game_time::GameTime};

#[derive(Clone)]
pub struct BlockEntityBattery {
    pub id: u64,

    pub trait_block: BlockTrait,
    pub trait_powered: PoweredTrait,
}

impl BlockEntityBattery {
    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {

    }
    
    pub fn get_traits(self) -> Vec<GameObjectTrait> {
        let mut traits = Vec::new();

        traits.push(self.trait_block.wrap_into_trait());
        traits.push(self.trait_powered.wrap_into_trait());

        traits
    }


    

}