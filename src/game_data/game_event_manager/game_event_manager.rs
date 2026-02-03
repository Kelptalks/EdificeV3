use std::sync::{Arc, RwLock};

use crate::game_data::{World, game_event_manager::{render_event_manager::render_event_manager::RenderEvent, world_event_manager::{self, world_event_manager::WorldEvent}}, screen::{Camera, screen_mananager::{self, ScreenManager}, world_config::{self, WorldConfig}}};


pub struct GameEventManager {
    world_events: Vec<WorldEvent>,
    render_events: Vec<RenderEvent>,
}

impl GameEventManager {
    pub fn new() -> GameEventManager {
        GameEventManager {
            world_events: Vec::new(),
            render_events: Vec::new(),
        }
    }

    //=====================================
    // Event constructors
    //=====================================

    pub fn init_world(&mut self, world_config: WorldConfig) {
        let range = world_config.get_chunk_rendering_range();
        self.render_events.push(RenderEvent::InitWorldRender(range));
        self.world_events.push(WorldEvent::GenWorld(world_config));
    }

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

    pub fn execute_render_events(&mut self, screen_mananager: &mut ScreenManager, world: &Arc<RwLock<World>>) {
        while let Some(render_event) = self.render_events.pop() {
            render_event.execute_render_event(screen_mananager, world);
        }
    }
}
