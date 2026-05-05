use crate::game_data::{screen::{ui_elements::panel, widget::{panel::panel::Panel, prelude::TabPanel, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_properties::WidgetId, window_manager::windows::window_type::Window}}};

pub struct DebugWin {
    panel: Panel,



    scroll_panel_id: WidgetId
}

impl DebugWin {
    pub fn new() -> DebugWin {
        let mut panel = Panel::new_blank();



        let scroll_panel = panel.add_scroll_panel();
        let scroll_panel_id = scroll_panel.get_id();


        DebugWin {
            panel,
            scroll_panel_id
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

        if let Some(scroll_panel) = self.panel.find_widget_with_id(self.scroll_panel_id) {
            if let WidgetType::ScrollPanel(scroll_panel) = scroll_panel {
                scroll_panel.clear_widgets();
                let debug_data = game_event_manager.get_mut_debug_data().get_rendering_debug_data();

                let strings = debug_data.to_string_vec();
                for string in strings {
                    let text_display = TextDisplay::new(string);
                    scroll_panel.add_widget(text_display.wrap_into_widget());
                }
                
            }
        }
        self.panel.size();
        
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }

    
    
}