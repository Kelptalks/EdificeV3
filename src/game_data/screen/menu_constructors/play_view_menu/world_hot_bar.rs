use crate::game_data::{game_event_manager::prelude::{Event, GameEvent, PlayerDataEvent}, screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{panel::panel::{PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot}, widget::WidgetType}}, types::{BlockTexture, UITextures}};

pub fn get_widget(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        
        // Toggle create location
        let create_location_button = panel.add_button();
        let blank_location_event = PlayerDataEvent::CreateLocationWithVar(ref_manager.selected_world_location.clone()).wrap_into_event();
        create_location_button.add_event(blank_location_event);

        // Toggle show drones
        let show_drones = panel.add_toggle_button();
        show_drones.set_toggle_ref(&ref_manager.show_drones_toggle);
        show_drones.set_block(BlockTexture::DroneUpRight);
        show_drones.set_text("Show Drones".to_string());
        
        // Toggle Show Locations
        let show_locations = panel.add_toggle_button();
        show_locations.set_toggle_ref(&ref_manager.show_locations_toggle);
        show_locations.set_icon(UITextures::LocationIcon);
        show_locations.set_text("Show Locations".to_string());
    
        
        
    }


    return panel;
}
    