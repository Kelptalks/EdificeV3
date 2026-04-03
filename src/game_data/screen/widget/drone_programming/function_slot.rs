use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{drone_programming::function::function::Function, drones::drone_actions::drone_actions::DroneAction}, screen::widget::{panel::panel::Panel, prelude::VarSlot, widget::{Widget, WidgetType}}};



pub struct FunctionSlot {
    panel: Panel,

    function: Rc<RefCell<Function>>,

}

impl FunctionSlot {
    pub fn new(drone_action_type: DroneAction) -> FunctionSlot {
        let mut panel = Panel::new([0.0; 4], [0.0; 4]);
        let function=  Function::new_from_drone_action(drone_action_type);
        
        // Constuct var slots
        for var in function.get_params() {
            let var_slot = VarSlot::new(var);
            panel.add_widget(var_slot.wrap_into_widget());
        }
        panel.size();

        FunctionSlot {
            panel: panel,
            function: Rc::new(RefCell::new(function)),
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::FunctionSlot(self)
    }

    pub fn get_function_ref(&self) -> &Rc<RefCell<Function>> {
        return &self.function;
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