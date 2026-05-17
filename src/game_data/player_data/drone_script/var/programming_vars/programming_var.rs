

use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::{control_flow::condition::Condition, function::function::Function, script_element::ScriptElement, var::{prim_vars::prim_var_type::PrimitiveVarType, var::Var, var_properties::{PropKey, VarPropModRequest, VarProperty}, var_type::{VarKind, VarType}}}, screen::widget::{drone_programming::function_slot::FunctionSlot, widget::WidgetType}, texture_manager::texture::Texture, types::UITextures};

#[derive(Clone, PartialEq)]
pub enum ProgrammingVar {
    ScriptingElement(ScriptElement),
    Condition(Condition),

    Script(Rc<RefCell<Function>>),
}

impl ProgrammingVar {
    pub fn wrap_into_var_type(self) -> VarType {
        VarType::ProgrammingVar(self)
    }

    pub fn create_var(self) -> Var {
        self.wrap_into_var_type().create_var()
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
            ProgrammingVar::Script(_ref_cell) => {
                UITextures::ScallingIconMidLeft.wrap_into_texture()
            },
        }
    }

    pub fn into_widget(&self) -> Option<WidgetType> {
        match self {
            ProgrammingVar::ScriptingElement(_script_element) => {
                None
            },
            ProgrammingVar::Condition(_condition) => {
                None
            },
            ProgrammingVar::Script(function_ref) => {
                Some(FunctionSlot::new_with_function(function_ref).wrap_into_widget())
            }
        }
    }

    //=====================================
    // Identity
    //=====================================

    pub fn to_kind(&self) -> ProgrammingVarKind {
        match self {
            ProgrammingVar::ScriptingElement(_) =>  ProgrammingVarKind::Function(),
            ProgrammingVar::Condition(_) => ProgrammingVarKind::Condition(),
            ProgrammingVar::Script(_ref_cell) => ProgrammingVarKind::Script(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ProgrammingVar::ScriptingElement(function) => function.get_name(),
            ProgrammingVar::Condition(condition) => condition.get_name(),
            ProgrammingVar::Script(ref_cell) => ref_cell.borrow().get_name(),
        }
    }

    //=====================================
    // Properties
    //=====================================


    pub fn get_properties(&self) -> Vec<VarProperty> {
        let mut props = Vec::new();

        match self {
            ProgrammingVar::ScriptingElement(_function) => {
                props.push(VarProperty { key: PropKey::Name, value: PrimitiveVarType::String(self.get_name()).create_var(), mutible: false });
            },
            ProgrammingVar::Condition(_condition) => {
                todo!("");
            },
            ProgrammingVar::Script(_ref_cell) => {
                todo!("")
            },
        }
        props
    }


    pub fn request_prop(&mut self, _request: VarPropModRequest) { 
        match self {
            ProgrammingVar::ScriptingElement(_function) => {
                eprintln!("No props requests for Var Function Exist");
            },
            ProgrammingVar::Condition(_condition) => {
                eprintln!("No props requests for Var Condition Exist");
            },
            ProgrammingVar::Script(_ref_cell) => {
                eprintln!("No props requests for Var script Exist");
            },
        }
    }

    //=====================================
    // Managment
    //=====================================

    pub fn clear(&mut self) {
        match self {
            ProgrammingVar::ScriptingElement(_function) => todo!(),
            ProgrammingVar::Condition(_condition) => todo!(),
            ProgrammingVar::Script(_ref_cell) => todo!(),
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