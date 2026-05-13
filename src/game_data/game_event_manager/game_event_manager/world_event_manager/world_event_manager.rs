use std::{cell::RefCell, rc::Rc};


use crate::game_data::{World, game_event_manager::{game_event_manager::{game_event_manager::GameEventManager, render_event_manager::render_event_manager::RenderEvent}, prelude::{Event, GameEvent}, world_event_manager::chunk_event::WorldChunkEvent}, player_data::{game_entity::game_entity_manager::GameEntityId, locations::location::WorldLocation}, types::BlockTexture, world_chunk::WorldChunk};

/*
#################
## World Event ##
#################
Enum and logic for executing events pertaining 
to modifications to the world
*/

#[derive(Clone)]
pub enum WorldEvent {
    // Direct
    Clear,
    ChunkEvent(WorldChunkEvent),

    // Modifcation
    ModBlock([i32; 3], BlockTexture),                                   // Params: (Block Cords, Block Type) | Modify a block in the world
    AddGameEntity([i32; 3], GameEntityId),
    
    FillLocation(Rc<RefCell<WorldLocation>>, Rc<RefCell<BlockTexture>>) // Params: (Block Cords, Block Type) | Fill a location with PlaceBlock Events
}

impl WorldEvent {

    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::WorldEvent(self));
    }

    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        return vec![self.wrap_into_event()];
    }

    //=====================================
    // Execution
    //=====================================
    pub fn execute_world_event(self, world: &mut World, event_manager: &mut GameEventManager) {
        match self {
            WorldEvent::Clear => {
                world.clear();
            },

            WorldEvent::ChunkEvent(chunk_event) => {
                chunk_event.execute_chunk_event(world, event_manager);
            }
            WorldEvent::ModBlock(cords, block_type) => {
                world.set_world_value( block_type.id_as_u16(), cords);
                event_manager.add_render_event(RenderEvent::ReRenderBlock(cords));
            }
            WorldEvent::AddGameEntity(world_cords, game_entity_id) => {
                if let Some(chunk) = world.get_mut_chunk_at_world_cords(world_cords) {
                    chunk.game_entities.insert(world_cords, game_entity_id);
                }
            },

            WorldEvent::FillLocation(world_location, block_texture) => {
                event_manager.add_world_events(world_location.borrow().get_area().get_fill_area_events(*block_texture.borrow()));
            },
            
        }
    }
}
