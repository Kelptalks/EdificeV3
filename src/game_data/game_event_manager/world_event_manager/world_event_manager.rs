use crate::game_data::{World, screen::world_config::{self, WorldConfig}};

/*
#################
## World Event ##
#################


*/
pub enum WorldEvent {
    Clear,                          // No Data
    GenWorld(WorldConfig),     // World Config
    GenLevel(u32)              // Level Id
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
    pub fn execute_world_event(&self, world: &mut World) {
        match self {
            WorldEvent::Clear => {

            },
            WorldEvent::GenWorld(world_config) => {
                
            },
            WorldEvent::GenLevel(level) => {
                
            },
        }
    }
}