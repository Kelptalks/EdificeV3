
use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::player_data::PlayerData, screen::{ScreenData, menu_constructors::play_view_menu::new_play_view::PlayViewConstructionManager, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::{Widget, WidgetType}, window_manager::widget_window_manager::WidgetWindowManager}}, types::UITextures};


//=====================================
// Root panel
//=====================================

pub fn get_menu(screen_data: &ScreenData, player_data: &PlayerData) -> WidgetType {
    return WidgetType::WidgetWindowManager(WidgetWindowManager::new(screen_data));
}


pub fn get_new_menu(screen_data: &ScreenData, player_data: &PlayerData) -> WidgetType {
    let mut play_view_menu_construct = PlayViewConstructionManager::new(player_data);
    return play_view_menu_construct.build_panel(screen_data);
}