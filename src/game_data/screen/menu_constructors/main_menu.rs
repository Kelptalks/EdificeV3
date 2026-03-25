use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::GameEvent, render_event_manager::render_event_manager::RenderEvent}, screen::{ScreenData, screen_data::CurrentMenu, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, text::text_input::TextInput, widget::{Widget, WidgetType}, widget_calculations::TextSize}}};


pub fn get_menu(screen_data: &ScreenData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
        panel.set_color(PanelColor::Clear);
        panel.set_new_background(BackgroundType::Static(crate::game_data::types::UITextures::FaceBackground));

        let header = panel.add_text_display("EDIFICE V3".to_string());
        header.set_text_scale(TextSize::ExtraLarge);

        // Play Button
        let bar_button = panel.add_bar_button("Play".to_string());
        bar_button.add_event(GameEvent::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::WorldCreationMenu)));
        bar_button.set_text_scale(TextSize::Large);
        
        // Settings Button
        let bar_button = panel.add_bar_button("Settings".to_string());
        bar_button.add_event(GameEvent::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::SettingsMenu)));
        bar_button.set_text_scale(TextSize::Large);
        
        let bar_button = panel.add_bar_button("Exit".to_string());
        bar_button.add_event(GameEvent::RenderEvent(RenderEvent::QuitGame));
        bar_button.set_text_scale(TextSize::Large);
        

        panel.size();
    }

    return panel;

}