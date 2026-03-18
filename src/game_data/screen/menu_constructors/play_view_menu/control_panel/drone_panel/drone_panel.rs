use crate::game_data::{player_data::player_data::PlayerData, screen::{menu_constructors::play_view_menu::control_panel::drone_panel::drone_world_view, widget::{panel::panel::{PanelAlignment, PanelOrientation}, text::header::TextDisplay, widget::WidgetType, widget_calculations::TextSize}}};

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

        drone_world_view::add_drone_world_view_panel(panel, player_data);
        

        panel.size();
    }

    return panel;
}