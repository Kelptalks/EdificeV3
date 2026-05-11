use crate::game_data::{World, game_event_manager::{event_manager::{Event, EventManager}, world_event_manager::world_event_manager::WorldEvent}, player_data::game_object::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityId}, game_object_manager::{GameObject, GameObjectId}, id_gen::IdGen, traits::trait_block::BlockTrait}, types::BlockTexture};


static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct BlockEntityFlour {
    trait_block: BlockTrait,
}

impl BlockEntityFlour {
    pub fn wrap_into_game_objcet(self) -> GameObject {
        BlockEntity::Flour(self).wrap_into_game_object()
    }
    
    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityFlour {
        let id = BlockEntityId::FlourID(ID_GEN.new_id()).wrap_into_game_object_id();
        
        // Block
        let mut block_trait = 
            BlockTrait::new(BlockTexture::flungle, cords);
        block_trait.init(id, event_manager);


        BlockEntityFlour {
            trait_block: block_trait,
        }
    }

    pub fn tik(&mut self, time: u64, world: &World, event_manager: &mut EventManager) {
        let self_world_cords = self.trait_block.world_cords;

        let range = 3;

        if time % 256 == 0 {
            // Scan around
            for x in -range..range {
                for y in -range..range {
                    for z in -1..1 {
                        let mut cords = [
                            self_world_cords[0] + x,
                            self_world_cords[1] + y,
                            self_world_cords[2] + z,
                        ];
                        let block = world.get_world_value_as_block(cords);

                        if block == BlockTexture::Grass {
                            cords[2] += 1;

                            let block_above = world.get_world_value_as_block(cords);
                            if block_above == BlockTexture::Air {
                                event_manager.add_event(
                                    WorldEvent::ModBlock(cords, BlockTexture::yellow_flowers)
                                    .wrap_into_event()
                                );
                                return;
                            }
                        }
                    }
                    
                }
            }

        }
    }
}