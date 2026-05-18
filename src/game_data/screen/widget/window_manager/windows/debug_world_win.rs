use crate::game_data::{
    screen::widget::{
        panel::panel::Panel,
        widget::{Widget, WidgetType},
        widget_properties::WidgetId,
        window_manager::windows::window_type::{Window, WindowType},
    },
    world::world::WorldEvent,
};

pub struct DebugWorldWindow {
    panel: Panel,

    unload_chunk_btn: WidgetId,
    toggle_borders_btn: WidgetId,
}

impl DebugWorldWindow {
    pub fn new() -> DebugWorldWindow {
        let mut panel = Panel::new_blank();

        let unload_btn = panel.add_button();
        unload_btn.set_text("Unload Cursor Chunk".to_string());
        let unload_chunk_btn = unload_btn.get_id();

        let borders_btn = panel.add_button();
        borders_btn.set_text("Toggle Chunk Borders".to_string());
        let toggle_borders_btn = borders_btn.get_id();

        DebugWorldWindow {
            panel,
            unload_chunk_btn,
            toggle_borders_btn,
        }
    }
}

impl Window for DebugWorldWindow {
    fn wrap_into_window_type(self) -> WindowType {
        WindowType::DebugWorld(self)
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        self.panel.render(texture_manager, screen_data, event_manager, player_data);

        if screen_data.was_left_pressed() {
            if let Some(WidgetType::Button(btn)) = self.panel.find_widget_with_id(self.unload_chunk_btn) {
                if btn.mouse_on(screen_data) {
                    let cords = player_data.get_cursor().get_cords();
                    event_manager.add_world_event(WorldEvent::UnloadChunkAtCords(cords));
                }
            }

            if let Some(WidgetType::Button(btn)) = self.panel.find_widget_with_id(self.toggle_borders_btn) {
                if btn.mouse_on(screen_data) {
                    event_manager.add_world_event(WorldEvent::ToggleChunkBorders);
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
