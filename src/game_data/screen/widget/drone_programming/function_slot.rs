use std::{cell::RefCell, fmt::format, rc::Rc};

use crate::game_data::{player_data::{drone_script::function::function::Function, drones::drone_actions::drone_actions::DroneAction}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::VarSlot, widget::{Widget, WidgetType}, widget_calculations::TextSize}};



pub struct FunctionSlot {
    panel: Panel,

}

impl FunctionSlot {

    pub fn new_with_function_ref(function: &Function) -> FunctionSlot {
        let mut panel = Panel::new_blank();
        
        // panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);

        panel.add_text_display(function.get_name()).set_text_scale(TextSize::ExtraSmall);



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
    fn get_pos(&self) -> [f32; 4] {
        self.panel.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.panel.get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.panel.get_preffered_scale()
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
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager);
    }
}