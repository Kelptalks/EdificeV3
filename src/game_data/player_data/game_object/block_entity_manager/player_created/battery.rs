use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_object::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityId}, game_object_manager::GameObject, id_gen::IdGen, traits::{game_object_trait_manager::GameObjectTrait, trait_block::BlockTrait, trait_powered::{self, PoweredTrait}}}, tik_manager::game_time::GameTime, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();


const POWER_LINK_CHECKS: [[i32; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

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
        let mut trait_powered = PoweredTrait::new();
        trait_powered.power_stored = 50000;


        BlockEntityBattery {
            id,
            
            trait_block,
            trait_powered,
        }
    }
    
    
    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        // self.trait_block.tik(time, world, event_manager);
        
        // Alert to has power
        let cords = self.trait_block.world_cords;
        for relative_power_link_cords in POWER_LINK_CHECKS {
            let relative_cords = [
                cords[0] + relative_power_link_cords[0],
                cords[1] + relative_power_link_cords[1],
                cords[2] + relative_power_link_cords[2],
            ];
            if let Some(game_object_id) = world.get_game_object(relative_cords) {
                
            }
        }

    }
    
    pub fn get_traits(self) -> Vec<GameObjectTrait> {
        let mut traits = Vec::new();

        traits.push(self.trait_block.wrap_into_trait());
        traits.push(self.trait_powered.wrap_into_trait());

        traits
    }



}