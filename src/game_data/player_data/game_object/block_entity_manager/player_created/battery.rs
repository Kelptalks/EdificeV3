use crate::game_data::{World, game_event_manager::event_manager::{Event, EventManager}, player_data::game_object::{block_entity_manager::{block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId}, player_created::radar::RadarEvent}, game_object_manager::GameObject, id_gen::IdGen, traits::{game_object_trait_manager::GameObjectTrait, trait_block::BlockTrait, trait_powered::{self, PoweredTrait, PoweredTraitEvent}}}, tik_manager::game_time::GameTime, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct BlockEntityBattery {
    pub id: u64,

    pub trait_block: BlockTrait,
    pub trait_powered: PoweredTrait,
}

impl BlockEntityBattery {
    pub fn wrap_into_game_object(self) -> GameObject {
        BlockEntity::Battery(self).wrap_into_game_object()
    }
    
    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityBattery {
        let id = ID_GEN.new_id();
        let game_objcet_id: crate::game_data::player_data::game_object::game_object_manager::GameObjectId = BlockEntityId::Battery(id).wrap_into_game_object_id();


        let mut trait_block = BlockTrait::new(BlockTexture::Battery1, cords);
        trait_block.give_animation(
            vec![BlockTexture::Battery1, BlockTexture::Battery2, BlockTexture::Battery3, BlockTexture::Battery4]
        );
        trait_block.init(game_objcet_id, event_manager);
        
        // Powered
        let mut trait_powered = PoweredTrait::new(game_objcet_id, cords);
        trait_powered.power_stored = 50000;


        BlockEntityBattery {
            id,
            
            trait_block,
            trait_powered,
        }
    }
    
    
    pub fn tik(&mut self, game_time: &GameTime, world: &World, event_manager: &mut EventManager) {
        // self.trait_block.tik(time, world, event_manager);
        
        // Alert to has power
        self.trait_powered.tik(game_time, world, event_manager);
    }
    
    pub fn get_traits(self) -> Vec<GameObjectTrait> {
        let mut traits = Vec::new();

        traits.push(self.trait_block.wrap_into_trait());
        traits.push(self.trait_powered.wrap_into_trait());

        traits
    }

}


/*
###################
## Battery Event ##
###################
Comments
*/

#[derive(Clone)]
pub enum BatteryEvent {
    PoweredTraitEvent(PoweredTraitEvent),
}

impl BatteryEvent {
    pub fn wrap_into_event(self, id: u64) -> Event {
        BlockEntityEvent::BatteryEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, radar: &mut BlockEntityBattery) {
        match self {

            BatteryEvent::PoweredTraitEvent(powered_trait_event) => {
                powered_trait_event.execute(&mut radar.trait_powered);
            },
        }
    }
}