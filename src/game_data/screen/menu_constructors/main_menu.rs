use crate::game_data::{game_event_manager::{game_event_manager::Event, render_event_manager::render_event_manager::RenderEvent}, screen::{ScreenData, screen_data::CurrentMenu, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget::{Widget, WidgetType}, widget_calculations::TextSize}}};


pub fn get_main_menu(screen_data: &ScreenData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
        panel.set_color(PanelColor::Clear);
        let header = panel.add_header("EDIFICE".to_string());
        header.set_text_scale(TextSize::ExtraLarge);


        let bar_button = panel.add_bar_button("Play".to_string(), Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::WorldCreationMenu)));
        bar_button.set_text_scale(TextSize::Large);

        let bar_button = panel.add_bar_button("BluePrints".to_string(), Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::LevelSelectMenu)));
        bar_button.set_text_scale(TextSize::Large);
        
        let bar_button = panel.add_bar_button("Settings".to_string(), Event::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::WorldCreationMenu)));
        bar_button.set_text_scale(TextSize::Large);
        
        let bar_button = panel.add_bar_button("Exit".to_string(), Event::RenderEvent(RenderEvent::QuitGame));
        bar_button.set_text_scale(TextSize::Large);
        
        panel.size();
    }

    return panel;

}