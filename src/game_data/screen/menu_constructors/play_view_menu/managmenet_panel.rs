use crate::game_data::screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{panel::panel::{PanelAlignment, PanelOrientation}, prelude::{PanelColor, TabPanel}, widget::WidgetType, widget_calculations::TextSize}};

fn get_main_view_panel(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        let panel_title = panel.add_text_display("Main View".to_string());
        panel_title.set_text_scale(TextSize::Large);
        

        panel.size();
    }
    return panel;
}



//=====================================
// Location Managment
//=====================================
fn get_location_scalling_mods(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {


 


        panel.size();
    }
    return panel;
}

fn get_location_panel(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        let panel_title = panel.add_text_display("Location".to_string());
        panel_title.set_text_scale(TextSize::Large);
        

        let location_name = panel.add_text_display(ref_manager.selected_var.borrow().get_name());
        location_name.set_text_scale(TextSize::Large);


        // Location Modification buttons 
        panel.add_widget(self::get_location_scalling_mods(ref_manager));





        panel.size();
    }
    return panel;
}


pub fn get_widget(ref_manager: &mut RefManager) -> WidgetType {    
    let mut tab_panel = TabPanel::new(&ref_manager.play_view_mode);

    tab_panel.set_button_panel_visiblity(false);

    tab_panel.add_panel(get_main_view_panel(ref_manager));
    tab_panel.add_panel(get_location_panel(ref_manager));


    return tab_panel.wrap_into_widget();
}

