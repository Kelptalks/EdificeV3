use crate::game_data::{player_data::drone_script::element_body::ScriptElementBody, screen::{ScreenData, screen_data, widget::{panel::panel::Panel, widget::{Widget, WidgetType}, widget_properties::{self, WidgetProperties}}}};

pub struct ScriptElementBodySlot {
    widget_props: WidgetProperties,
    
    widgets: Vec<WidgetType>,




}


impl ScriptElementBodySlot {
    pub fn new(body: &ScriptElementBody) -> ScriptElementBodySlot {
        let mut widgets = Vec::new();
        
        for (line, element) in body.elements.iter().enumerate() {
           widgets.push(element.create_widget(line));
        }

        ScriptElementBodySlot {
            widget_props: WidgetProperties::new_blank(),
            widgets,
        }
    }


    pub fn get_mouse_line_index(&self, screen_data: &ScreenData) -> usize {
        for (index, widget) in self.widgets.iter().enumerate() {
            if widget.mouse_on(screen_data) {
                return index
            }
        }
        0
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
