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
}

pub enum PrimitiveVarRef {
    DroneItem(Rc<RefCell<DroneItem>>),
    Block(Rc<RefCell<BlockTexture>>),
}

impl PrimitiveVarRef {
    pub fn to_kind(&self) -> PrimitiveVarTypeKind {
        match self {
            PrimitiveVarRef::DroneItem(_) => PrimitiveVarTypeKind::DroneItem,
            PrimitiveVarRef::Block(_) => PrimitiveVarTypeKind::Block,
        }
    }

    pub fn set_var_ref(&self, var: PrimitiveVar) {
        match (self, var) {
            (PrimitiveVarRef::DroneItem(ref_cell), PrimitiveVar::DroneItem(drone_item)) => {
                *ref_cell.borrow_mut() = drone_item;
            },
            (PrimitiveVarRef::Block(ref_cell), PrimitiveVar::Block(block_texture)) => {
                *ref_cell.borrow_mut() = block_texture;
            },
            _ => {
                println!("Cannot Set VarRef of different type");
            }
        }
    }

    pub fn to_var(&self) -> PrimitiveVar {
        match self {
            PrimitiveVarRef::DroneItem(ref_cell) => PrimitiveVar::DroneItem(*ref_cell.borrow()),
            PrimitiveVarRef::Block(ref_cell) => PrimitiveVar::Block(*ref_cell.borrow()),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            PrimitiveVarRef::DroneItem(_) => "DroneItem".to_string(),
            PrimitiveVarRef::Block(_) => "Block".to_string(),
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

    pub fn create_ref_var(&self) -> PrimitiveVarRef {
        match self {
            PrimitiveVarTypeKind::DroneItem => PrimitiveVarRef::DroneItem(Rc::new(RefCell::new(DroneItem::Ash))),
            PrimitiveVarTypeKind::Block => PrimitiveVarRef::Block(Rc::new(RefCell::new(BlockTexture::Air))),
        }
    }
}
