use std::{cell::RefCell, rc::Rc};

use crate::game_data::{screen::widget::{panel::panel::{PanelAlignment, PanelOrientation}, prelude::TabPanel, text::text_input::{self, TextInput}, widget::WidgetType}, types::UITextures};


fn get_cord_creation_widget() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        let x_string_ref = Rc::new(RefCell::new("000000".to_string()));
        let text_input = TextInput::new_text_input(&x_string_ref);
        panel.add_widget(text_input.wrap_into_widget());

        let x_string_ref = Rc::new(RefCell::new("000000".to_string()));
        let text_input = TextInput::new_text_input(&x_string_ref);
        panel.add_widget(text_input.wrap_into_widget());

        let x_string_ref = Rc::new(RefCell::new("000000".to_string()));
        let text_input = TextInput::new_text_input(&x_string_ref);
        panel.add_widget(text_input.wrap_into_widget());

    }

    panel
}

pub fn get_var_create_widget() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        let current_panel_index = Rc::new(RefCell::new(0));
        let mut tab_panel = TabPanel::new(&current_panel_index);



        let button = tab_panel.add_panel(get_cord_creation_widget());
        

        panel.add_widget(tab_panel.wrap_into_widget());

        panel.size();

    }

    return panel;
}