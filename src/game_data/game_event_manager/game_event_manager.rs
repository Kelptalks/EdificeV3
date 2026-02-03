use std::sync::{Arc, RwLock};

use crate::game_data::{World, game_event_manager::{render_event_manager::render_event_manager::RenderEvent, world_event_manager::{self, world_event_manager::{WorldEvent}}}, level_manager::level_manager::LevelManager, screen::{Camera, menus::world_creation_menu::world_config::WorldConfig, screen_mananager::{self, ScreenManager}}, world_gen::WorldGenManager};

/*
#######################
## World Event Tools ##
#######################

*/

pub struct EventTools {
    world_gen_manager: WorldGenManager,
    level_manager: LevelManager,
}

impl EventTools {
    pub fn new() -> EventTools {
        EventTools {
            world_gen_manager: WorldGenManager::new(),
            level_manager: LevelManager::new(),
        }
    }


    pub fn get_mut_world_gen_manager(&mut self) -> &mut WorldGenManager {
        return &mut self.world_gen_manager;
    }

    pub fn get_mut_level_manager(&mut self) -> &mut LevelManager {
        return &mut self.level_manager;
    }

    pub fn get_world_gen_manager(&self) -> &WorldGenManager {
        return &self.world_gen_manager;
    }

    pub fn get_level_manager(&self) -> &LevelManager {
        return &self.level_manager;
    }

}

/*
########################
## Game Event Manager ##
########################

*/

pub struct GameEventManager {
    // Event
    world_events: Vec<WorldEvent>,
    render_events: Vec<RenderEvent>,

    // Event tools
    event_tools: EventTools,
}

impl GameEventManager {
    pub fn new() -> GameEventManager {
        GameEventManager {
            // Events
            world_events: Vec::new(),
            render_events: Vec::new(),

            // Event tools
            event_tools: EventTools::new(),
            
        }
    }

    //=====================================
    // Getters and setters
    //=====================================
    pub fn get_event_tools(&self) -> &EventTools {
        return &self.event_tools;
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

    pub fn add_render_event(&mut self, render_event: RenderEvent) {
        self.render_events.push(render_event);
    }

    //=====================================
    // Execution
    //=====================================

    pub fn execute_world_events(&mut self, world: &mut World) {
        while let Some(world_event) = self.world_events.pop() {
            world_event.execute_world_event(world, &mut self.event_tools);
        }
    }

    pub fn execute_render_events(&mut self, screen_mananager: &mut ScreenManager, world: &Arc<RwLock<World>>) {
        while let Some(render_event) = self.render_events.pop() {
            render_event.execute_render_event(&mut self.event_tools, screen_mananager, world);
        }
    }
}
