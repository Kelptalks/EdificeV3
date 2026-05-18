use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    screen::{
        menu_constructors::play_view_menu::selection_panel::selection_panel::{
            get_block_selection_panel, get_item_selection_panel,
        },
        widget::{
            panel::panel::Panel,
            prelude::TabPanel,
            widget::{Widget, WidgetType},
            window_manager::windows::window_type::{Window, WindowType},
        },
    },
    types::UITextures,
};

pub struct BlockSelectWindow {
    panel: Panel,
}

impl BlockSelectWindow {
    pub fn new() -> BlockSelectWindow {
        let tab_index = Rc::new(RefCell::new(0usize));
        let mut panel = Panel::new_blank();
        let mut tab_panel = TabPanel::new(&tab_index);

        let btn = tab_panel.add_panel(get_block_selection_panel());
        btn.set_text("Blocks".to_string());
        btn.set_icon(UITextures::BlockVarIcon);

        let btn = tab_panel.add_panel(get_item_selection_panel());
        btn.set_text("Items".to_string());
        btn.set_icon(UITextures::ItemVarIcon);

        panel.add_widget(tab_panel.wrap_into_widget());

        BlockSelectWindow { panel }
    }
}

impl Window for BlockSelectWindow {
    fn wrap_into_window_type(self) -> WindowType {
        WindowType::BlockSelect(self)
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        self.panel.render(texture_manager, screen_data, event_manager, player_data);
    }

    fn get_mut_panel(&mut self) -> &mut Panel {
        &mut self.panel
    }

    fn get_panel(&self) -> &Panel {
        &self.panel
    }
}
