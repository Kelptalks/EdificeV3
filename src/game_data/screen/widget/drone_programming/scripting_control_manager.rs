use std::collections::VecDeque;

use crate::game_data::{game_event_manager::prelude::EventManager, player_data::drone_script::{element_body::ScriptElementBody, var::{programming_vars::programming_var::ProgrammingVar, var_type::VarType}}};




pub fn handle_mouse_element_body_incert(
    element_body: &mut ScriptElementBody, 
    event_manager: &mut EventManager, 
    index_keys: &mut VecDeque<usize>,
) {
        let var_held_by_mouse = event_manager.get_mut_event_tools().get_mut_mouse_widget_data().get_var_held();

        if let Some(var) = var_held_by_mouse {
            let var_type_ref = var.get_var_type_ref();
            let borrow = var_type_ref.borrow();
            if let VarType::ProgrammingVar(ProgrammingVar::ScriptingElement(mouses_element)) = &*borrow {
                

                
                element_body.insert_element_with_key(index_keys, mouses_element.clone());


            }
        }


    }