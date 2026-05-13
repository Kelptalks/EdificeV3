use crate::game_data::{World, game_event_manager::{event_manager::EventManager, world_event_manager::world_event_manager::WorldEvent}, player_data::game_entity::{components::entity_components::EntityComponent, game_entity_manager::GameEntityId}, tik_manager::game_time::GameTime, types::BlockTexture};

#[derive(Clone)]
pub struct BlockComponent {
    pub block_type: BlockTexture,

    animation_frame: usize,
    animation: Option<Vec<BlockTexture>>,

    pub world_cords: [i32; 3],
}

impl BlockComponent {
    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::Block(self)
    }

    pub fn new(block_type: BlockTexture, cords: [i32; 3]) -> BlockComponent {
        BlockComponent {
            block_type,

            animation_frame: 0,
            animation: None,

            world_cords: cords
        }
    }

    pub fn give_animation(&mut self, animation: Vec<BlockTexture>) {
        self.animation_frame = 0;
        self.animation = Some(animation);
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        if time.is_second {
            if let Some(frames) = &self.animation {
                if self.animation_frame >= frames.len() {
                    self.animation_frame = 0;
                }

                event_manager.add_world_event(
                    WorldEvent::ModBlock(self.world_cords, frames[self.animation_frame])
                );

                self.animation_frame += 1;
            }
        }
    }

    pub fn init(&mut self, id: GameEntityId, event_manager: &mut EventManager) {
        event_manager.add_world_event(
            WorldEvent::ModBlock(self.world_cords, self.block_type)
        );
        event_manager.add_world_event(
            WorldEvent::AddGameEntity(self.world_cords, id)
        );
    }
}
