use crate::game_data::screen::widget::{panel::panel::Panel, widget::{Widget, WidgetType}, widget_properties::WidgetProperties, window_manager::windows::debug_win::DebugWin};

pub enum WindowType {
    Debug(DebugWin)
}

impl WindowType {
    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::WindowType(self)
    }
    
    fn get_panel(&self) -> &Panel {
        match self {
            WindowType::Debug(debug_win) => debug_win.get_panel(),
        }  
    } 

    fn get_mut_panel(&mut self) -> &mut Panel {
        match self {
            WindowType::Debug(debug_win) => debug_win.get_mut_panel(),
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
        self.get_mut_panel().size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        match self {
            WindowType::Debug(debug_win) => {
                debug_win.render(texture_manager, screen_data, game_event_manager, player_data)
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