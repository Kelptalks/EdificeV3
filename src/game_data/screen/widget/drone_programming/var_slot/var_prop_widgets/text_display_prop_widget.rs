use std::{cell::RefCell, rc::Rc, string};

use crate::game_data::{player_data::drone_script::var::{prim_vars::prim_var_type::PrimitiveVarType, var::Var, var_properties::{PropKey, PropValue, VarPropModRequest}, var_type::VarType}, screen::widget::{drone_programming::var_slot::var_prop_widgets::var_prop_widgets::VarPropVal, panel::panel::Panel, text::text_input::TextInput, widget::{Widget, WidgetType}}};

pub struct TextDisplayPropWidget {
    
    // Mutable
    mutable: bool,
    input_focused_ref: Option<Rc<RefCell<bool>>>,
    
    key: PropKey,
    panel: Panel,
    string_ref: Rc<RefCell<String>>,
}

impl TextDisplayPropWidget {
    pub fn new(key: PropKey, mutable: bool) -> TextDisplayPropWidget {
        
        let mut panel = Panel::new_blank();
        
        // Key name 
        let key_name = format!("{}: ", key.to_name());
        panel.add_text_display(key_name);

        let string_ref;
        let input_focused_ref;
        if mutable {
            string_ref = Rc::new(RefCell::new("Edit".to_string()));
            let text_input = TextInput::new_text_input(&string_ref);
            input_focused_ref = Some(text_input.get_focused_ref().clone());
            panel.add_widget(text_input.wrap_into_widget());
        }
        else {
            let text_display = panel.add_text_display("String Prop Widget".to_string());
            string_ref = text_display.get_string_ref().clone();
            input_focused_ref = None;
        }
        
    

        
        TextDisplayPropWidget {
            mutable,
            input_focused_ref,

            key,
            panel,
            string_ref
        }
    }


    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::VarPropValWidget(VarPropVal::String(self))
    }


    pub fn update_with_var(&mut self, var: &Var) -> Vec<VarPropModRequest> {
        let mut prop_requests = Vec::new();
        

        // Don't update if the text input is focused on
        if self.mutable {
            if let Some(focused) = &self.input_focused_ref {
                if *focused.borrow() {
                    prop_requests.push(
                        VarPropModRequest::Set(
                            self.key, 
                            PrimitiveVarType::String(self.string_ref.borrow().clone()).create_var()
                        )
                    );
                    return prop_requests;
                }
            }
        }
        
        if let Some(string) = var.as_string() {
            *self.string_ref.borrow_mut() = string.clone();
        }
        else {
            eprintln!("Cannot convert var from key{} to string in text display widget", self.key.to_name())
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

