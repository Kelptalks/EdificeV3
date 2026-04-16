
use crate::game_data::{player_data::drone_script::var::{game_vars::action_var::ActionVarType, var_properties::{VarPropModRequest, VarProperty}, var_type::VarType}, texture_manager::texture::Texture};

pub use super::{
    dynamic_var::{DynamicVarType, DynamicVarTypeKind},
    primitive_var::{PrimitiveVarType, PrimitiveVarTypeKind},
};


// ─── GameVar ──────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub enum GameVarType {
    Primitive(PrimitiveVarType),
    Dynamic(DynamicVarType),
    Action(ActionVarType)
}

impl PartialEq for GameVarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GameVarType::Primitive(a), GameVarType::Primitive(b)) => a == b,
            (GameVarType::Dynamic(a), GameVarType::Dynamic(b)) => a == b,
            _ => false,
        }
    }
}

impl GameVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        VarType::Game(self)
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            GameVarType::Primitive(p) => p.get_texture(),
            GameVarType::Dynamic(d) => d.get_texture(),
            GameVarType::Action(d) => d.get_texture(),
        }
    }

    pub fn to_kind(&self) -> GameVarTypeKind {
        match self {
            GameVarType::Primitive(p) => GameVarTypeKind::Primitive(p.to_kind()),
            GameVarType::Dynamic(d) => GameVarTypeKind::Dynamic(d.to_kind()),
            GameVarType::Action(d) => todo!()
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            GameVarType::Primitive(p) => p.get_name(),
            GameVarType::Dynamic(d) => d.get_name(),
            GameVarType::Action(d) => todo!(),
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            GameVarType::Primitive(primitive_var) => primitive_var.get_properties(),
            GameVarType::Dynamic(dynamic_var) => dynamic_var.get_properties(),
            GameVarType::Action(d) => todo!(),
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            GameVarType::Primitive(primitive_var_type_kind) => {
                eprintln!("NO IMPLEMENTATION IMPLEMENTED FOR MINIPULATING PRIM VARS");
            },
            GameVarType::Dynamic(dynamic_var_type_kind) => {
                dynamic_var_type_kind.request_prop(request);
            },
            GameVarType::Action(d) => todo!(),
        }
    } 

    

    pub fn clear(&mut self) {
        match self {
            GameVarType::Primitive(primitive_var) => primitive_var.clear(),
            GameVarType::Dynamic(dynamic_var) => dynamic_var.clear(),
            GameVarType::Action(d) => todo!(),
        }
    }
}

// ─── GameVarTypeKind ──────────────────────────────────────────────────────────

#[derive(PartialEq, Clone, Copy)]
pub enum GameVarTypeKind {
    Primitive(PrimitiveVarTypeKind),
    Dynamic(DynamicVarTypeKind),
}

impl GameVarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            GameVarTypeKind::Primitive(p) => p.get_texture(),
            GameVarTypeKind::Dynamic(d) => d.get_texture(),
        }
    }

    
}
