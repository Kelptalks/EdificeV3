use crate::game_data::{drone_programming::var::game_vars::game_var_type::{GameVarType, GameVarTypeKind}, texture_manager::texture::Texture};


#[derive(PartialEq)]
pub enum VarTypeKind {
    VarType,
    Game(GameVarTypeKind)
}

impl VarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            VarTypeKind::VarType => {
                return Texture::UITexture(crate::game_data::types::UITextures::ScallingIconMidCenter);
            },
            VarTypeKind::Game(game_var_type_kind) => {
                return game_var_type_kind.get_texture();
            },
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum VarType {
    Game(GameVarType),
}

impl VarType {
    pub fn get_texture(&self) -> Texture {
        match self {
            VarType::Game(game_var_type) => game_var_type.get_texture(),
        }
    }

    pub fn to_kind(&self) -> VarTypeKind {
        match self {
            VarType::Game(game_var_type) => {
                VarTypeKind::Game(game_var_type.to_kind())
            },
        }
    }
}