use crate::game_data::{World, game_event_manager::world_event_manager::world_event_manager::WorldEvent};


pub struct GameEventManager {
    world_events: Vec<WorldEvent>
}

impl GameEventManager {
    pub fn new() -> GameEventManager {
        GameEventManager {
            world_events: Vec::new(),
        }
    }

    //=====================================
    // Adding events
    //=====================================

    pub fn add_world_event(&mut self, world_event: WorldEvent) {
        self.world_events.push(world_event);
    }

    //=====================================
    // Execution
    //=====================================

    pub fn execute_world_events(&mut self, world: &mut World) {
        while let Some(world_event) = self.world_events.pop() {
            world_event.execute_world_event(world);
        }
    }
}
