use crate::game_data::{drone_programming::var::{game_vars::game_var_type::{GameVar, GameVarTypeKind}, var_type::{Var, VarTypeKind}}, game_event_manager::{game_event_manager::GameEvent, render_event_manager::render_event_manager::RenderEvent}, player_data::player_data::PlayerData, screen::{ScreenData, screen_data::CurrentMenu, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::{Widget, WidgetType}, widget_calculations::TextSize}}, types::{BlockTexture, drone_item::DroneItem}};




pub fn get_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Clear);
        panel.set_new_background(BackgroundType::Scrolling(crate::game_data::types::UITextures::MirrorBackground));

        let header = panel.add_text_display("Settings".to_string());
        header.set_text_scale(TextSize::ExtraLarge);


        // Back button
        let button = panel.add_button();
        button.add_event(GameEvent::RenderEvent(RenderEvent::ChangeMenu(screen_data.get_current_menu())));
        button.set_icon(crate::game_data::types::UITextures::XIcon);
        button.set_text("Back".to_string());

        panel.size();
    }

    return panel;

}