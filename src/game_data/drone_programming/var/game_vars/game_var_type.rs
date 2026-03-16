
use crate::game_data::{player_data::locations::location::WorldLocation, texture_manager::texture::Texture, types::{BlockTexture, drone_item::DroneItem}};


#[derive(PartialEq)]
pub enum GameVarTypeKind {
    DroneItem,
    Block,
}

impl GameVarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            GameVarTypeKind::DroneItem => {
                return Texture::DroneItemTexture(crate::game_data::types::DroneItemTexture::Ash)
            },
            GameVarTypeKind::Block => {
                return Texture::BlockTexture(BlockTexture::Selector)
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameVarType {
    DroneItem(DroneItem),
    Block(BlockTexture),
}

impl GameVarType {
    pub fn get_texture(&self) -> Texture {
        match self {
            GameVarType::DroneItem(drone_item) => {
                return Texture::DroneItemTexture(drone_item.to_texture_enum());
            },
            GameVarType::Block(block_texture) => {
                return Texture::BlockTexture(*block_texture);
            },
        }
    }

    pub fn to_kind(&self) -> GameVarTypeKind {
        match self {
            GameVarType::DroneItem(_drone_item) => return GameVarTypeKind::DroneItem,
            GameVarType::Block(_block_texture) => return GameVarTypeKind::Block,
        }
    }
}