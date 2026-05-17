
use crate::game_data::screen::{ScreenData, widget::{panel::panel::Panel, widget::WidgetType}};



pub fn get_menu(_screen_data: &ScreenData) -> WidgetType {
    
    let panel = Panel::new_blank();

    panel.wrap_into_widget()


}