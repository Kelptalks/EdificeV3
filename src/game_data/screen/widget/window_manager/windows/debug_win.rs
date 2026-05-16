use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    screen::widget::{
        panel::panel::{Panel, PanelAlignment, PanelOrientation},
        prelude::TabPanel,
        text::header::TextDisplay,
        widget::{Widget, WidgetType},
        widget_properties::WidgetId,
        window_manager::windows::window_type::Window,
    },
};

pub struct DebugWin {
    panel: Panel,
    tab_index: Rc<RefCell<usize>>,
    last_categories: Vec<String>,
    scroll_panel_ids: Vec<WidgetId>,
}

impl DebugWin {
    pub fn new() -> DebugWin {
        DebugWin {
            panel: Panel::new_blank(),
            tab_index: Rc::new(RefCell::new(0)),
            last_categories: Vec::new(),
            scroll_panel_ids: Vec::new(),
        }
    }

    fn rebuild_panel(&mut self, categories: &[String]) {
        let mut panel = Panel::new_blank();
        let mut tab_panel = TabPanel::new(&self.tab_index);

        self.scroll_panel_ids.clear();
        for cat in categories {
            let mut inner = Panel::new_blank();
            inner.add_text_display(cat.clone());
            inner.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

            let scroll = inner.add_scroll_panel();
            scroll.set_prefered_scale([0.5; 2]);
            self.scroll_panel_ids.push(scroll.get_id());

            let btn = tab_panel.add_panel(inner.wrap_into_widget());
            btn.set_text(cat.clone());
        }

        panel.add_widget(tab_panel.wrap_into_widget());
        self.panel = panel;
    }
}

impl Window for DebugWin {
    fn wrap_into_window_type(self) -> super::window_type::WindowType {
        super::window_type::WindowType::Debug(self)
    }

    fn get_mut_panel(&mut self) -> &mut Panel {
        &mut self.panel
    }

    fn get_panel(&self) -> &Panel {
        &self.panel
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        let debug_data = game_event_manager.get_mut_debug_data();

        let current_cats: Vec<String> = debug_data.categories().map(|s| s.to_string()).collect();
        if current_cats != self.last_categories {
            self.last_categories = current_cats.clone();
            self.rebuild_panel(&current_cats);
        }

        for (cat, widget_id) in self.last_categories.iter().zip(self.scroll_panel_ids.iter()) {
            if let Some(WidgetType::ScrollPanel(sp)) = self.panel.find_widget_with_id(*widget_id) {
                sp.clear_widgets();
                for line in debug_data.get_data(cat) {
                    sp.add_widget(TextDisplay::new(line.clone()).wrap_into_widget());
                }
            }
        }

        self.panel.size();
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }
}
