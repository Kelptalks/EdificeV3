use crate::game_data::{screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{panel::panel::{PanelAlignment, PanelOrientation}, prelude::PanelColor, widget::WidgetType}}, types::{BlockTexture, UITextures}};

pub fn get_widget(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        
        // Toggle create location
        let create_location_button = panel.add_button();

        // Toggle show drones
        let show_drones = panel.add_toggle_button();
        show_drones.set_toggle_ref(&ref_manager.show_drones_toggle);
        show_drones.set_block(BlockTexture::DroneUpRight);
        
        // Toggle Show Locations
        let show_locations = panel.add_toggle_button();
        show_locations.set_toggle_ref(&ref_manager.show_locations_toggle);
        show_locations.set_icon(UITextures::LocationIcon);

    }


    return panel;
}
    