use crate::game_data::{game_event_manager::{game_event_manager::Event, render_event_manager::render_event_manager::RenderEvent}, player_data::player_data::PlayerData, screen::{ScreenData, screen_data::CurrentMenu, widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::WidgetType, widget_calculations::TextSize}}, types::UITextures};



pub fn add_control_panel(panel: &mut Panel, screen_data: &ScreenData, player_data: &mut PlayerData) {
    let controls_sub_panel = panel.add_sub_panel();
    controls_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
    
    // Header
    let text_display = controls_sub_panel.add_text_display("Controls".to_string());
    text_display.set_text_scale(TextSize::Medium);

    controls_sub_panel.size();
    
}

pub fn add_play_view_panel(panel: &mut Panel, screen_data: &ScreenData, player_data: &mut PlayerData) {
    let play_view_sub_panel = panel.add_sub_panel();
    play_view_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

    // Header
    let text_display = play_view_sub_panel.add_text_display("play view".to_string());
    text_display.set_text_scale(TextSize::Medium);

    // Add World Rendering
    let play_view = play_view_sub_panel.add_play_world_view_renderer();
    let world_ref = player_data.get_world_ref();
    play_view.link_world_ref(world_ref);
    let cords_ref = player_data.get_mut_location_manager().get_player_cursor_location_cords_ref();
    play_view.link_camera_world_cords_ref(cords_ref);

}

pub fn add_menu_nav_panel(panel: &mut Panel, screen_data: &ScreenData, player_data: &mut PlayerData) {
    let menu_nav_sub_panel = panel.add_sub_panel();
    menu_nav_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

    // Back button
    let button = menu_nav_sub_panel.add_button();
    button.add_event(Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::MapView)));
    button.set_icon(crate::game_data::types::UITextures::MapIcon);
    button.set_text("Map".to_string());

    let button = menu_nav_sub_panel.add_button();
    button.add_event(Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::SettingsMenu)));
    button.set_icon(crate::game_data::types::UITextures::AreaIcon);
    button.set_text("Settings".to_string());

    let button = menu_nav_sub_panel.add_button();
    button.add_event(Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::MainMenu)));
    button.set_icon(crate::game_data::types::UITextures::XIcon);
    button.set_text("Main Menu".to_string());
}

pub fn get_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::TopLeft);
        
        // panel.set_background(crate::game_data::types::UITextures::VoidBackground);
        panel.set_new_background(BackgroundType::Scrolling(UITextures::VoidBackground));
        panel.set_color(PanelColor::Clear);

        // Add panels
        self::add_control_panel(panel, screen_data, player_data);
        self::add_play_view_panel(panel, screen_data, player_data);
        self::add_menu_nav_panel(panel, screen_data, player_data);

        panel.size();
    }

    return panel;

}