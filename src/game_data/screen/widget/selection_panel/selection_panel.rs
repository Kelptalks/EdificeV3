use std::{cell::RefCell, rc::Rc};

use crate::game_data::screen::widget::{scroll_panel::scroll_panel::ScrollPanel, selection_panel::selection_panel_config::WidgetUpdateManager, widget::{Widget, WidgetType}};

pub struct SelectionPanel {
    scroll_panel: ScrollPanel,

    update_manager: Rc<RefCell<WidgetUpdateManager>>,
}

impl SelectionPanel {
    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::SelectionPanel(self)
    }

    pub fn new(update_manager: Rc<RefCell<WidgetUpdateManager>>) -> SelectionPanel {
        SelectionPanel {
            scroll_panel: ScrollPanel::new(), 
            update_manager, 
        }
    }

 
    pub fn get_update_manager(&self) -> &Rc<RefCell<WidgetUpdateManager>> {
        return &self.update_manager;
    }

    

}

impl Widget for SelectionPanel {
    fn get_pos(&self) -> [f32; 4] {
        self.scroll_panel.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.scroll_panel.get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.scroll_panel.get_preffered_scale()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.scroll_panel.set_buffers(pos);
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.scroll_panel.set_parent_pos(pos);
    }

    fn size(&mut self) {
        self.scroll_panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        bounds: Option<[f32; 4]>,
    ) {
        // Add new widgets
        let mut binding = self.update_manager.borrow_mut();
        let widgets_to_add = binding.get_widgets_to_add();

        for widget in widgets_to_add.pop() {
            self.scroll_panel.add_widget(widget);
        }

        drop(binding);
        
        
        self.size();
        self.scroll_panel.render(texture_manager, screen_data, game_event_manager, bounds);

        

    }
}

