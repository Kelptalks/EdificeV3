use crate::game_data::{player_data::drone_script::control_flow::condition::Condition, screen::{ui_elements::panel, widget::{drone_programming::scripting_widget_type::ScriptingElementWidget, panel::panel::Panel, prelude::PanelColor, widget::Widget}}};

pub struct ConditionSlot {
    panel: Panel
}

impl ConditionSlot {
    pub fn new(condition: &Condition) -> ConditionSlot {
        let mut panel = Panel::new_blank();
     
        

        ConditionSlot {
            panel
        }
    }
}


impl Widget for ConditionSlot {
    fn get_widget_properties(&self) -> &crate::game_data::screen::widget::widget_properties::WidgetProperties {
        self.panel.get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut crate::game_data::screen::widget::widget_properties::WidgetProperties {
        self.panel.get_mut_widget_properties()
    }

    fn size(&mut self) {
        self.panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager);
    }
}


impl ScriptingElementWidget for ConditionSlot {
    fn get_line_index(&self) -> usize {
        0
    }

    fn set_highlighted(&mut self) {
        self.panel.set_color(PanelColor::Dark);
    }
}