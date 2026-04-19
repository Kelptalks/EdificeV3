use std::{cell::RefCell, fmt::format, rc::Rc};

use crate::game_data::{player_data::{drone_script::function::function::Function, drones::drone_actions::drone_actions::DroneAction}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::VarSlot, widget::{Widget, WidgetType}, widget_calculations::TextSize, widget_properties::WidgetProperties}};



pub struct FunctionSlot {
    panel: Panel,
}

impl FunctionSlot {

    pub fn new_with_function(function: &Function) -> FunctionSlot {
        let mut panel = Panel::new_blank();

        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);

        panel.add_text_display(function.get_name()).set_text_scale(TextSize::ExtraSmall);

        let params = function.get_params();
        for param in params {
            let mut var_slot = VarSlot::new_with_var_ref(param.clone());
            var_slot.set_dragging_properties(true, true, true);
            panel.add_widget(var_slot.wrap_into_widget());
        }

        for element in function.get_body() {
           panel.add_widget(element.create_widget());
        }

        panel.size();
        FunctionSlot {
            panel: panel,
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::FunctionSlot(self)
    }

}

impl Widget for FunctionSlot {
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

        if self.panel.is_mouse_on() && screen_data.was_left_pressed() {
            println!("Mouse pressed on panel");
            let panel_mouse_is_on_option = self.panel.get_sub_widget_mouse_on(screen_data);
            if let Some(panel_mouse_is_on) = panel_mouse_is_on_option {
                if let WidgetType::Panel(panel) = panel_mouse_is_on {
                    println!("Mouse on sub panel");
                    panel.set_color(crate::game_data::screen::widget::prelude::PanelColor::Dark);
                }
            }
        }
    }
}
