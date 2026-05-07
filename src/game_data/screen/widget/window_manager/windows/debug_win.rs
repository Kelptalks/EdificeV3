use std::{cell::RefCell, rc::Rc};

use crate::game_data::{screen::{ui_elements::panel, widget::{panel::{image_display_panel::{self, ImageDisplayPanel}, panel::{Panel, PanelAlignment, PanelOrientation}}, prelude::TabPanel, tab_panel, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_properties::WidgetId, window_manager::windows::window_type::Window}}, texture_manager::{texture::Texture, texture_cashe::texture_cashe::CashedTextureID}, types::BlockTexture};

pub struct DebugWin {
    panel: Panel,

    cashed_texture: [Option<CashedTextureID>; 64],

    tab_panel_id: WidgetId,
    atlas_panel_id: WidgetId,
    rendering_data_id: WidgetId,
}

impl DebugWin {
    pub fn new() -> DebugWin {
        let mut panel = Panel::new_blank();



        let current_current_panel_index_ref = Rc::new(RefCell::new(0));
        let mut tab_panel = TabPanel::new(&current_current_panel_index_ref);


        // Atlas
        let mut atlas_panel = Panel::new_blank();
        atlas_panel.add_text_display("atlas viewer".to_string());
        atlas_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        let scroll_panel = atlas_panel.add_scroll_panel();

        let image_display_panel = 
            ImageDisplayPanel::new(
                Texture::BlockTexture(BlockTexture::Battery1), 
                [0.1, 0.1]
            );
        
        scroll_panel.add_widget(image_display_panel.wrap_into_widget());

        let image_display_panel = 
            ImageDisplayPanel::new(
                Texture::Atlas(0), 
                [1.0, 1.0]
            );
        
        scroll_panel.add_widget(image_display_panel.wrap_into_widget());


        let atlas_panel_id = atlas_panel.get_id();
        let atlas_tab_button = tab_panel.add_panel(atlas_panel.wrap_into_widget());
        atlas_tab_button.set_text("Texture Atlas Viewer".to_string());
        atlas_tab_button.set_icon(crate::game_data::types::UITextures::MapIcon);
        


        // Rendering Data
        let mut rendering_data_panel = Panel::new_blank();
        rendering_data_panel.add_text_display("Rendering Data".to_string());
        rendering_data_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        let scroll_panel = rendering_data_panel.add_scroll_panel();
        

        let rendering_data_id = scroll_panel.get_id();
        let rendering_data_tab_button = tab_panel.add_panel(rendering_data_panel.wrap_into_widget());
        rendering_data_tab_button.set_text("Rendering Data".to_string());
        rendering_data_tab_button.set_icon(crate::game_data::types::UITextures::CameraIcon);

        let tab_panel_id = tab_panel.get_id();
        panel.add_widget(tab_panel.wrap_into_widget());




        DebugWin {
            panel: panel,

            cashed_texture: [None; 64],
            tab_panel_id,
            atlas_panel_id,
            rendering_data_id,
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


        let debug_data = game_event_manager.get_mut_debug_data().get_rendering_debug_data();
        if let Some(WidgetType::ScrollPanel(rendering_data_panel)) = self.panel.find_widget_with_id(self.rendering_data_id) {
            rendering_data_panel.clear_widgets();

            let strings = debug_data.to_string_vec();
            for string in strings {
                let text_display = TextDisplay::new(string);
                rendering_data_panel.add_widget(text_display.wrap_into_widget());
            }
                
            

        }
        self.panel.size();
        

        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);


    }

    
    
}