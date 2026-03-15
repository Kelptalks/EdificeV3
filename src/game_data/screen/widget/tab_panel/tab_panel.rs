use crate::game_data::{screen::widget::{button::button::Button, panel::panel::Panel, widget::{Widget, WidgetType}, widget_calculations}, types::UITextures};

pub struct TabPanel {
    button_panel: Panel,
    
    sub_panels: Vec<WidgetType>,

    // Parent 
    prefered_scale: [f32; 2],
    parent_pos: [f32; 4],
    external_buffers: [f32; 4],

    // Self
    internal_buffers: [f32; 4],
    pos: [f32; 4],
}



impl TabPanel {
    pub fn new() -> TabPanel {
        TabPanel {
            button_panel: Panel::new([0.0; 4], [0.0; 4]),
            sub_panels: Vec::new(),

            prefered_scale: [0.0; 2],
            parent_pos: [0.0; 4],
            external_buffers: [0.0; 4],


            pos: [0.0; 4],
            internal_buffers: [0.0; 4],
        }
    }


    pub fn add_panel(&mut self, panel: WidgetType) -> &mut Button { 
        self.sub_panels.push(panel);
        return self.button_panel.add_button();

    }

}


impl Widget for TabPanel {
    fn get_pos(&self) -> [f32; 4] {
        self.button_panel.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.button_panel.get_scale()
    }

    fn get_prefered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;

    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
    }

    fn size(&mut self) {
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);

        // Size button first
        let button_prefred_scale = self.button_panel.get_prefered_scale();
        let sub_panel_prefered_scale = self.sub_panels[0].get_prefered_scale();
        self.prefered_scale = [
            button_prefred_scale[0] + sub_panel_prefered_scale[0],
            button_prefred_scale[1] + sub_panel_prefered_scale[1],
        ];

        for sub_panel in &mut self.sub_panels {
            sub_panel.set_parent_pos(self.pos);
            sub_panel.set_buffers(self.internal_buffers);
            sub_panel.size();
        }
    }

    fn render(
        &mut self, 
        texture_manager: &mut crate::game_data::TextureManager, 
        screen_data: &crate::game_data::screen::ScreenData, 
        game_event_manager: &mut crate::game_data::game_event_manager::game_event_manager::GameEventManager
    ) {
        self.button_panel.render(texture_manager, screen_data, game_event_manager);
        texture_manager.render_ui_element_with_pos(UITextures::ScallingIconMidCenter, self.pos);

        if self.sub_panels.len() > 0 {
            self.sub_panels[0].render(texture_manager, screen_data, game_event_manager);
        }
    }
}