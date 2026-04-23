

use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::{control_flow::condition::Condition, function::{function::Function, function_call::FunctionCall}, script_element::{self, ScriptElement}, var::{prim_vars::prim_var_type::PrimitiveVarType, var::Var, var_properties::{PropKey, VarPropModRequest, VarProperty}, var_type::{VarKind, VarType}}}, screen::widget::widget::WidgetType, texture_manager::texture::Texture};

#[derive(Clone, PartialEq)]
pub enum ProgrammingVar {
    ScriptingElement(ScriptElement),
    Condition(Condition),
}

impl ProgrammingVar {
    pub fn wrap_into_var(self) -> VarType {
        VarType::ProgrammingVar(self)
    }
    
    //=====================================
    // Visual
    //=====================================


    pub fn get_texture(&self) -> Texture {
        match self {
            ProgrammingVar::ScriptingElement(script_element) => {
                
                
                script_element.get_texture()
            }
            ProgrammingVar::Condition(condition) => {
                
                // return Texture::BlockTexture(crate::game_data::types::BlockTexture::Air)
                condition.get_texture()
            }
        }
    }

    pub fn into_widget(&self) -> Option<WidgetType> {
        match self {
            ProgrammingVar::ScriptingElement(script_element) => {
                None
            },
            ProgrammingVar::Condition(condition) => {
                None
            },
        }
    }

    //=====================================
    // Identity
    //=====================================

    pub fn to_kind(&self) -> ProgrammingVarKind {
        match self {
            ProgrammingVar::ScriptingElement(_) =>  ProgrammingVarKind::Function(),
            ProgrammingVar::Condition(_) => ProgrammingVarKind::Condition(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ProgrammingVar::ScriptingElement(function) => function.get_name(),
            ProgrammingVar::Condition(condition) => condition.get_name(),
        }
    }

    //=====================================
    // Properties
    //=====================================


    pub fn get_properties(&self) -> Vec<VarProperty> {
        let mut props = Vec::new();

        match self {
            ProgrammingVar::ScriptingElement(function) => {
                props.push(VarProperty { key: PropKey::Name, value: PrimitiveVarType::String(self.get_name()).create_var(), mutible: false });
            },
            ProgrammingVar::Condition(condition) => {
                todo!("");
            },
        }
        props
    }


    pub fn request_prop(&mut self, request: VarPropModRequest) { 
        match self {
            ProgrammingVar::ScriptingElement(function) => {
                eprintln!("No props requests for Var Function Exist");
            },
            ProgrammingVar::Condition(condition) => {
                eprintln!("No props requests for Var Condition Exist");
            },
        }
    }

    //=====================================
    // Managment
    //=====================================

    pub fn clear(&mut self) {
        match self {
            ProgrammingVar::ScriptingElement(function) => todo!(),
            ProgrammingVar::Condition(condition) => todo!(),
        }
    }


}


#[derive(Clone, Copy)]
pub enum ProgrammingVarKind {
    Script(),
    Function(),
    Condition(),
}

impl ProgrammingVarKind {
    pub fn wrap_into_kind(self) -> VarKind {
        VarKind::ProgrammingVar(self)
    }
    
    pub fn get_texture(&self) -> Texture {
        match self {
            _ => {
                Texture::UITexture(crate::game_data::types::UITextures::AreaIcon)
            }
        }
    }


}