use std::{cell::RefCell, rc::Rc};

use crate::game_data::screen::widget::widget::WidgetType;

pub struct WidgetUpdateManager {
    widgets_to_add: Vec<WidgetType>,

}

impl WidgetUpdateManager {
    pub fn new() -> Rc<RefCell<WidgetUpdateManager>> {
        let update_manager = WidgetUpdateManager {
            widgets_to_add: Vec::new()
        };

        Rc::new(RefCell::new(update_manager))
    }

    pub fn add_widget(&mut self, widget: WidgetType) {
        self.widgets_to_add.push(widget);
    }

    pub fn get_widgets_to_add(&mut self) -> &mut Vec<WidgetType> {
        return &mut self.widgets_to_add;
    }
}