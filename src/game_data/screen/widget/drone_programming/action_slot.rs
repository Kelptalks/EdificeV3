use crate::game_data::{player_data::drone_script::action::{self, action::Action}, screen::widget::{panel::panel::Panel, prelude::VarSlot, widget::{Widget, WidgetType}}};

pub struct ActionSlot {
    panel: Panel,
    
    action: Action,
}


impl ActionSlot {
    pub fn new(action: Action) -> ActionSlot {        
        let mut panel = Panel::new_blank();
        
        panel.add_text_display(action.get_name());


        // Add params
        let param_sub_panel = panel.add_sub_panel();
        for param_var_ref in action.get_params_var_refs() {
            let var_slot = VarSlot::new_with_var_ref(param_var_ref.clone());
            param_sub_panel.add_widget(var_slot.wrap_into_widget());
        }

        // Add return
        let return_var_option = action.get_return_var();
        if let Some(return_var) = return_var_option {
            let return_sub_panel = panel.add_sub_panel();
            let var_slot = VarSlot::new_with_var(return_var.clone());
            return_sub_panel.add_widget(var_slot.wrap_into_widget());
        } 


        
        ActionSlot {
            panel: panel,
            action: action,
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::ActionSlot(self)
    }
}


impl Widget for ActionSlot {
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
        self.panel.set_buffers(pos)
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.panel.set_parent_pos(pos)
    }

    fn size(&mut self) {
        self.panel.size()
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager)
    }
}