use crate::game_data::{game_event_manager::{game_event_manager::Event, render_event_manager::render_event_manager::RenderEvent}, player_data::player_data::PlayerData, screen::{ScreenData, screen_data::CurrentMenu, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::{Widget, WidgetType}, widget_calculations::TextSize}}};



pub fn get_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Clear);
        panel.set_new_background(BackgroundType::Scrolling(crate::game_data::types::UITextures::MirrorBackground));

        let tab_panel = panel.add_tab_panel();
        let mut tab_sub_panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);
        if let WidgetType::Panel(tab_sub_panel) = &mut tab_sub_panel {
            tab_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
            tab_sub_panel.add_bar_button("test_this_shit".to_string());
            tab_sub_panel.add_bar_button("test_this_shit".to_string());
            tab_sub_panel.add_bar_button("test_this_shit".to_string());
            tab_sub_panel.add_bar_button("test_this_shit".to_string());
            tab_sub_panel.add_bar_button("test_this_shit".to_string());
            tab_sub_panel.size();
        }
        tab_panel.add_panel(tab_sub_panel);
        tab_panel.size();

        let header = panel.add_text_display("Settings".to_string());
        header.set_text_scale(TextSize::ExtraLarge);


        // Back button
        let button = panel.add_button();
        button.add_event(Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::MainMenu)));
        button.set_icon(crate::game_data::types::UITextures::XIcon);
        button.set_text("Main Menu".to_string());

        panel.size();
    }

    return panel;

}