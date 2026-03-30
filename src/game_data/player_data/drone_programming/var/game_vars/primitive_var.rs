use std::{cell::RefCell, rc::Rc};

use crate::game_data::{texture_manager::texture::Texture, types::{BlockTexture, drone_item::DroneItem}};


#[derive(Clone, PartialEq)]
pub enum PrimitiveVar {
    DroneItem(DroneItem),
    Block(BlockTexture),
}

impl PrimitiveVar {
    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVar::DroneItem(drone_item) => {
                return Texture::DroneItemTexture(drone_item.to_texture_enum());
            },
            PrimitiveVar::Block(block_texture) => {
                return Texture::BlockTexture(*block_texture);
            },
        }
    }

    pub fn to_kind(&self) -> PrimitiveVarTypeKind {
        match self {
            PrimitiveVar::DroneItem(_) => PrimitiveVarTypeKind::DroneItem,
            PrimitiveVar::Block(_) => PrimitiveVarTypeKind::Block,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            PrimitiveVar::DroneItem(_) => "DroneItem".to_string(),
            PrimitiveVar::Block(block) => block.to_string().to_string(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            PrimitiveVar::DroneItem(drone_item) => {
                *drone_item = DroneItem::Ash
            },
            PrimitiveVar::Block(block_texture) => {
                *block_texture = BlockTexture::Air
            },
        }
    }
}

#[derive(PartialEq)]
pub enum PrimitiveVarTypeKind {
    DroneItem,
    Block,
}

impl PrimitiveVarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVarTypeKind::DroneItem => {
                return Texture::DroneItemTexture(crate::game_data::types::DroneItemTexture::Ash)
            },
            PrimitiveVarTypeKind::Block => {
                return Texture::BlockTexture(BlockTexture::Selector)
            },
        }
    }

}
