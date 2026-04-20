

use crate::game_data::{player_data::drone_script::control_flow::control_flow::ControlFlow, screen::widget::{drone_programming::scripting_widget_type::ScriptingElementWidget, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::PanelColor, widget::{Widget, WidgetType}, widget_calculations::TextSize, widget_properties::WidgetProperties}};

pub struct ControlFlowSlot {
    panel: Panel,
    line: usize,
}


impl ControlFlowSlot {
    pub fn new(control_flow: &ControlFlow, line: usize) -> ControlFlowSlot{
        let mut panel = Panel::new_blank();

        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
        panel.add_text_display(control_flow.get_name()).set_text_scale(TextSize::ExtraSmall);

        let condition = control_flow.get_condition();
        panel.add_widget(condition.get_widget());

        ControlFlowSlot {
            panel,
            line,
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::ControlFlowSlot(self)
    }
}

impl Widget for ControlFlowSlot {
    fn get_widget_properties(&self) -> &WidgetProperties {
        self.panel.get_widget_properties()
        
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        self.panel.get_mut_widget_properties()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.panel.set_buffers(pos);
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.panel.set_parent_pos(pos);
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
        self.panel.set_color(PanelColor::Light);
    }
}

impl ScriptingElementWidget for ControlFlowSlot {
    fn get_line_index(&self) -> usize {
        self.line
    }
    
    fn set_highlighted(&mut self) {
        self.panel.set_color(PanelColor::Dark);
    }
}
