use crate::game_data::screen::{ui_elements::panel, widget::{panel::panel::Panel, widget::{Widget, WidgetType}}};

pub struct ScriptingPanel {
    panel : Panel,


}

impl ScriptingPanel {
    pub fn new() -> ScriptingPanel {
        let mut panel = Panel::new_blank();

        panel.add_text_display("test this shit".to_string());

        ScriptingPanel {
            panel: panel,
            
            
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        return WidgetType::ScriptingPanel(self)
    }
}

impl Widget for ScriptingPanel {
    fn get_pos(&self) -> [f32; 4] {
        self.panel.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.panel.get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.panel.get_preffered_scale()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.panel.set_buffers(pos)
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.panel.set_parent_pos(pos)
    }

    fn size(&mut self) {
        self.panel.size()
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager);
    }
}