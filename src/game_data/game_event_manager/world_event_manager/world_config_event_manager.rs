use crate::game_data::{World, game_event_manager::game_event_manager::EventData};

#[derive(Clone)]
pub enum WorldConfigEvent {
    ModWorldSize(i32),
}


impl WorldConfigEvent {
    //=====================================
    // Execution
    //=====================================
    pub fn execute_world_event(&self, world: &mut World, event_data: &mut EventData) {
        let world_config = event_data.get_mut_world_gen_manager().get_mut_world_config();
        match self {
            WorldConfigEvent::ModWorldSize(scale) => {
                world_config.mod_scale(*scale);
            },
        }
    }
}