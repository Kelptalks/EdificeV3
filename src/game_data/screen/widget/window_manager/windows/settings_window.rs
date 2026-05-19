use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    player_data::cursor::cursor_event_scheduler::CursorEvent,
    screen::widget::{
        panel::panel::{Panel, PanelAlignment, PanelOrientation},
        widget::Widget,
        window_manager::windows::window_type::{Window, WindowType},
        widget_calculations::TextSize,
    },
    types::UITextures,
};

pub struct SettingsWindow {
    panel: Panel,
    chunk_load_text_ref: Rc<RefCell<String>>,
}

impl SettingsWindow {
    pub fn new() -> SettingsWindow {
        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);

        let sub = panel.add_sub_panel();
        sub.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);

        let label = sub.add_text_display("Chunk Load".to_string());
        label.set_text_scale(TextSize::Small);

        let mod_btn = sub.add_button();
        mod_btn.set_icon(UITextures::ModIcon);
        mod_btn.add_left_click_event(CursorEvent::SetChunkLoadDistance([7, 7, 3]).wrap_into_event());
        mod_btn.add_right_click_event(CursorEvent::SetChunkLoadDistance([3, 3, 1]).wrap_into_event());

        let text_display = sub.add_text_display("X:5 Y:5 Z:2".to_string());
        let chunk_load_text_ref = text_display.get_string_ref().clone();

        panel.size();

        SettingsWindow {
            panel,
            chunk_load_text_ref,
        }
    }
}

impl Window for SettingsWindow {
    fn wrap_into_window_type(self) -> WindowType {
        WindowType::Settings(self)
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        let dist = player_data.get_cursor().get_chunk_load_distance();
        *self.chunk_load_text_ref.borrow_mut() = format!("X:{} Y:{} Z:{}", dist[0], dist[1], dist[2]);

        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }

    fn get_mut_panel(&mut self) -> &mut Panel {
        &mut self.panel
    }

    fn get_panel(&self) -> &Panel {
        &self.panel
    }
}
