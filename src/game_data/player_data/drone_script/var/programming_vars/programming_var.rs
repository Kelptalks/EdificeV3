

use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::{function::{function::Function, function_call::FunctionCall}, var::{var_properties::{PropKey, PropValue, VarPropModRequest, VarProperty}, var_type::VarType}}, texture_manager::texture::Texture};

#[derive(Clone, PartialEq)]
pub enum ProgrammingVar {
    FunctionCall(FunctionCall),
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
            ProgrammingVar::FunctionCall(_) =>  ProgrammingVarKind::Function(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ProgrammingVar::FunctionCall(function) =>     function.get_name(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            ProgrammingVar::FunctionCall(function) => todo!(),
        }
    }


    pub fn get_properties(&self) -> Vec<VarProperty> {
        let mut props = Vec::new();

        match self {
            ProgrammingVar::FunctionCall(function) => {
                props.push(VarProperty { key: PropKey::Name, value: PropValue::String(self.get_name()), mutible: false });
            },
        }

        props
    }


    pub fn request_prop(&mut self, request: VarPropModRequest) { 
        match self {
            ProgrammingVar::FunctionCall(function) => {
                eprintln!("No props requests for Var Function Exist");
            },
        }
    }

    //=====================================
    // Into
    //=====================================



    //=====================================
    // Var Constructors
    //=====================================
    
    pub fn construct_function_call_var(function: FunctionCall) -> Rc<RefCell<VarType>> {
        let var = ProgrammingVar::FunctionCall(function).wrap_into_var();
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