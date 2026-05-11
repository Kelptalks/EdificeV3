use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_object::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityId}, game_object_manager::GameObject, id_gen::IdGen, traits::{trait_block::{self, BlockTrait}, trait_vision::VisionTrait}}, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();

#[derive(Clone)]
pub struct BlockEntityRadar {
    trait_vision: VisionTrait,
    trait_block: BlockTrait,
}

impl BlockEntityRadar {

    pub fn wrap_into_game_objcet(self) -> GameObject {
        BlockEntity::Radar(self).wrap_into_game_object()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityRadar {
        let id: crate::game_data::player_data::game_object::game_object_manager::GameObjectId = BlockEntityId::RadarID(ID_GEN.new_id()).wrap_into_game_object_id();
        
        // Block
        let mut block_trait = 
            BlockTrait::new(BlockTexture::LBM, cords);
        block_trait.init(id, event_manager);
        


        // Vission
        let mut trait_vision = VisionTrait::new();
        let chunk_cords = World::world_cords_to_chunk_cords(cords);
        let range = 3;
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

        

        BlockEntityRadar {
            trait_vision: trait_vision,
            trait_block: block_trait,
        }
    }


    pub fn tik(&mut self, time: u64, world: &World, event_manager: &mut EventManager) {
        self.trait_vision.tik(event_manager);
    }
}

