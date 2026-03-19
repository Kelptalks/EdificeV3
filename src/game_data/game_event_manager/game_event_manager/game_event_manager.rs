use crate::game_data::{game_event_manager::game_event_manager::{player_data_event_manager::player_event_manager::PlayerDataEvent, render_event_manager::render_event_manager::RenderEvent, widget_event_manager::{mouse_action_manager::MouseWidgetData, widget_event_manager::WidgetEvent}, world_event_manager::world_event_manager::WorldEvent}, level_manager::level_manager::LevelManager, world_gen::WorldGenManager};

#[derive(Clone)]
pub enum GameEvent  {
    // Game
    WidgetEvent(WidgetEvent),
    WorldEvent(WorldEvent),
    RenderEvent(RenderEvent),
    PlayerDataEvent(PlayerDataEvent),
}

pub struct GameEventManager {
    // Game Events
    pub widget_events: Vec<WidgetEvent>,
    pub world_events: Vec<WorldEvent>,
    pub render_events: Vec<RenderEvent>,
    pub player_data_events: Vec<PlayerDataEvent>,

    // Tools
    world_gen_manager: WorldGenManager,
    level_manager: LevelManager,
    mouse_widget_data: MouseWidgetData,
}

impl GameEventManager {
    pub fn new() -> GameEventManager {
        GameEventManager {
            // Events
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

    //=====================================
    // Game Events
    //=====================================

    pub fn add_render_event(&mut self, render_event: RenderEvent) {
        self.render_events.push(render_event);
    }

    pub fn add_world_event(&mut self, world_event: WorldEvent) {
        self.world_events.push(world_event);
    }

    pub fn add_world_events(&mut self, events: Vec<WorldEvent>) {
        for event in events {
            self.add_world_event(event);
        }
    }

    pub fn add_widget_event(&mut self, widget_event: WidgetEvent) {
        self.widget_events.push(widget_event);
    }

    pub fn add_player_data_event(&mut self, player_data_event: PlayerDataEvent) {
        self.player_data_events.push(player_data_event);
    }

    pub fn add_game_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::WorldEvent(world_event) => {
                self.add_world_event(world_event);
            },
            GameEvent::RenderEvent(render_event) => {
                self.add_render_event(render_event);
            },
            GameEvent::WidgetEvent(widget_event) => {
                self.add_widget_event(widget_event);
            }
            GameEvent::PlayerDataEvent(player_data_event) => {
                self.add_player_data_event(player_data_event);
            }
        }
    }

    pub fn add_game_events(&mut self, events: &Vec<GameEvent>) {
        for event in events {
            self.add_game_event(event.clone());
        }
    }

}