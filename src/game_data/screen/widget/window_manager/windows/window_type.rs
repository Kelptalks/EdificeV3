use crate::game_data::screen::widget::{panel::panel::Panel, widget::{Widget, WidgetType}, widget_properties::WidgetProperties, window_manager::windows::{block_select_win::BlockSelectWindow, cheat_window::CheatWindow, debug_win::DebugWin, debug_world_win::DebugWorldWindow, drone_spectate_win::DroneSpectateWindow, game_entity_win::GameEntityWindow, settings_menu_win::SettingsMenuWidget, settings_window::SettingsWindow}};


pub enum WindowType {
    Custom(Panel),

    CheatWindow(CheatWindow),
    GameEntityWindow(GameEntityWindow),
    Debug(DebugWin),
    DroneSpectate(DroneSpectateWindow),
    DebugWorld(DebugWorldWindow),
    BlockSelect(BlockSelectWindow),
    SettingsMenu(SettingsMenuWidget),
    Settings(SettingsWindow),
}

impl WindowType {
    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::WindowType(self)
    }
    
    fn get_panel(&self) -> &Panel {
        match self {
            WindowType::Custom(panel) => panel,
            WindowType::Debug(w) => w.get_panel(),
            WindowType::DroneSpectate(w) => w.get_panel(),
            WindowType::GameEntityWindow(w) => w.get_panel(),
            WindowType::CheatWindow(w) => w.get_panel(),
            WindowType::DebugWorld(w) => w.get_panel(),
            WindowType::BlockSelect(w) => w.get_panel(),
            WindowType::SettingsMenu(w) => w.get_panel(),
            WindowType::Settings(w) => w.get_panel(),
        }
    }

    fn get_mut_panel(&mut self) -> &mut Panel {
        match self {
            WindowType::Custom(panel) => panel,
            WindowType::Debug(w) => w.get_mut_panel(),
            WindowType::DroneSpectate(w) => w.get_mut_panel(),
            WindowType::GameEntityWindow(w) => w.get_mut_panel(),
            WindowType::CheatWindow(w) => w.get_mut_panel(),
            WindowType::DebugWorld(w) => w.get_mut_panel(),
            WindowType::BlockSelect(w) => w.get_mut_panel(),
            WindowType::SettingsMenu(w) => w.get_mut_panel(),
            WindowType::Settings(w) => w.get_mut_panel(),
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
            WindowType::DebugWorld(w) => {
                w.render(texture_manager, screen_data, game_event_manager, player_data);
            },
            WindowType::BlockSelect(w) => {
                w.render(texture_manager, screen_data, game_event_manager, player_data);
            },
            WindowType::SettingsMenu(w) => {
                w.render(texture_manager, screen_data, game_event_manager, player_data);
            },
            WindowType::Settings(w) => {
                w.render(texture_manager, screen_data, game_event_manager, player_data);
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