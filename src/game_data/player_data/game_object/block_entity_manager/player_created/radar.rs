use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_object::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityId}, game_object_manager::GameObject, id_gen::IdGen, traits::{game_object_trait_manager::GameObjectTrait, trait_block::{self, BlockTrait}, trait_powered::{self, PoweredTrait}, trait_vision::VisionTrait}}, tik_manager::game_time::GameTime, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();

#[derive(Clone)]
pub struct BlockEntityRadar {
    pub id: u64,
    
    pub trait_vision: VisionTrait,
    pub trait_block: BlockTrait,
    pub trait_powered: PoweredTrait,
}

impl BlockEntityRadar {

    pub fn wrap_into_game_object(self) -> GameObject {
        BlockEntity::Radar(self).wrap_into_game_object()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityRadar {
        let id = ID_GEN.new_id();
        let game_objcet_id: crate::game_data::player_data::game_object::game_object_manager::GameObjectId = BlockEntityId::RadarID(id).wrap_into_game_object_id();
        
        // Block
        let mut block_trait = 
            BlockTrait::new(BlockTexture::LBM, cords);
        block_trait.init(game_objcet_id, event_manager);
        

        // Vission
        let mut trait_vision = VisionTrait::new();
        let chunk_cords = World::world_cords_to_chunk_cords(cords);
        let range = 1;
        for x in -range..range {
            for y in -range..range {
                for z in -range..range {
                    let chunk_to_load = [
                        chunk_cords[0] + x,
                        chunk_cords[1] + y,
                        chunk_cords[2] + z,
                    ];
                    trait_vision.add_chunk_to_view(chunk_to_load);
                }
            }
        }

        // Powered
        let mut trait_powered = PoweredTrait::new();
        trait_powered.stored_power = 10000;

        BlockEntityRadar {
            id,

            trait_vision: trait_vision,
            trait_block: block_trait,
            trait_powered: trait_powered
        }
    }


    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        let power_required = self.trait_vision.chunks_loaded() as u32;
        
        // Tike vision
        if self.trait_powered.tik(power_required) {
            self.trait_vision.tik(event_manager);
        }

        // Only Consume power every 256 tiks
        if time.is_minute {
            self.trait_powered.consume_power(power_required);
        }
    }

    pub fn get_traits(self) -> Vec<GameObjectTrait> {
        let mut traits = Vec::new();

        traits.push(self.trait_block.wrap_into_trait());
        traits.push(self.trait_powered.wrap_into_trait());
        traits.push(self.trait_vision.wrap_into_trait());

        traits
    }
}

