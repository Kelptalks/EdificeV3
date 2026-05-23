use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_entity::{components::entity_components::{EntityComponent, EntityComponentEvent}, game_entity_manager::GameEntityId}, screen::widget::world_rendering::area_rendering_manager::ray_caster::ray_casting_config::Direction, tik_manager::game_time::GameTime, tools::cords_tool, types::BlockTexture, world::world::WorldEvent};

#[derive(Clone)]
pub struct BlockComponent {
    id: GameEntityId,

    pub block_type: BlockTexture,

    moved: Option<[i32; 3]>,

    animation_frame: usize,
    
    animation: Option<Vec<BlockTexture>>,

    pub world_cords: [i32; 3],
}

impl BlockComponent {
    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::Block(self)
    }

    pub fn new(block_type: BlockTexture, cords: [i32; 3], id: GameEntityId) -> BlockComponent {
        BlockComponent {
            id,
            block_type,

            moved: None,

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
        // Handle movment
        if let Some(moved) = self.moved {
            // Remove old block
            event_manager.add_world_event(
                WorldEvent::ModBlock(self.world_cords, BlockTexture::Air)
            );
            event_manager.add_world_event(
                WorldEvent::RemoveGameEntity(self.world_cords)
            );
            
            let new_cords = cords_tool::add_cords(self.world_cords, moved);

            // add new block
            self.world_cords = new_cords;
            event_manager.add_world_event(
                WorldEvent::ModBlock(self.world_cords, self.block_type)
            );
            event_manager.add_world_event(
                WorldEvent::AddGameEntity(self.world_cords, self.id)
            );
            

            self.moved = None;
        }
        
        // animations
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



#[derive(Clone)]
pub enum BlockComponentEvent {
    Move([i32; 3])
}

impl BlockComponentEvent {

    pub fn wrap_into_component_event(self) -> EntityComponentEvent {
        EntityComponentEvent::Block(self)
    }


    pub fn execute(self, block: &mut BlockComponent) {
        match self {
            BlockComponentEvent::Move(cord_mod) => {
                block.moved = Some(cord_mod);
            },
        }
    }
}
