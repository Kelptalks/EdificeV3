

use crate::game_data::{player_data::drone_programming::var::{game_vars::game_var_type::{GameVar, GameVarTypeKind}, var_properties::VarProperty}, texture_manager::texture::Texture};



#[derive(Clone, Copy)]
pub enum VarTypeKind {
    Any,
    Game(GameVarTypeKind)
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
                return Texture::UITexture(crate::game_data::types::UITextures::ScallingIconMidCenter);
            },
            VarTypeKind::Game(game_var_type_kind) => {
                return game_var_type_kind.get_texture();
            },
        }
    }

    

}


#[derive(Clone, PartialEq)]
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

    pub fn get_name(&self) -> String {
        match self {
            Var::Game(game_var) => game_var.get_name(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            Var::Game(game_var) => {game_var.clear()},
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            Var::Game(game_var) => {
                game_var.get_properties()
            }
        }
    }


}
