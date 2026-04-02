use crate::game_data::{game_event_manager::{game_event_manager::GameEvent, render_event_manager::render_event_manager::RenderEvent, world_event_manager::world_event_manager::WorldEvent}, screen::{ScreenData, screen_data::CurrentMenu, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::{Widget, WidgetType}, widget_calculations::TextSize}}, types::{BlockTexture, UITextures}, world_gen::world_config::WorldConfig};


pub fn get_menu(screen_data: &ScreenData, world_config: &mut WorldConfig) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        
        // panel.set_background(crate::game_data::types::UITextures::VoidBackground);
        panel.set_new_background(BackgroundType::Scrolling(UITextures::VoidBackground));
        panel.set_color(PanelColor::Clear);

        // Menu Tittle
        let text_display = panel.add_text_display("World Creation".to_string());
        text_display.set_text_scale(TextSize::ExtraLarge);

        // Config panel
        let config_panel = panel.add_sub_panel();
        config_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        config_panel.set_color(PanelColor::Dark);
        
            let text_display = config_panel.add_text_display("World Settings".to_string());
            text_display.set_text_scale(TextSize::Large);

            let world_toggle_panel = config_panel.add_sub_panel();
            world_toggle_panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);

            let toggle_button = world_toggle_panel.add_toggle_button();
            world_config.set_flat_world_toggle_link(toggle_button.get_toggle_ref());
            toggle_button.set_block(BlockTexture::Grass);
            toggle_button.set_text("World Flat".to_string());

            // Create world button
            let button = config_panel.add_bar_button("Create World".to_string());
            button.add_event(GameEvent::WorldEvent(WorldEvent::GenWorld()));
            button.add_event(GameEvent::RenderEvent(RenderEvent::InitWorldRender()));
            button.add_event(GameEvent::RenderEvent(RenderEvent::ChangeMenu(CurrentMenu::PlayView)));
            button.set_text_scale(TextSize::Large);

        // Back button
        let button = panel.add_button();
        button.add_left_click_event(RenderEvent::ChangeMenu(CurrentMenu::MainMenu).wrap_into_event());
        button.set_icon(crate::game_data::types::UITextures::XIcon);
        button.set_text("Main Menu".to_string());
        
        panel.size();
    }

    return panel;

}