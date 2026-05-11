use crate::game_data::{player_data::game_object::game_object_manager::GameObjectId, screen::{ui_elements::panel, widget::{panel::panel::Panel, widget::{Widget, WidgetType}, widget_properties::WidgetId, window_manager::windows::window_type::{Window, WindowType}}}};

pub struct GameObjectWindow {
    object_id: GameObjectId,

    scroll_panel_id: WidgetId,
    panel: Panel,
}

impl GameObjectWindow {
    pub fn new(id: GameObjectId) -> GameObjectWindow {
        let mut panel = Panel::new_blank();

        let scroll_panel = panel.add_scroll_panel();
        scroll_panel.set_prefered_scale([0.5; 2]);
        let scroll_panel_id = scroll_panel.get_id();


        GameObjectWindow {
            object_id: id,

            scroll_panel_id,
            panel: panel
        }
    }
}

impl Window for GameObjectWindow {
    fn wrap_into_window_type(self) -> super::window_type::WindowType {
        WindowType::GameObjectWindow(self)
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
            if let Some(game_object) = player_data.game_object_manager.clone_game_object(self.object_id) {
                scroll_panel.clear_widgets();
                
                let traits = game_object.get_traits();
                for t in traits {
                    scroll_panel.add_widget(t.into_widget());
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