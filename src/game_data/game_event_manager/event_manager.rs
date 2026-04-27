
use std::{cell::RefCell, rc::Rc};

use crate::game_data::{World, game_event_manager::{dispatch_event_manager::dispatch_event_manager::DispatchEvent, game_event_manager::{game_event_manager::{GameEvent, GameEventManager}, player_data_event_manager::player_event_manager::PlayerDataEvent, render_event_manager::render_event_manager::RenderEvent, widget_event_manager::widget_event_manager::WidgetEvent, world_event_manager::world_event_manager::WorldEvent}, input_event_manager::input_event_manager::InputEvent}, player_data::player_data::PlayerData, screen::screen_mananager::ScreenManager};

/*
###################
## Event Manager ##
###################
*/

#[derive(Clone)]
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
    game_event_manager: GameEventManager,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            // Event Disptachers
            dispatch_events: Vec::new(),
            input_events: Vec::new(),

            // Event tools
            game_event_manager: GameEventManager::new(),
            
        }
    }

    //=====================================
    // Getters and setters
    //=====================================
    pub fn get_event_tools(&self) -> &GameEventManager {
        return &self.game_event_manager;
    }

    pub fn get_mut_event_tools(&mut self) -> &mut GameEventManager {
        return &mut self.game_event_manager;
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
        self.game_event_manager.add_world_event(world_event);
    }

    pub fn add_render_event(&mut self, render_event: RenderEvent) {
        self.game_event_manager.add_render_event(render_event);
    }

    pub fn add_widget_event(&mut self, widget_event: WidgetEvent) {
        self.game_event_manager.add_widget_event(widget_event);
    }

    pub fn add_player_data_event(&mut self, player_data_event: PlayerDataEvent) {
        self.game_event_manager.add_player_data_event(player_data_event);
    }

    pub fn add_game_event(&mut self, event: GameEvent) {
        self.game_event_manager.add_game_event(event);
    }

    pub fn add_game_events(&mut self, events: &Vec<GameEvent>) {
        self.game_event_manager.add_game_events(events);
    }

    //=====================================
    // Event
    //=====================================

    pub fn add_event(&mut self, event: Event) {
        match event {
            Event::DispatchEvent(dispatch_event) => {
                self.dispatch_events.push(dispatch_event);
            },
            Event::InputEvent(input_event) => {
                self.input_events.push(input_event);
            },
            Event::GameEvent(game_event) => {
                self.add_game_event(game_event);
            },
        }
    }

    pub fn add_events(&mut self, events: &Vec<Event>) {
        for event in events {
            self.add_event(event.clone());
        }
    }

    //=====================================
    // Execution
    //=====================================

    pub fn execute_dispatch_events(&mut self) {
        while let Some(dispatch_event) = self.dispatch_events.pop() {
            self.add_events(&dispatch_event.get_events_dispatched());
        }
    }

    pub fn dispatch_input_events(&mut self, screen_mananager: &mut ScreenManager) {
        while let Some(input_event) = self.input_events.pop() {
            self.add_events(&input_event.get_input_events_to_dispatch(screen_mananager.get_screen_data()));
        }
        self.execute_dispatch_events();
    }
    
    pub fn execute_world_events(&mut self, world: &mut World) {
        while let Some(world_event) = self.game_event_manager.world_events.pop() {
            world_event.execute_world_event(world, &mut self.game_event_manager);
        }
    }

    pub fn execute_render_events(&mut self, screen_mananager: &mut ScreenManager, player_data: &PlayerData) {
        while let Some(render_event) = self.game_event_manager.render_events.pop() {
            render_event.execute_render_event(&mut self.game_event_manager, screen_mananager, player_data);
        }
    }

    pub fn execute_player_data_events(&mut self, player_data: &mut PlayerData) {
        while let Some(player_data_event) = self.game_event_manager.player_data_events.pop() {
            player_data_event.execute_player_data_events(&mut self.game_event_manager, player_data);
        }
    }

    pub fn execute_widget_events(&mut self) {
        while let Some(widget_event) = self.game_event_manager.widget_events.pop() {
            let events = widget_event.execute_widget_event(&mut self.game_event_manager);
            self.add_events(&events);
        }
    }

}
