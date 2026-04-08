
use crate::game_data::{player_data::drone_programming::var::{var_properties::VarProperty, var_type::Var}, texture_manager::texture::Texture};

pub use super::{
    dynamic_var::{DynamicVar, DynamicVarTypeKind},
    primitive_var::{PrimitiveVar, PrimitiveVarTypeKind},
};


// ─── GameVar ──────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub enum GameVar {
    Primitive(PrimitiveVar),
    Dynamic(DynamicVar),
}

impl PartialEq for GameVar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GameVar::Primitive(a), GameVar::Primitive(b)) => a == b,
            (GameVar::Dynamic(a), GameVar::Dynamic(b)) => a == b,
            _ => false,
        }
    }
}

impl GameVar {
    pub fn wrap_into_var(self) -> Var {
        Var::Game(self)
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            GameVar::Primitive(p) => p.get_texture(),
            GameVar::Dynamic(d) => d.get_texture(),
        }
    }

    pub fn to_kind(&self) -> GameVarTypeKind {
        match self {
            GameVar::Primitive(p) => GameVarTypeKind::Primitive(p.to_kind()),
            GameVar::Dynamic(d) => GameVarTypeKind::Dynamic(d.to_kind()),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            GameVar::Primitive(p) => p.get_name(),
            GameVar::Dynamic(d) => d.get_name(),
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            GameVar::Primitive(primitive_var) => primitive_var.get_properties(),
            GameVar::Dynamic(dynamic_var) => dynamic_var.get_properties(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            GameVar::Primitive(primitive_var) => primitive_var.clear(),
            GameVar::Dynamic(dynamic_var) => dynamic_var.clear(),
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
