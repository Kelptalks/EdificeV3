use std::sync::{Arc, RwLock};

use crate::game_data::{World, game_event_manager::{render_event_manager::render_event_manager::RenderEvent, widget_event_manager::widget_event_manager::WidgetEvent, world_event_manager::{self, world_event_manager::WorldEvent}}, level_manager::level_manager::LevelManager, screen::{Camera, screen_mananager::{self, ScreenManager}}, world_gen::WorldGenManager};

/*
#######################
## World Event Tools ##
#######################

*/

#[derive(Clone)]
pub enum Event  {
    WidgetEvent(WidgetEvent),
    WorldEvent(WorldEvent),
    RenderEvent(RenderEvent),
}

pub struct EventData {
    // Events
    widget_events: Vec<WidgetEvent>,
    world_events: Vec<WorldEvent>,
    render_events: Vec<RenderEvent>,

    // Tools
    world_gen_manager: WorldGenManager,
    level_manager: LevelManager,
}

impl EventData {
    pub fn new() -> EventData {
        EventData {
            // Events
            widget_events: Vec::new(),
            world_events: Vec::new(),
            render_events: Vec::new(),

            // Tools
            world_gen_manager: WorldGenManager::new(),
            level_manager: LevelManager::new(),
        }
    }

    //=====================================
    // Tools
    //=====================================

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


    //=====================================
    // Events
    //=====================================

    pub fn add_render_event(&mut self, render_event: RenderEvent) {
        self.render_events.push(render_event);
    }

}

/*
########################
## Game Event Manager ##
########################

*/

pub struct GameEventManager {
    // Event tools
    event_data: EventData,
}

impl GameEventManager {
    pub fn new() -> GameEventManager {
        GameEventManager {
            // Event tools
            event_data: EventData::new(),
            
        }
    }

    //=====================================
    // Getters and setters
    //=====================================
    pub fn get_event_tools(&self) -> &EventData {
        return &self.event_data;
    }

    //=====================================
    // Event constructors
    //=====================================

    pub fn init_world(&mut self) {
        self.event_data.render_events.push(RenderEvent::InitWorldRender());
        self.event_data.world_events.push(WorldEvent::GenWorld());
    }

    pub fn add_world_event(&mut self, world_event: WorldEvent) {
        self.event_data.world_events.push(world_event);
    }

    pub fn add_render_event(&mut self, render_event: RenderEvent) {
        self.event_data.render_events.push(render_event);
    }

    pub fn add_widget_event(&mut self, widget_event: WidgetEvent) {
        self.event_data.widget_events.push(widget_event);
    }

    pub fn add_event(&mut self, event: Event) {
        match event {
            Event::WorldEvent(world_event) => {
                self.add_world_event(world_event);
            },
            Event::RenderEvent(render_event) => {
                self.add_render_event(render_event);
            },
            Event::WidgetEvent(widget_event) => {
                self.add_widget_event(widget_event);
            }
        }
    }

    //=====================================
    // Execution
    //=====================================

    pub fn execute_world_events(&mut self, world: &mut World) {
        while let Some(world_event) = self.event_data.world_events.pop() {
            world_event.execute_world_event(world, &mut self.event_data);
        }
    }

    pub fn execute_render_events(&mut self, screen_mananager: &mut ScreenManager, world: &Arc<RwLock<World>>) {
        while let Some(render_event) = self.event_data.render_events.pop() {
            render_event.execute_render_event(&mut self.event_data, screen_mananager, world);
        }
    }

    pub fn execute_widget_events(&mut self) {
        while let Some(widget_event) = self.event_data.widget_events.pop() {
            widget_event.execute_widget_event(&mut self.event_data);
        }
    }
}
