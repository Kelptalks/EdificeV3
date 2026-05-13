use crate::game_data::screen::widget::{panel::panel::Panel, widget::{Widget, WidgetType}, widget_properties::WidgetProperties, window_manager::windows::{cheat_window::CheatWindow, debug_win::DebugWin, drone_spectate_win::DroneSpectateWindow, game_entity_win::GameEntityWindow}};


pub enum WindowType {
    Custom(Panel),
    
    CheatWindow(CheatWindow),
    GameEntityWindow(GameEntityWindow),
    Debug(DebugWin),
    DroneSpectate(DroneSpectateWindow),
}

impl WindowType {
    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::WindowType(self)
    }
    
    fn get_panel(&self) -> &Panel {
        match self {
            WindowType::Custom(panel) => {
                panel
            }
            WindowType::Debug(debug_win) => debug_win.get_panel(),
            WindowType::DroneSpectate(drone_spectate_window) => drone_spectate_window.get_panel(),
            WindowType::GameEntityWindow(game_entity_window) => game_entity_window.get_panel(),
            WindowType::CheatWindow(cheat_window) => cheat_window.get_panel(),
        }  
    } 

    fn get_mut_panel(&mut self) -> &mut Panel {
        match self {
            WindowType::Custom(panel) => {
                panel
            },
            WindowType::Debug(debug_win) => debug_win.get_mut_panel(),
            WindowType::DroneSpectate(drone_spectate_window) => drone_spectate_window.get_mut_panel(),
            WindowType::GameEntityWindow(game_entity_window) => game_entity_window.get_mut_panel(),
            WindowType::CheatWindow(cheat_window) => cheat_window.get_mut_panel(),
        }
    } 
}

impl Widget for WindowType {
    fn get_widget_properties(&self) -> &WidgetProperties {
        self.get_panel().get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        self.get_mut_panel().get_mut_widget_properties()
    }

    fn size(&mut self) {
        let panel = self.get_mut_panel();
        panel.size();
        // panel.set_parent_pos(pos);

    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        match self {
            WindowType::Custom(panel) => {
                panel.render(texture_manager, screen_data, game_event_manager, player_data);
            },
            WindowType::Debug(debug_win) => {
                debug_win.render(texture_manager, screen_data, game_event_manager, player_data)
            },
            WindowType::DroneSpectate(drone_spectate_window) => {
                drone_spectate_window.render(texture_manager, screen_data, game_event_manager, player_data);
            },
            WindowType::GameEntityWindow(game_entity_window) => {
                game_entity_window.render(texture_manager, screen_data, game_event_manager, player_data);
            },
            WindowType::CheatWindow(cheat_window) => {
                cheat_window.render(texture_manager, screen_data, game_event_manager, player_data);
            },
        }
    }
}

pub trait Window {
    fn wrap_into_window_type(self) -> WindowType;
    

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    );

    fn get_mut_panel(&mut self) -> &mut Panel;

    fn get_panel(&self) -> &Panel;
}