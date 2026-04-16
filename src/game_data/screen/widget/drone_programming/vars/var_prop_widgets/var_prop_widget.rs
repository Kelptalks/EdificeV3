use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::var::{game_vars::primitive_var::PrimitiveVarType, programming_vars::programming_var::ProgrammingVar, var_properties::{PropKey, PropValue, VarPropModRequest}, var_type::VarType}, screen::widget::{drone_programming::vars::var_prop_widgets::var_prop_widgets::VarPropVal, panel::panel::Panel, prelude::VarSlot, text::text_input::TextInput, toggle_button::{self, toggle_button::ToggleButton}, widget::{Widget, WidgetType}}, types::drone_item::DroneItem};

pub struct VarPropWidget {
    // Mutable
    mutable: bool,

    key: PropKey,
    panel: Panel,


    set_var_bool: Rc<RefCell<bool>>,
}

impl VarPropWidget {
    pub fn new(key: PropKey, mutable: bool) -> VarPropWidget {
        
        let mut panel = Panel::new_blank();
        
        // Var Slot
        let var_type = PrimitiveVarType::DroneItem(DroneItem::Ash).wrap_into_var_type();
        let var_slot = VarSlot::new(var_type);
        panel.add_widget(var_slot.wrap_into_widget());
        
        if let PropKey::Script = key {

        }


        // Set Var
        let mut button = ToggleButton::new();
        let toggle_ref = button.get_toggle_ref();
        button.set_text("Set Var".to_string());



        panel.add_widget(button.wrap_into_widget());
        // Key name 
        let key_name = format!("{}: ", key.to_name());
        panel.add_text_display(key_name);

        
    

        
        VarPropWidget {
            mutable,

            key,
            panel,

            set_var_bool: toggle_ref,
        }
    }


    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::VarPropValWidget(VarPropVal::Var(self))
    }


    pub fn update_with_val(&mut self, val: PropValue) -> Vec<VarPropModRequest> {
        let mut prop_requests = Vec::new();


        if *self.set_var_bool.borrow() {

            
            // prop_requests.push(VarPropModRequest::Set(self.key, PropValue::Var(Some(self.var.clone()))));

            *self.set_var_bool.borrow_mut() = false;
            todo!()
        }


        self.panel.size();

        return prop_requests;
    }



    pub fn get_root_mut_panel(&mut self) -> &mut Panel {
        return &mut self.panel
    }

    pub fn get_root_panel(&self) -> &Panel {
        return &self.panel;
    }

    
}

