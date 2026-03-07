use rand::rand_core::le;

use crate::game_data::{World, game_event_manager::{game_event_manager::EventData, render_event_manager::render_event_manager::RenderEvent}, level_manager::level_manager::LevelManager, screen::menus::world_creation_menu::world_config::WorldConfig, types::BlockTexture, world_gen::WorldGenManager};

/*
#################
## World Event ##
#################
Enum and logic for executing events pertaining 
to modifications to the world
*/

#[derive(Clone)]
pub enum WorldEvent {
    Clear,                     // No Data
    GenWorld(WorldConfig),     // World Config
    GenLevel(u32),             // Level Id
    ModBlock([i32; 3], BlockTexture)
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
            WorldEvent::GenWorld(world_config) => {
                let size = world_config.get_scale() as i32 / 2;
                let start_cords = [-size, -size, -100];
                let end_cords = [size, size, 100];
                        
                event_data.get_mut_world_gen_manager().generate_area(world, start_cords, end_cords);
                
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
