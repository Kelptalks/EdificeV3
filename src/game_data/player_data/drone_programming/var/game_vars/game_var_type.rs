use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drones::drone::Drone, texture_manager::texture::Texture};

pub use super::{
    dynamic_var::{DynamicVar, DynamicVarRef, DynamicVarTypeKind},
    primitive_var::{PrimitiveVar, PrimitiveVarRef, PrimitiveVarTypeKind},
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
}


// ─── GameVarRef ───────────────────────────────────────────────────────────────

pub enum GameVarRef {
    Primitive(PrimitiveVarRef),
    Dynamic(DynamicVarRef),
}

impl GameVarRef {
    pub fn to_kind(&self) -> GameVarTypeKind {
        match self {
            GameVarRef::Primitive(p) => GameVarTypeKind::Primitive(p.to_kind()),
            GameVarRef::Dynamic(d) => GameVarTypeKind::Dynamic(d.to_kind()),
        }
    }

    pub fn set_var_ref(&self, var: GameVar) {
        match (self, var) {
            (GameVarRef::Primitive(p_ref), GameVar::Primitive(p_var)) => {
                p_ref.set_var_ref(p_var);
            },
            (GameVarRef::Dynamic(d_ref), GameVar::Dynamic(d_var)) => {
                d_ref.set_var_ref(d_var);
            },
            _ => {
                println!("Cannot Set VarRef of different type");
            }
        }
    }

    pub fn to_var(&self) -> GameVar {
        match self {
            GameVarRef::Primitive(p) => GameVar::Primitive(p.to_var()),
            GameVarRef::Dynamic(d) => GameVar::Dynamic(d.to_var()),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            GameVarRef::Primitive(p) => p.get_name(),
            GameVarRef::Dynamic(d) => d.get_name(),
        }
    }
}


// ─── GameVarTypeKind ──────────────────────────────────────────────────────────

#[derive(PartialEq)]
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

    pub fn create_ref_var(&self) -> GameVarRef {
        match self {
            GameVarTypeKind::Primitive(p) => GameVarRef::Primitive(p.create_ref_var()),
            GameVarTypeKind::Dynamic(d) => GameVarRef::Dynamic(d.create_ref_var()),
        }
    }
}
