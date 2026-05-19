
use crate::game_data::{game_event_manager::{game_event_manager::GameEvent, render_event_manager::render_event_manager::RenderEvent}, screen::{ScreenData, screen_data::CurrentMenu, widget::{widget::WidgetType, window_manager::windows::{settings_menu_win::SettingsMenuWidget, window_type::{Window, WindowType}}}}};


pub fn get_menu(_screen_data: &ScreenData) -> WidgetType {
    let back_event = GameEvent::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::MainMenu));
    let widget = SettingsMenuWidget::new(back_event);
    WindowType::SettingsMenu(widget).wrap_into_widget()
}