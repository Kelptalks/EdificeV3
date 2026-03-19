use std::{cell::RefCell, rc::Rc};

use rand::rand_core::le;

use crate::game_data::{World, game_event_manager::{game_event_manager::GameEventManager, render_event_manager::render_event_manager::RenderEvent}, locations::world_area::WorldArea, player_data::locations::location::WorldLocation, types::BlockTexture};

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
    Clear,                              // No Data
    GenWorld(),              // World Config
    
    // Modifcation
    GenLevel(u32),                      // Level Id
    ModBlock([i32; 3], BlockTexture),    // Block Cords, Block Type
    FillLocation(Rc<RefCell<WorldLocation>>, BlockTexture)
}

impl WorldEvent {
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
            WorldEvent::ModBlock(cords, block_type) => {
                world.set_world_value(block_type.id_as_u16(), *cords);
                event_data.add_render_event(RenderEvent::ReRenderBlock(*cords));
            }
            WorldEvent::FillLocation(world_location, block_texture) => {
                event_data.add_world_events(world_location.borrow().get_area().get_fill_area_events(*block_texture));
            },
        }
    }
}
