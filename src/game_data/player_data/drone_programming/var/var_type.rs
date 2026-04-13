

use crate::game_data::{player_data::drone_programming::var::{game_vars::game_var_type::{GameVar, GameVarTypeKind}, programming_vars::programming_var::{self, ProgrammingVar, ProgrammingVarKind}, var_properties::{VarPropModRequest, VarProperty}}, texture_manager::texture::Texture, types::BlockTexture};



#[derive(Clone, Copy)]
pub enum VarTypeKind {
    Any,
    Game(GameVarTypeKind),
    ProgrammingVar(ProgrammingVarKind),
}

impl PartialEq for VarTypeKind {
    fn eq(&self, other: &Self) -> bool {
        if matches!(self, Self::Any) || matches!(other, Self::Any) {
            return true;
        }
        match (self, other) {
            (Self::Game(a), Self::Game(b)) => a == b,
            _ => false,
        }
    }
}

impl VarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            VarTypeKind::Any => {
                return Texture::UITexture(crate::game_data::types::UITextures::AnyVarIcon);
            },
            VarTypeKind::Game(game_var_type_kind) => {
                return game_var_type_kind.get_texture();
            },
            VarTypeKind::ProgrammingVar(programming_var_kind) => {
                return programming_var_kind.get_texture();
            },
        }
    }

    

}


#[derive(Clone, PartialEq)]
pub enum Var {
    Game(GameVar),
    ProgrammingVar(ProgrammingVar),
}

impl Var {
    pub fn get_texture(&self) -> Texture {
        match self {
            Var::Game(game_var_type) => game_var_type.get_texture(),
            Var::ProgrammingVar(programming_var) => programming_var.get_texture(),
        }
    }

    pub fn to_kind(&self) -> VarTypeKind {
        match self {
            Var::Game(game_var_type) => {
                VarTypeKind::Game(game_var_type.to_kind())
                
            },
            Var::ProgrammingVar(programming_var) => {
                VarTypeKind::ProgrammingVar(programming_var.to_kind())
            }
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Var::Game(game_var) => game_var.get_name(),
            Var::ProgrammingVar(programming_var) => programming_var.get_name(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            Var::Game(game_var) => {game_var.clear()},
            Var::ProgrammingVar(programming_var) => {programming_var.clear();}
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            Var::Game(game_var) => {
                game_var.get_properties()
            }
            Var::ProgrammingVar(programming_var) => {
                programming_var.get_properties()
            }
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            Var::Game(game_var) => {
                game_var.request_prop(request)
            },
            Var::ProgrammingVar(programming_var) => {
                programming_var.request_prop(request);
            }
        }
    } 

}
