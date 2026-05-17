#![allow(dead_code)]
use crate::game_data::screen::{menu_constructors::play_view_menu::{manager_panel::{blue_print_management_panel::get_blue_print_management_panel, drone_managment_panel::get_drone_managment_panel, locaton_managmenet_panel::get_location_panel, main_managment_panel::get_main_view_panel}, new_play_view::RefManager}, widget::{prelude::TabPanel, widget::WidgetType}};




pub fn get_widget(ref_manager: &mut RefManager) -> WidgetType {    
    let mut tab_panel = TabPanel::new(&ref_manager.play_view_mode);

    tab_panel.set_button_panel_visiblity(false);

    tab_panel.add_panel(get_main_view_panel(ref_manager));
    tab_panel.add_panel(get_location_panel(ref_manager));
    tab_panel.add_panel(get_blue_print_management_panel(ref_manager));
    tab_panel.add_panel(get_drone_managment_panel(ref_manager));

    return tab_panel.wrap_into_widget();
}


