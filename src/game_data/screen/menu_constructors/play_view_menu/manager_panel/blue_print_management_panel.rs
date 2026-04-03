use crate::game_data::screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{panel::panel::{PanelAlignment, PanelOrientation}, prelude::PanelColor, widget::WidgetType, widget_calculations::TextSize}};

pub fn get_blue_print_management_panel(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        let panel_title = panel.add_text_display("Blue Print".to_string());
        panel_title.set_text_scale(TextSize::Large);
        

        panel.size();
    }
    return panel;
}

