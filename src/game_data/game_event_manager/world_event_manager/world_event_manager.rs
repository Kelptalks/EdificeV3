use rand::rand_core::le;

use crate::game_data::{World, game_event_manager::game_event_manager::EventTools, level_manager::level_manager::LevelManager, screen::menus::world_creation_menu::world_config::WorldConfig, types::BlockType, world_gen::WorldGenManager};

/*
#################
## World Event ##
#################

*/

pub enum WorldEvent {
    Clear,                     // No Data
    GenWorld(WorldConfig),     // World Config
    GenLevel(u32),             // Level Id
    ModBlock([i32; 3], BlockType)
}

impl WorldEvent {
    
    //=====================================
    // Constructors 
    //=====================================
    pub fn new_clear() -> WorldEvent {
        return WorldEvent::Clear;
    }
    pub fn new_gen_world(world_config: WorldConfig) -> WorldEvent {
        return WorldEvent::GenWorld(world_config);
    }
    pub fn new_gen_level(level: u32) -> WorldEvent {
        return WorldEvent::GenLevel(level);
    }

    //=====================================
    // Execution
    //=====================================
    pub fn execute_world_event(&self, world: &mut World, event_tools: &mut EventTools) {
        match self {
            WorldEvent::Clear => {
                world.clear();
            },
            WorldEvent::GenWorld(world_config) => {
                let size = world_config.get_scale() as i32 / 2;
                let start_cords = [-size, -size, -100];
                let end_cords = [size, size, 100];
                        
                event_tools.get_mut_world_gen_manager().generate_area(world, start_cords, end_cords);
            },
            WorldEvent::GenLevel(level) => {
                event_tools.get_level_manager().get_level_at_index(level.clone() as usize).gen_level(world);
            },
            WorldEvent::ModBlock(cords, block_type) => {
                world.set_world_value(block_type.id_as_u16(), *cords);
            }
        }
    }
}
