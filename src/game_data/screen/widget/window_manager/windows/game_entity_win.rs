use crate::game_data::{player_data::game_entity::game_entity_manager::GameEntityId, screen::{widget::{panel::panel::Panel, widget::{Widget, WidgetType}, widget_properties::WidgetId, window_manager::windows::window_type::{Window, WindowType}}}};

pub struct GameEntityWindow {
    entity_id: GameEntityId,

    scroll_panel_id: WidgetId,
    panel: Panel,
}

impl GameEntityWindow {
    pub fn new(id: GameEntityId) -> GameEntityWindow {
        let mut panel = Panel::new_blank();

        let scroll_panel = panel.add_scroll_panel();
        scroll_panel.set_prefered_scale([0.5; 2]);
        let scroll_panel_id = scroll_panel.get_id();

        GameEntityWindow {
            entity_id: id,
            scroll_panel_id,
            panel,
        }
    }
}

impl Window for GameEntityWindow {
    fn wrap_into_window_type(self) -> super::window_type::WindowType {
        WindowType::GameEntityWindow(self)
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);

        if let Some(WidgetType::ScrollPanel(scroll_panel)) = self.panel.find_widget_with_id(self.scroll_panel_id) {
            if let Some(game_entity) = player_data.game_entity_manager.clone_game_entity(self.entity_id) {
                scroll_panel.clear_widgets();

                if let Some(widget) = game_entity.clone().get_window() {
                    scroll_panel.add_widget(widget);
                }
                else {
                    let components = game_entity.get_components();
                    for c in components {
                        scroll_panel.add_widget(c.into_widget());
                    }
                }
            }
        }
    }

    fn get_mut_panel(&mut self) -> &mut Panel {
        &mut self.panel
    }

    fn get_panel(&self) -> &Panel {
        &self.panel
    }
}
