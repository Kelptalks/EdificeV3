

use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::{function::{function::Function, function_call::FunctionCall}, script_element::ScriptElement, var::{var_properties::{PropKey, PropValue, VarPropModRequest, VarProperty}, var_type::VarType}}, texture_manager::texture::Texture};

#[derive(Clone, PartialEq)]
pub enum ProgrammingVar {
    ScriptingElement(ScriptElement),
}

impl ProgrammingVar {
    pub fn wrap_into_var(self) -> VarType {
        VarType::ProgrammingVar(self)
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
            ProgrammingVar::ScriptingElement(_) =>  ProgrammingVarKind::Function(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ProgrammingVar::ScriptingElement(function) =>     function.get_name(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            ProgrammingVar::ScriptingElement(function) => todo!(),
        }
    }


    pub fn get_properties(&self) -> Vec<VarProperty> {
        let mut props = Vec::new();

        match self {
            ProgrammingVar::ScriptingElement(function) => {
                props.push(VarProperty { key: PropKey::Name, value: PropValue::String(self.get_name()), mutible: false });
            },
        }

        props
    }


    pub fn request_prop(&mut self, request: VarPropModRequest) { 
        match self {
            ProgrammingVar::ScriptingElement(function) => {
                eprintln!("No props requests for Var Function Exist");
            },
        }
    }

    //=====================================
    // Into
    //=====================================


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