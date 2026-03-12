use rand::rand_core::le;

use crate::game_data::{World, game_event_manager::{game_event_manager::EventData, render_event_manager::render_event_manager::RenderEvent}, types::BlockTexture};

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
    ModBlock([i32; 3], BlockTexture)    // Block Cords, Block Type
}

impl WorldEvent {
    //=====================================
    // Execution
    //=====================================
    pub fn execute_world_event(&self, world: &mut World, event_data: &mut EventData) {
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
        }
    }
}
