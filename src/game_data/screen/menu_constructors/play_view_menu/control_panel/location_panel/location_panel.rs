use crate::game_data::{player_data::player_data::PlayerData, screen::{menu_constructors::play_view_menu::control_panel::{location_panel::location_world_view}, widget::{panel::panel::{PanelAlignment, PanelOrientation}, text::header::TextDisplay, widget::WidgetType, widget_calculations::TextSize}}};

pub fn get_location_panel(player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Header
        let text_display = panel.add_text_display("Location Manager".to_string());
        text_display.set_text_scale(TextSize::Medium);
        
        location_world_view::add_location_world_view_panel(panel, player_data);

        panel.size();
    }


    return panel;
}


