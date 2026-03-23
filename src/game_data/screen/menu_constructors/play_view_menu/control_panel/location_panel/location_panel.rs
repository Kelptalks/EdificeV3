use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, LocationEvent}, locations::world_area::WorldArea, player_data::{locations::{location::WorldLocation, location_config::{self, WorldLocationConfig}, location_manager::LocationManager}, player_data::PlayerData}, screen::{menu_constructors::play_view_menu::control_panel::location_panel::location_world_view, ui_elements::panel, widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_color::PanelColor}, text::header::TextDisplay, widget::WidgetType, widget_calculations::TextSize}}, types::UITextures};

pub fn add_location_shifting_panel(panel: &mut Panel, location_ref: Rc<RefCell<WorldLocation>>) {
    let sub_panel = panel.add_sub_panel();
    sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

    let row1_panel = sub_panel.add_sub_panel();
    row1_panel.set_color(PanelColor::Clear);
    row1_panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
    
    let top_left_button = row1_panel.add_button();
    top_left_button.set_icon(UITextures::ScallingIconTopLeft);
    top_left_button.add_event(LocationEvent::ModSize([1, 0, 0], true).wrap_into_event(location_ref.clone()));

    let top_right_button = row1_panel.add_button();
    top_right_button.set_icon(UITextures::ScallingIconTopRight);
    top_right_button.add_event(LocationEvent::ModSize([-1, 0, 0], true).wrap_into_event(location_ref.clone()));

    let row2_panel = sub_panel.add_sub_panel();
    row2_panel.set_color(PanelColor::Clear);
    row2_panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
    let bot_left_button = row2_panel.add_button();
    bot_left_button.set_icon(UITextures::ScallingIconBotLeft);
    bot_left_button.add_event(LocationEvent::ModSize([0, 1, 0], true).wrap_into_event(location_ref.clone()));

    let bot_right_button = row2_panel.add_button();
    bot_right_button.set_icon(UITextures::ScallingIconBotRight);
    bot_right_button.add_event(LocationEvent::ModSize([0, -1, 0], true).wrap_into_event(location_ref.clone()));
}

pub fn get_location_panel(player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);

        // Create a location for the use in location config, rendering, and input based minipulation
        let location_ref = player_data.get_mut_location_manager().create_location("Location Config".to_string(), WorldArea::new_blank());
        let location_config = WorldLocationConfig::new(location_ref.clone());
        location_config.get_location_name();

        location_world_view::add_location_world_view_panel(panel, player_data, location_ref.clone());

        let control_sub_panel = panel.add_sub_panel();
        control_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);



        let create_location_button = control_sub_panel.add_button();
        create_location_button.set_text("Create Location".to_string());
        create_location_button.set_icon(UITextures::LocationIcon);
        let create_location_event = 

        add_location_shifting_panel(control_sub_panel, location_ref);

        panel.size();
    }


    return panel;
}


