use crate::game_data::{game_event_manager::{game_event_manager::Event, render_event_manager::render_event_manager::RenderEvent, world_event_manager::world_event_manager::WorldEvent}, screen::{ScreenData, screen_data::CurrentMenu, widget::{button, panel::{panel::{PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget::{Widget, WidgetType}, widget_calculations::TextSize}}};


pub fn get_menu(screen_data: &ScreenData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);


        // Back button
        let button = panel.add_button();
        button.add_event(Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::MainMenu)));
        button.set_icon(crate::game_data::types::UITextures::XIcon);

        // Menu Tittle
        let text_display = panel.add_text_display("World Creation Menu".to_string());
        text_display.set_text_scale(TextSize::Large);

        // Config panel
        let config_panel = panel.add_sub_panel();

        // Create world button
        let button = config_panel.add_bar_button("Create World".to_string());
        button.add_event(Event::WorldEvent(WorldEvent::GenWorld()));
        button.add_event(Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::PlayView)));
        button.set_text_scale(TextSize::Medium);

        panel.size();
    }

    println!("Created World Creation Menu");

    return panel;

}