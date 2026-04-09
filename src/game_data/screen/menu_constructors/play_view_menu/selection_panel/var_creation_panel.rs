use std::{cell::RefCell, rc::Rc};

use crate::game_data::{screen::widget::{panel::panel::{PanelAlignment, PanelOrientation}, prelude::TabPanel, text::text_input::{self, TextInput}, widget::WidgetType}, types::UITextures};



pub fn get_var_create_widget() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        let current_panel_index = Rc::new(RefCell::new(0));
        let mut tab_panel = TabPanel::new(&current_panel_index);


        

        panel.add_widget(tab_panel.wrap_into_widget());

        panel.size();

    }

    return panel;
}