

use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::{condition::condition::Condition, function::function::Function, script::Script, var::{var_properties::{PropKey, PropValue, VarPropModRequest, VarProperty}, var_type::Var}}, texture_manager::texture::Texture};

#[derive(Clone, PartialEq)]
pub enum ProgrammingVar {
    Script(Rc<RefCell<Script>>),
    Function(Function),
    Condition(Condition),
}

impl ProgrammingVar {
    pub fn wrap_into_var(self) -> Var {
        Var::ProgrammingVar(self)
    }
    
    pub fn get_texture(&self) -> Texture {
        match self {
            _ => {
                Texture::UITexture(crate::game_data::types::UITextures::AreaIcon)
            }
        }
    }

    pub fn to_kind(&self) -> ProgrammingVarKind {
        match self {
            ProgrammingVar::Script(_) =>    ProgrammingVarKind::Script(),
            ProgrammingVar::Function(_) =>  ProgrammingVarKind::Function(),
            ProgrammingVar::Condition(_) => ProgrammingVarKind::Condition(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ProgrammingVar::Script(script) =>           script.borrow().get_name(),
            ProgrammingVar::Function(function) =>     function.get_name(),
            ProgrammingVar::Condition(condition) =>  condition.get_name(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            ProgrammingVar::Script(script) => *script = Rc::new(RefCell::new(Script::new())),
            ProgrammingVar::Function(function) => *function = Function::new_blank(),
            ProgrammingVar::Condition(condition) => *condition = Condition::new(),
        }
    }


    pub fn get_properties(&self) -> Vec<VarProperty> {
        let mut props = Vec::new();

        match self {
            ProgrammingVar::Script(script) => {
                props.push(VarProperty { key: PropKey::Name, value: PropValue::String(self.get_name()), mutible: true });
            },
            ProgrammingVar::Function(function) => {
                props.push(VarProperty { key: PropKey::Name, value: PropValue::String(self.get_name()), mutible: false });
            },
            ProgrammingVar::Condition(condition) => {
                props.push(VarProperty { key: PropKey::Name, value: PropValue::String(self.get_name()), mutible: false });
            },
        }

        props
    }


    pub fn request_prop(&mut self, request: VarPropModRequest) { 
        match self {
            ProgrammingVar::Script(script) => {
                Self::handle_script_prop_request(script, request);
            },
            ProgrammingVar::Function(function) => {
                eprintln!("No props requests for Var Function Exist");
            },
            ProgrammingVar::Condition(condition) => {
                eprintln!("No props requests for Var Condition Exist");
            },
        }
    }
    

    pub fn handle_script_prop_request(script: &Rc<RefCell<Script>>, request: VarPropModRequest) {
        match request {
            VarPropModRequest::Set(prop_key, prop_value) => {
                match prop_key {
                    PropKey::Name => {
                        if let PropValue::String(new_name)  = prop_value {
                            script.borrow_mut().set_name(new_name);
                        }
                    },
                    _ => {
                        eprintln!("Key({}) Not supported for ProgrammingVar::Script", prop_key.to_name());
                    }
                }
            },
            _ => {
                eprintln!("Request({}) not supported for ProgrammingVar::Script", request.get_name());
            }
        }
    }

    //=====================================
    // Into
    //=====================================


    pub fn into_script(var: &Rc<RefCell<Var>>) -> Option<Rc<RefCell<Script>>> {
        let borrow = var.borrow();
        if let Var::ProgrammingVar(ProgrammingVar::Script(script)) = &*borrow {
            Some(script.clone())
        }
        else {
            None
        }
    }


    //=====================================
    // Var Constructors
    //=====================================

    pub fn construct_script_var(script: Rc<RefCell<Script>>) -> Rc<RefCell<Var>> {
        let var = ProgrammingVar::Script(script).wrap_into_var();
        return Rc::new(RefCell::new(var))
    }

    pub fn construct_function_var(function: Function) -> Rc<RefCell<Var>> {
        let var = ProgrammingVar::Function(function).wrap_into_var();
        return Rc::new(RefCell::new(var))
    }



}


#[derive(Clone, Copy)]
pub enum ProgrammingVarKind {
    Script(),
    Function(),
    Condition(),
}

impl ProgrammingVarKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            _ => {
                Texture::UITexture(crate::game_data::types::UITextures::AreaIcon)
            }
        }
    }


}