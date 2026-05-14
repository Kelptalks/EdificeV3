use crate::game_data::{World, game_event_manager::event_manager::{Event, EventManager}, player_data::game_entity::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityId}, components::{block_component::WorldBlockComponent, entity_components::EntityComponent}, game_entity_manager::{GameEntity, GameEntityId}}, tik_manager::game_time::GameTime, tools::id_gen::IdGen, types::BlockTexture, world::world::WorldEvent};


static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct BlockEntityFlungle {
    pub id: u64,

    block: WorldBlockComponent,
}

impl BlockEntityFlungle {
    pub fn wrap_into_game_entity(self) -> GameEntity {
        BlockEntity::Flour(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityFlungle {
        let id = ID_GEN.new_id();
        let game_entity_id = BlockEntityId::Flungle(id).wrap_into_game_entity_id();

        let mut block = WorldBlockComponent::new(BlockTexture::Flungle, cords);
        block.init(game_entity_id, event_manager);

        BlockEntityFlungle {
            id,
            block,
        }
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        let self_world_cords = self.block.world_cords;

        let range = 3;

        if time.is_hour {
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

    pub fn get_components(self) -> Vec<EntityComponent> {
        let mut components = Vec::new();
        components.push(self.block.wrap_into_component());
        components
    }
}
