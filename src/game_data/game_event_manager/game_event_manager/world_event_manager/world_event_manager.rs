use std::{cell::RefCell, rc::Rc};


use crate::game_data::{World, game_event_manager::{game_event_manager::{game_event_manager::GameEventManager, render_event_manager::render_event_manager::RenderEvent}, prelude::{Event, GameEvent}}, player_data::locations::location::WorldLocation, types::BlockTexture};

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
    Clear,                              // No Data                      // Params: () | Clear the world
    GenWorld(),              // World Config                            // Params: () | Generate the world
    
    // Modifcation
    GenLevel(u32),                                                      // Params: (Level Id) | Generate a level with id
    PlaceBlock([i32; 3], BlockTexture),                                 // Params: (Block Cords, Block Type) | Modifys a block and also attempts to create entitys if block is of a cirtain type
    ModBlock([i32; 3], BlockTexture),                                   // Params: (Block Cords, Block Type) | Modify a block in the world
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
    pub fn execute_world_event(&self, world: &mut World, event_data: &mut GameEventManager) {
        match self {
            WorldEvent::Clear => {
                world.clear();
            },
            WorldEvent::GenWorld() => {
                let render_range = event_data.get_mut_world_gen_manager().get_world_config().get_chunk_rendering_range();
                event_data.get_mut_world_gen_manager().generate_area(world);
    
            },
            WorldEvent::GenLevel(level) => {
                event_data.get_level_manager().get_level_at_index(level.clone() as usize).gen_level(world);
            },
            WorldEvent::PlaceBlock(cords, block_type) => {
                world.set_world_value(block_type.id_as_u16(), *cords);

                // If block is an entity add it to the world
                if block_type.is_block_entity() {
                    let entity_type = block_type.to_block_entity_type();
                    event_data.add_game_event(entity_type.to_creation_event(*cords));
                }

                event_data.add_render_event(RenderEvent::ReRenderBlock(*cords));
            }
            WorldEvent::ModBlock(cords, block_type) => {
                world.set_world_value(block_type.id_as_u16(), *cords);
                event_data.add_render_event(RenderEvent::ReRenderBlock(*cords));
            }
            WorldEvent::FillLocation(world_location, block_texture) => {
                event_data.add_world_events(world_location.borrow().get_area().get_fill_area_events(*block_texture.borrow()));
            },
        }
    }
}
