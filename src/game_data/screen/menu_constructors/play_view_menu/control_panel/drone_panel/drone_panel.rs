use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, LocationEvent}, locations::world_area::WorldArea, player_data::{drone_programming::var::{game_vars::game_var_type::GameVarTypeKind, var_type::VarTypeKind}, locations::location::WorldLocation, player_data::PlayerData}, screen::{menu_constructors::play_view_menu::control_panel::drone_panel::drone_world_view, widget::{drone_programming::vars::var_slot::VarSlot, panel::panel::{PanelAlignment, PanelOrientation}, text::header::TextDisplay, widget::WidgetType, widget_calculations::TextSize}}};

pub fn lock_camera_to_drone_event(location_ref: Rc<RefCell<WorldLocation>>) -> Vec<Event> {
    let events = Vec::new();

    // let location_setting_event = LocationEvent::SetLocationPoint((), ())


    return events;
}

pub fn get_drone_panel(player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);

        // Header
        let scroll_panel = panel.add_scroll_panel();
        scroll_panel.set_prefered_scale(0.8);

        let mut text_display = TextDisplay::new("Controls".to_string());
        text_display.set_text_scale(TextSize::Small);
        scroll_panel.add_widget(WidgetType::TextDisplay(text_display));


        scroll_panel.set_prefered_scale(0.5);

        let drone_slot_ref = VarSlot::new(VarTypeKind::Game(GameVarTypeKind::Drone));



        let location_ref = 
            Rc::new(RefCell::new(WorldLocation::new("Drone_Spectating".to_string(), WorldArea::new_blank(), 404)));


        drone_world_view::add_drone_world_view_panel(panel, player_data, &location_ref);
        

        panel.size();
    }

    return panel;
}