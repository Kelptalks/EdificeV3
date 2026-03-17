use crate::game_data::{game_event_manager::{game_event_manager::Event, render_event_manager::render_event_manager::RenderEvent}, player_data::player_data::PlayerData, screen::{ScreenData, screen_data::CurrentMenu, widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::WidgetType, widget_calculations::TextSize}}, types::UITextures};

//=====================================
// Selection Panel
//=====================================

pub fn get_block_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Header
        let text_display = panel.add_text_display("Blocks".to_string());
        text_display.set_text_scale(TextSize::Medium);

        let scroll_panel = panel.add_scroll_panel();
        scroll_panel.set_prefered_scale([0.5, 0.7]);
        for i in 0..10 {
            let mut test_panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
            if let WidgetType::Panel(test_panel) = &mut test_panel {
                test_panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
                test_panel.add_button();
                test_panel.add_button();
                test_panel.add_button();

            }
            scroll_panel.add_panel(test_panel);
        }
        panel.size();
    }

    return panel;
}

pub fn get_item_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Header
        let text_display = panel.add_text_display("Items".to_string());
        text_display.set_text_scale(TextSize::Medium);
    }

    return panel;
}

pub fn get_location_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Header
        let text_display = panel.add_text_display("Locations".to_string());
        text_display.set_text_scale(TextSize::Medium);

        
    }

    return panel;
}

pub fn get_drone_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Header
        let text_display = panel.add_text_display("Drones".to_string());
        text_display.set_text_scale(TextSize::Medium);
    }

    return panel;
}

pub fn add_selection_menu(panel: &mut Panel, screen_data: &ScreenData, player_data: &mut PlayerData) {
    let selection_sub_panel = panel.add_sub_panel();
    selection_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

    // Header
    let text_display = selection_sub_panel.add_text_display("Controls".to_string());
    text_display.set_text_scale(TextSize::Large);


    let selection_tab_panel = selection_sub_panel.add_tab_panel();

    // Add Block selection
    let button = selection_tab_panel.add_panel(get_block_selection_panel());
    button.set_text("Blocks".to_string());
    button.set_block(crate::game_data::types::BlockTexture::Grass);

    // Add Item selection
    let button = selection_tab_panel.add_panel(get_item_selection_panel());
    button.set_text("Items".to_string());
    button.set_icon(UITextures::ScallingIconMidCenter);

    // Add Location Selection
    let button = selection_tab_panel.add_panel(get_location_selection_panel());
    button.set_text("Locations".to_string());
    button.set_icon(UITextures::AreaIcon);

    // Add Drone Selection
    let button = selection_tab_panel.add_panel(get_drone_selection_panel());
    button.set_text("Drones".to_string());
    button.set_block(crate::game_data::types::BlockTexture::DroneBotRight);

}

//=====================================
// Control interface
//=====================================

pub fn get_location_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Header
        let text_display = panel.add_text_display("Location Manager".to_string());
        text_display.set_text_scale(TextSize::Medium);
    }

    return panel;
}

pub fn get_drone_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Header
        let text_display = panel.add_text_display("Drone Controller".to_string());
        text_display.set_text_scale(TextSize::Medium);
    }

    return panel;
}

pub fn get_building_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Header
        let text_display = panel.add_text_display("Building".to_string());
        text_display.set_text_scale(TextSize::Medium);

        // 
        let button = panel.add_button();
    }

    return panel;
}

pub fn add_control_panel(panel: &mut Panel, screen_data: &ScreenData, player_data: &mut PlayerData) {
    let controls_sub_panel = panel.add_sub_panel();
    
    controls_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
    
    // Header
    let text_display = controls_sub_panel.add_text_display("Controls".to_string());
    text_display.set_text_scale(TextSize::Large);


    let control_tab_panel = controls_sub_panel.add_tab_panel();

    // Add Drone Controls Tab
    let button = control_tab_panel.add_panel(get_drone_selection_panel());
    button.set_text("drones".to_string());
    button.set_block(crate::game_data::types::BlockTexture::DroneBotRight);

    // Add Location Manager Tab
    let button = control_tab_panel.add_panel(get_location_panel());
    button.set_text("locations".to_string());
    button.set_icon(UITextures::AreaIcon);

    // Add Location Manager Tab
    let button = control_tab_panel.add_panel(get_building_panel());
    button.set_text("Building".to_string());
    button.set_icon(UITextures::BluePrintIcon);
    
}

//=====================================
// Play View Panel
//=====================================

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

//=====================================
// Menu Nav Panel
//=====================================


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

//=====================================
// Root panel
//=====================================

pub fn get_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::TopLeft);
        
        // panel.set_background(crate::game_data::types::UITextures::VoidBackground);
        panel.set_new_background(BackgroundType::Scrolling(UITextures::VoidBackground));
        panel.set_color(PanelColor::Clear);

        // Add panels
        self::add_menu_nav_panel(panel, screen_data, player_data);
        self::add_control_panel(panel, screen_data, player_data);
        self::add_selection_menu(panel, screen_data, player_data);
        self::add_play_view_panel(panel, screen_data, player_data);

        panel.size();
    }

    return panel;

}