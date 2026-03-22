use std::fmt::Error;

use image::error;

use crate::game_data::{drone_programming::var::game_vars::game_var_type::{GameVarRef, GameVar, GameVarTypeKind}, texture_manager::texture::Texture};


#[derive(PartialEq)]
pub enum VarTypeKind {
    Any,
    Game(GameVarTypeKind)
}

impl VarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            VarTypeKind::Any => {
                return Texture::UITexture(crate::game_data::types::UITextures::ScallingIconMidCenter);
            },
            VarTypeKind::Game(game_var_type_kind) => {
                return game_var_type_kind.get_texture();
            },
        }
    }

    pub fn create_mut_var(&self) -> VarRef{
        match self {
            VarTypeKind::Any => todo!(),
            VarTypeKind::Game(game_var_type_kind) => {
                return VarRef::Game(game_var_type_kind.create_mut_var());
            },
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum Var {
    Game(GameVar),
}

impl Var {
    pub fn get_texture(&self) -> Texture {
        match self {
            Var::Game(game_var_type) => game_var_type.get_texture(),
        }
    }

    pub fn to_kind(&self) -> VarTypeKind {
        match self {
            Var::Game(game_var_type) => {
                VarTypeKind::Game(game_var_type.to_kind())
            },
        }
    }
}

pub enum VarRef {
    Game(GameVarRef),
}

impl VarRef {
    pub fn to_kind(&self) -> VarTypeKind {
        match self {
            VarRef::Game(game_var_type) => {
                VarTypeKind::Game(game_var_type.to_kind())
            },
        }
    }

    pub fn set_var_ref(&self, var: Var) {
        match (self, var) {
            (VarRef::Game(game_var_ref), Var::Game(game_var)) =>{
                game_var_ref.set_var_ref(game_var);
            }
        }
    }

    pub fn to_var(&self) -> Var {
        match self {
            VarRef::Game(game_var_mut) => Var::Game(game_var_mut.to_var()),
        }
    }
}