use crate::game_data::{game_event_manager::{game_event_manager::Event, render_event_manager::render_event_manager::RenderEvent}, player_data::player_data::PlayerData, screen::{ScreenData, screen_data::CurrentMenu, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::WidgetType}}, types::UITextures};

pub fn get_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::TopLeft);
        
        // panel.set_background(crate::game_data::types::UITextures::VoidBackground);
        panel.set_new_background(BackgroundType::Scrolling(UITextures::VoidBackground));
        panel.set_color(PanelColor::Clear);

        // Controls sub panel
        let controls_sub_panel = panel.add_sub_panel();
        controls_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        controls_sub_panel.add_text_display("Controls".to_string());
        
        // Init play view
        let play_view_sub_panel = panel.add_sub_panel();
        play_view_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        play_view_sub_panel.add_text_display("play view".to_string());

        let play_view = play_view_sub_panel.add_play_world_view_renderer();
        
        let world_ref = player_data.get_world_ref();
        play_view.link_world_ref(world_ref);
        
        let cords_ref = player_data.get_mut_location_manager().get_player_cursor_location_cords_ref();
        play_view.link_camera_world_cords_ref(cords_ref);



        // Back button
        let button = panel.add_button();
        button.add_event(Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::MainMenu)));
        button.set_icon(crate::game_data::types::UITextures::XIcon);
        button.set_text("Main Menu".to_string());

        let button = panel.add_button();
        button.add_event(Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::SettingsMenu)));
        button.set_icon(crate::game_data::types::UITextures::AreaIcon);
        button.set_text("Settings".to_string());

        panel.size();
    }

    return panel;

}