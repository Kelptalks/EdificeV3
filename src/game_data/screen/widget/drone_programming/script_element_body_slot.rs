use std::collections::VecDeque;

use crate::game_data::{player_data::drone_script::element_body::ScriptElementBody, screen::{ScreenData, screen_data, widget::{self, panel::panel::Panel, widget::{Widget, WidgetType}, widget_calculations, widget_properties::{self, WidgetProperties}}}};

pub struct ScriptElementBodySlot {
    widget_props: WidgetProperties,
    
    
    widgets: Vec<WidgetType>,




}


impl ScriptElementBodySlot {
    pub fn new(body: &ScriptElementBody) -> ScriptElementBodySlot {
        let mut widgets = Vec::new();

        let mut head_panel = Panel::new_blank();
        head_panel.add_text_display("Head".to_string());
        widgets.push(head_panel.wrap_into_widget());        

        for (line, element) in body.elements.iter().enumerate() {
           widgets.push(element.create_widget(line));
        }

        let mut tail_panel = Panel::new_blank();
        tail_panel.add_text_display("Tail".to_string());
        widgets.push(tail_panel.wrap_into_widget());        



        let mut body = ScriptElementBodySlot {
            
            widget_props: WidgetProperties::new_blank(),
            
            widgets,
        };

        body.size();

        return body;
    }


    pub fn get_mouse_line_index(&self, screen_data: &ScreenData) -> Option<Vec<usize>> {
        for (index, widget) in self.widgets.iter().enumerate() {
            if widget.mouse_on(screen_data) {
                return Some(vec![index])
            }
        }
        None
    }

    pub fn get_mouse_incert_index(&self, screen_data: &ScreenData) -> VecDeque<usize> {
        let mut indexes = VecDeque::new();

        for (index, widget) in self.widgets.iter().enumerate() {
            if widget.mouse_on(screen_data) {
                if index == 0 {
                    indexes.push_back(0);
                    return indexes;
                }


                if let WidgetType::ControlFlowSlot(control_flow_slot) = widget {
                    let mut child_indexes = control_flow_slot.get_mouse_incert_index(screen_data);
                    child_indexes.push_back(if widget_calculations::is_mouse_on_top_half(widget.get_pos(), screen_data) {
                        index - 1
                    } else {
                        index - 1
                    });
                    return child_indexes;
                }
                
                if widget_calculations::is_mouse_on_top_half(widget.get_pos(), screen_data) {
                    indexes.push_back(index - 1)
                }
                else {
                    indexes.push_back(index)
                }
            }
        }

        indexes
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::ScriptElementBodySlot(self)
    }
    

}

impl Widget for ScriptElementBodySlot {
    fn get_widget_properties(&self) -> &crate::game_data::screen::widget::widget_properties::WidgetProperties {
        return &self.widget_props;
        
    }

    fn get_mut_widget_properties(&mut self) -> &mut crate::game_data::screen::widget::widget_properties::WidgetProperties {
        return &mut self.widget_props
    }

    fn size(&mut self) {
        self.widget_props.calculate_prefered_scale_from_widget_list(&self.widgets);
        self.widget_props.scale_based_off_parent();


        let mut current_bot_buffer = 0.0;

        let parent_pos = self.get_pos();
        let parent_scale = self.get_scale();
        
        
        for widget in &mut self.widgets {
            
            let prefered_scale = widget.get_preffered_scale();
            
            let top_buffer = parent_scale[1] - (prefered_scale[1] + current_bot_buffer);

            let buffer = [
                0.0,
                current_bot_buffer,
                parent_scale[0] - prefered_scale[0],
                top_buffer,
            ];

            widget.set_parent_pos(parent_pos);
            widget.set_buffers(buffer);

            current_bot_buffer += prefered_scale[1];
            
            widget.size();
        }

    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
    ) {
        
        for widget in &mut self.widgets {
            widget.render(texture_manager, screen_data, game_event_manager);

        }
    }
}
