
use std::{cell::RefCell, rc::{self, Rc}};

use crate::game_data::{player_data::locations::location::WorldLocation, texture_manager::texture::Texture, types::{BlockTexture, DroneItemTexture, drone_item::DroneItem}};


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

    pub fn create_mut_var(&self) -> GameVarRef {
        match self {
            GameVarTypeKind::DroneItem => return GameVarRef::DroneItem(Rc::new(RefCell::new(DroneItem::Ash))),
            GameVarTypeKind::Block => return GameVarRef::Block(Rc::new(RefCell::new(BlockTexture::Air))),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameVar {
    DroneItem(DroneItem),
    Block(BlockTexture),
    Location(u32),
}

impl GameVar {
    pub fn get_texture(&self) -> Texture {
        match self {
            GameVar::DroneItem(drone_item) => {
                return Texture::DroneItemTexture(drone_item.to_texture_enum());
            },
            GameVar::Block(block_texture) => {
                return Texture::BlockTexture(*block_texture);
            },
            GameVar::Location(u32) => {
                todo!("Have not yet implemented Location Var")
            },
        }
    }

    pub fn to_kind(&self) -> GameVarTypeKind {
        match self {
            GameVar::DroneItem(_drone_item) => return GameVarTypeKind::DroneItem,
            GameVar::Block(_block_texture) => return GameVarTypeKind::Block,
            GameVar::Location(u32) => {
                todo!("Have not yet implemented Location Var")
            },
        }
    }
}

pub enum GameVarRef {
    DroneItem(Rc<RefCell<DroneItem>>),
    Block(Rc<RefCell<BlockTexture>>),
    Location(Rc<RefCell<WorldLocation>>),
}

impl GameVarRef {
    pub fn to_kind(&self) -> GameVarTypeKind {
        match self {
            GameVarRef::DroneItem(_drone_item) => return GameVarTypeKind::DroneItem,
            GameVarRef::Block(_block_texture) => return GameVarTypeKind::Block,
            GameVarRef::Location(_u32) => {
                todo!("Have not yet implemented Location Var")
            },
        }
    }

    pub fn set_var_ref(&self, var: GameVar) { 
        match (self, var) {
            (GameVarRef::DroneItem(ref_cell), GameVar::DroneItem(drone_item)) => {
                *ref_cell.borrow_mut() = drone_item;
            },
            
            (GameVarRef::Block(ref_cell), GameVar::Block(block_texture)) => {
                *ref_cell.borrow_mut() = block_texture;
            },
            (GameVarRef::Location(ref_cell), GameVar::Location(_)) => todo!("Have not yet implemented Location Var"),
            _ => {
                println!("Cannot Set VarRef of diffrent using diffrent type");
            }
        }
    }

    pub fn to_var(&self) -> GameVar {
        match self {
            GameVarRef::DroneItem(ref_cell) => {
                return GameVar::DroneItem(*ref_cell.borrow())
            },
            GameVarRef::Block(ref_cell) => {
                return GameVar::Block(*ref_cell.borrow())
            },
            GameVarRef::Location(ref_cell) => {
                return GameVar::Location(*&ref_cell.borrow().get_id())
            },
        }
    }
}