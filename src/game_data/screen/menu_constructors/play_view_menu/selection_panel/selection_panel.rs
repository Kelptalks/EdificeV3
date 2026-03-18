use crate::game_data::{player_data::player_data::PlayerData, screen::{ScreenData, menu_constructors::play_view_menu::selection_panel::building_panel, widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget_calculations::TextSize}}, types::UITextures};



pub fn add_control_panel(panel: &mut Panel, screen_data: &ScreenData, player_data: &mut PlayerData) {
    let controls_sub_panel = panel.add_sub_panel();
    
    controls_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
    
    // Header
    let text_display = controls_sub_panel.add_text_display("Controls".to_string());
    text_display.set_text_scale(TextSize::Large);


    let control_tab_panel = controls_sub_panel.add_tab_panel();

    // Add Drone Controls Tab
    let button = control_tab_panel.add_panel(get_drone_panel(player_data));
    button.set_text("drones".to_string());
    button.set_block(crate::game_data::types::BlockTexture::DroneBotRight);

    // Add Location Manager Tab
    let button = control_tab_panel.add_panel(get_location_panel(player_data));
    button.set_text("locations".to_string());
    button.set_icon(UITextures::AreaIcon);

    // Add Location Manager Tab
    let button = control_tab_panel.add_panel(building_panel::building_panel::get_building_panel(player_data));
    button.set_text("Building".to_string());
    button.set_icon(UITextures::BluePrintIcon);
    
}