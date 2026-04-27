

use std::collections::VecDeque;

use crate::game_data::{player_data::{drone_script::{control_flow::control_flow::ControlFlow, element_body}, player_data::PlayerData}, screen::{ScreenData, screen_data, ui_elements::panel, widget::{self, drone_programming::{script_element_body_slot::ScriptElementBodySlot, scripting_widget_type::ScriptingElementWidget}, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::PanelColor, widget::{Widget, WidgetType}, widget_calculations::{self, TextSize}, widget_properties::WidgetProperties}}};

pub struct ControlFlowSlot {
    panel: Panel,
    line: usize,
}


impl ControlFlowSlot {
    pub fn new(control_flow: &ControlFlow, line: usize) -> ControlFlowSlot{
        let mut panel = Panel::new_blank();
        panel.set_color(PanelColor::Orange);

        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
        panel.add_text_display(control_flow.get_name()).set_text_scale(TextSize::ExtraSmall);

        let condition = control_flow.get_condition();
        panel.add_widget(condition.get_widget());


        let element_body = ScriptElementBodySlot::new(control_flow.get_body());
        panel.add_widget(element_body.wrap_into_widget());


        ControlFlowSlot {
            panel,
            line,
        }
    }

    pub fn get_mouse_incert_index(&self, screen_data: &ScreenData) -> VecDeque<usize> {
        
        if let Some(WidgetType::ScriptElementBodySlot(body_slots)) = self.panel.get_sub_widget_mouse_on(screen_data) {
            return body_slots.get_mouse_incert_index(screen_data)
        }
        else {
            VecDeque::new()
        }
        

    }

    pub fn get_mut_body_widget(&mut self) -> Option<&mut ScriptElementBodySlot> {
        for sub_widget in self.panel.get_mut_sub_widgets() {
            if let WidgetType::ScriptElementBodySlot(body) = sub_widget {
                return Some(body)
            }
        }
        None
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::ControlFlowSlot(self)
    }

    pub fn get_internal_index(&self) -> usize {
        return 0;
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
        player_data: &PlayerData,
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
        self.panel.set_color(PanelColor::Orange);
    }
}

impl ScriptingElementWidget for ControlFlowSlot {
    fn get_line_index(&self) -> usize {
        self.line
    }
    
    fn highlight(&mut self, color: [u8; 3]) {
        self.panel.set_color(PanelColor::Custom(color[0], color[1], color[2]));
    }
    
    fn get_line_incert_index(&self, screen_data: &ScreenData) -> usize {
        if self.mouse_on(screen_data){
            if !widget_calculations::is_mouse_on_top_half(self.panel.get_pos(), screen_data) {
                return self.line + 1
            }
            else {
                return self.line
            }
        }
        else {
            return 0;
        }
        
        todo!()
    }
}
