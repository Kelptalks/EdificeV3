use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::debug_data::debug_data::DebugData, screen::{ui_elements::panel, widget::{panel::{image_display_panel::{self, ImageDisplayPanel}, panel::{Panel, PanelAlignment, PanelOrientation}}, prelude::TabPanel, tab_panel, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_properties::WidgetId, window_manager::windows::window_type::Window}}, texture_manager::{texture::Texture, texture_cashe::texture_cashe::CashedTextureID}, types::BlockTexture};

pub struct DebugWin {
    panel: Panel,

    cashed_texture: [Option<CashedTextureID>; 64],

}

impl DebugWin {
    pub fn new(debug_data: &mut DebugData) -> DebugWin {
        let mut panel = Panel::new_blank();



        let current_current_panel_index_ref = Rc::new(RefCell::new(0));
        let mut tab_panel = TabPanel::new(&current_current_panel_index_ref);

        for tab in &debug_data.debug_data_tabs_names {
            let mut panel = Panel::new_blank();

            panel.add_text_display(tab.to_string());
            panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
            
            
            let scroll_panel = panel.add_scroll_panel();
            debug_data.debug_data_widget_ids.push(scroll_panel.get_id());
            
            let tab_button = tab_panel.add_panel(panel.wrap_into_widget());
            tab_button.set_text(tab.to_string());
        }

        panel.add_widget(tab_panel.wrap_into_widget());




        DebugWin {
            panel: panel,

            cashed_texture: [None; 64],
        }
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

        for (i, widget) in debug_data.debug_data_widget_ids.iter().enumerate() {
            if let Some(WidgetType::ScrollPanel(debug_widget)) = self.panel.find_widget_with_id(*widget) {
                debug_widget.clear_widgets();
                let data = debug_data.get_data(i);
                
                for str in data {
                    let text = TextDisplay::new(str.to_string());
                    debug_widget.add_widget(text.wrap_into_widget());
                }


            }
        }

        self.panel.size();
        

        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);


    }

    
    
}