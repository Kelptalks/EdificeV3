use std::sync::{Arc, RwLock};

use crate::game_data::{World, game_event_manager::{input_event_manager::input_event_manager::InputEvent, player_data_event_manager::player_event_manager::PlayerDataEvent, render_event_manager::render_event_manager::RenderEvent, widget_event_manager::{mouse_action_manager::MouseWidgetData, widget_event_manager::WidgetEvent}, world_event_manager::{self, world_event_manager::WorldEvent}}, level_manager::level_manager::LevelManager, player_data::{self, player_data::PlayerData}, screen::{Camera, screen_mananager::{self, ScreenManager}}, world_gen::WorldGenManager};

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
    PlayerDataEvent(PlayerDataEvent),
}

pub struct EventData {
    // Input Events
    input_events: Vec<InputEvent>,
    
    // Game Events
    widget_events: Vec<WidgetEvent>,
    world_events: Vec<WorldEvent>,
    render_events: Vec<RenderEvent>,
    player_data_events: Vec<PlayerDataEvent>,

    // Tools
    world_gen_manager: WorldGenManager,
    level_manager: LevelManager,
    mouse_widget_data: MouseWidgetData,
}

impl EventData {
    pub fn new() -> EventData {
        EventData {
            // Events
            input_events: Vec::new(),
            widget_events: Vec::new(),
            world_events: Vec::new(),
            render_events: Vec::new(),
            player_data_events: Vec::new(),

            // Tools
            world_gen_manager: WorldGenManager::new(),
            level_manager: LevelManager::new(),
            mouse_widget_data: MouseWidgetData::new(),
        }
    }

    //=====================================
    // Tools Getters
    //=====================================

    // World Gen
    pub fn get_mut_world_gen_manager(&mut self) -> &mut WorldGenManager {
        return &mut self.world_gen_manager;
    }
    pub fn get_world_gen_manager(&self) -> &WorldGenManager {
        return &self.world_gen_manager;
    }

    // Level Gen
    pub fn get_mut_level_manager(&mut self) -> &mut LevelManager {
        return &mut self.level_manager;
    }
    pub fn get_level_manager(&self) -> &LevelManager {
        return &self.level_manager;
    }

    // mouse action
    pub fn get_mut_mouse_widget_data(&mut self) -> &mut MouseWidgetData {
        return &mut self.mouse_widget_data;
    }

    pub fn get_mouse_widget_data(&self) -> &MouseWidgetData {
        return &self.mouse_widget_data;
    }

    pub fn add_input_event(&mut self, input_event: InputEvent) {
        self.input_events.push(input_event);
    }


    //=====================================
    // Input Events
    //=====================================

    pub fn add_input_events(&mut self, input_events: &Vec<InputEvent>) {
        for event in input_events {
            self.add_input_event(event.clone());
        }
    }

    //=====================================
    // Game Events
    //=====================================

    pub fn add_render_event(&mut self, render_event: RenderEvent) {
        self.render_events.push(render_event);
    }

    pub fn add_world_event(&mut self, world_event: WorldEvent) {
        self.world_events.push(world_event);
    }

    pub fn add_widget_event(&mut self, widget_event: WidgetEvent) {
        self.widget_events.push(widget_event);
    }

    pub fn add_player_data_event(&mut self, player_data_event: PlayerDataEvent) {
        self.player_data_events.push(player_data_event);
    }

    pub fn add_game_event(&mut self, event: Event) {
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
            Event::PlayerDataEvent(player_data_event) => {
                self.add_player_data_event(player_data_event);
            }
        }
    }

    pub fn add_game_events(&mut self, events: &Vec<Event>) {
        for event in events {
            self.add_game_event(event.clone());
        }
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

    pub fn get_mut_event_tools(&mut self) -> &mut EventData {
        return &mut self.event_data;
    }


    //=====================================
    // Input Event constructor
    //=====================================

    pub fn add_input_event(&mut self, input_event: InputEvent) {
        self.event_data.input_events.push(input_event);
    }

    pub fn add_input_events(&mut self, input_events: &Vec<InputEvent>) {
        self.event_data.add_input_events(input_events);
    }


    //=====================================
    // Game Event constructors
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

    pub fn add_player_data_event(&mut self, player_data_event: PlayerDataEvent) {
        self.event_data.player_data_events.push(player_data_event);
    }

    pub fn add_event(&mut self, event: Event) {
        self.event_data.add_game_event(event);
    }

    pub fn add_events(&mut self, events: &Vec<Event>) {
        self.event_data.add_game_events(events);
    }

    //=====================================
    // Execution
    //=====================================

    pub fn execute_input_events(&mut self, screen_mananager: &mut ScreenManager) {
        while let Some(input_event) = self.event_data.input_events.pop() {
            input_event.execute_input_events(&mut self.event_data, screen_mananager.get_screen_data());
        }
    }

    pub fn execute_world_events(&mut self, world: &mut World) {
        while let Some(world_event) = self.event_data.world_events.pop() {
            world_event.execute_world_event(world, &mut self.event_data);
        }
    }

    pub fn execute_render_events(&mut self, screen_mananager: &mut ScreenManager, player_data: &mut PlayerData) {
        while let Some(render_event) = self.event_data.render_events.pop() {
            render_event.execute_render_event(&mut self.event_data, screen_mananager, player_data);
        }
    }

    pub fn execute_player_data_events(&mut self, player_data: &mut PlayerData) {
        while let Some(player_data_event) = self.event_data.player_data_events.pop() {
            player_data_event.execute_player_data_events(&mut self.event_data, player_data);
        }
    }

    pub fn execute_widget_events(&mut self) {
        while let Some(widget_event) = self.event_data.widget_events.pop() {
            widget_event.execute_widget_event(&mut self.event_data);
        }
    }

}
