use std::{cell::{Ref, RefCell}, collections::HashMap, ops::Index, rc::Rc, usize};

use crate::game_data::{
    player_data::{
        drone_script::{
            element_body::ScriptElementBody, script_element::{self, ScriptElement}, var::{var::{Var, VarRef}, var_properties::VarPropModRequest, var_type::VarType}
        }, 
        drones::drone_actions::{
            drone_actions::DroneAction, 
            prim_actions::{drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}
        }
    }, 
    screen::{camera_controls, widget::prelude::VarSlot}};

#[derive(Clone)]
pub struct Function {
    name: String,

    params: Vec<VarRef>,

    body: ScriptElementBody,

    return_value: Var,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        eprintln!("Part Equal not implemented for Function");
        todo!()
    }
}

impl Function {
    pub fn new_blank() -> Function {
        Function {
            name: "Blank Function".to_string(),

            params: Vec::new(),
            
            body: ScriptElementBody::new(),

            return_value: Var::new_blank(),
        }
    }

    pub fn to_script_element(&self) -> ScriptElement {
        ScriptElement::Function(self.clone())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    //=====================================
    // Params
    //=====================================
    pub fn get_params(&self) -> &Vec<VarRef> {
        &self.params
    }

    //=====================================
    // Body
    //=====================================
    
    pub fn get_mut_body(&mut self) -> &mut ScriptElementBody {
        return &mut self.body
    }

    pub fn get_body(&self) -> &ScriptElementBody {
        return &self.body
    }


    //=====================================
    // Return
    //=====================================

    pub fn get_return_var(&mut self) -> VarRef {
        self.return_value.into_var_ref()
    }


}