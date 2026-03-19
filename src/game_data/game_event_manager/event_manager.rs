use std::sync::{Arc, RwLock};

use crate::game_data::{World, game_event_manager::{dispatch_event_manager::dispatch_event_manager::DispatchEvent, game_event_manager::{game_event_manager::{GameEvent, GameEventManager}, player_data_event_manager::player_event_manager::PlayerDataEvent, render_event_manager::render_event_manager::RenderEvent, widget_event_manager::widget_event_manager::WidgetEvent, world_event_manager::world_event_manager::WorldEvent}, input_event_manager::input_event_manager::InputEvent}, player_data::player_data::PlayerData, screen::screen_mananager::ScreenManager};

/*
###################
## Event Manager ##
###################
*/

pub enum Event {
    DispatchEvent(DispatchEvent),
    InputEvent(InputEvent),
    GameEvent(GameEvent),
}

pub struct EventManager {
    // Event Disptachers
    dispatch_events: Vec<DispatchEvent>,
    input_events: Vec<InputEvent>,

    // Event tools
    event_data: GameEventManager,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            // Event Disptachers
            dispatch_events: Vec::new(),
            input_events: Vec::new(),

            // Event tools
            event_data: GameEventManager::new(),
            
        }
    }

    //=====================================
    // Getters and setters
    //=====================================
    pub fn get_event_tools(&self) -> &GameEventManager {
        return &self.event_data;
    }

    pub fn get_mut_event_tools(&mut self) -> &mut GameEventManager {
        return &mut self.event_data;
    }


    //=====================================
    // Dispatch Event constructor
    //=====================================

    pub fn add_dispatch_event(&mut self, dispatch_event: DispatchEvent) {
        self.dispatch_events.push(dispatch_event);
    }


    //=====================================
    // Input Event adders
    //=====================================

    pub fn add_input_event(&mut self, input_event: InputEvent) {
        self.input_events.push(input_event);
    }

    pub fn add_input_events(&mut self, input_events: &Vec<InputEvent>) {
        for event in input_events {
            self.add_input_event(event.clone());
        }
    }

    //=====================================
    // Game Event adders
    //=====================================

    pub fn add_world_event(&mut self, world_event: WorldEvent) {
        self.event_data.add_world_event(world_event);
    }

    pub fn add_render_event(&mut self, render_event: RenderEvent) {
        self.event_data.add_render_event(render_event);
    }

    pub fn add_widget_event(&mut self, widget_event: WidgetEvent) {
        self.event_data.add_widget_event(widget_event);
    }

    pub fn add_player_data_event(&mut self, player_data_event: PlayerDataEvent) {
        self.event_data.add_player_data_event(player_data_event);
    }

    pub fn add_game_event(&mut self, event: GameEvent) {
        self.event_data.add_game_event(event);
    }

    pub fn add_game_events(&mut self, events: &Vec<GameEvent>) {
        self.event_data.add_game_events(events);
    }

    //=====================================
    // Execution
    //=====================================

    pub fn execute_input_events(&mut self, screen_mananager: &mut ScreenManager) {
        while let Some(input_event) = self.input_events.pop() {
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
