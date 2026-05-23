use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    game_event_manager::{game_event_manager::GameEvent, prelude::PlayerDataEvent},
    player_data::cursor::cursor_event_scheduler::CursorEvent,
    screen::widget::{
        panel::panel::Panel,
        widget::Widget,
        window_manager::windows::window_type::{Window, WindowType},
    },
};

pub struct SettingsMenuWidget {
    panel: Panel,
    xy_text_ref: Rc<RefCell<String>>,
    z_text_ref: Rc<RefCell<String>>,
}

impl SettingsMenuWidget {
    pub fn new(back_event: GameEvent) -> SettingsMenuWidget {
        use crate::game_data::screen::widget::panel::panel::{PanelAlignment, PanelOrientation};
        use crate::game_data::screen::widget::widget_calculations::TextSize;

        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);

        // Back button
        let back_btn = panel.add_bar_button("Back".to_string());
        back_btn.add_event(back_event);
        back_btn.set_text_scale(TextSize::Large);

        panel.add_text_display("View Distance".to_string()).set_text_scale(TextSize::Medium);

        // XY row
        let xy_row = panel.add_sub_panel();
        xy_row.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        xy_row.add_text_display("XY:".to_string());
        let xy_dec = xy_row.add_bar_button("-".to_string());
        xy_dec.add_event(GameEvent::PlayerDataEvent(PlayerDataEvent::CursorEvent(
            CursorEvent::ModChunkLoadDistance([-1, -1, 0])
        )));
        let xy_text = xy_row.add_text_display("5".to_string());
        let xy_text_ref = xy_text.get_string_ref().clone();
        let xy_inc = xy_row.add_bar_button("+".to_string());
        xy_inc.add_event(GameEvent::PlayerDataEvent(PlayerDataEvent::CursorEvent(
            CursorEvent::ModChunkLoadDistance([1, 1, 0])
        )));

        // Z row
        let z_row = panel.add_sub_panel();
        z_row.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        z_row.add_text_display("Z: ".to_string());
        let z_dec = z_row.add_bar_button("-".to_string());
        z_dec.add_event(GameEvent::PlayerDataEvent(PlayerDataEvent::CursorEvent(
            CursorEvent::ModChunkLoadDistance([0, 0, -1])
        )));
        let z_text = z_row.add_text_display("2".to_string());
        let z_text_ref = z_text.get_string_ref().clone();
        let z_inc = z_row.add_bar_button("+".to_string());
        z_inc.add_event(GameEvent::PlayerDataEvent(PlayerDataEvent::CursorEvent(
            CursorEvent::ModChunkLoadDistance([0, 0, 1])
        )));

        SettingsMenuWidget {
            panel,
            xy_text_ref,
            z_text_ref,
        }
    }
}

impl Window for SettingsMenuWidget {
    fn wrap_into_window_type(self) -> WindowType {
        WindowType::SettingsMenu(self)
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        let dist = player_data.get_cursor().get_chunk_load_distance();
        *self.xy_text_ref.borrow_mut() = format!("{}", dist[0]);
        *self.z_text_ref.borrow_mut() = format!("{}", dist[2]);

        self.panel.set_parent_pos(screen_data.get_viewport_uv());
        self.panel.size();
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }

    fn get_mut_panel(&mut self) -> &mut Panel {
        &mut self.panel
    }

    fn get_panel(&self) -> &Panel {
        &self.panel
    }
}
