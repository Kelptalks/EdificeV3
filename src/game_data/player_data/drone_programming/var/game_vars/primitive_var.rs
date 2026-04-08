
use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::var::{game_vars::game_var_type::GameVar, var_properties::{PropKey, PropValue, VarProperty}, var_type::Var}, texture_manager::texture::Texture, types::{BlockTexture, drone_item::DroneItem}};


#[derive(Clone, PartialEq)]
pub enum PrimitiveVar {
    DroneItem(DroneItem),
    Block(BlockTexture),
    Cords([i32; 3]),
}

impl PrimitiveVar {
    pub fn wrap_into_var(self) -> Var {
        GameVar::Primitive(self).wrap_into_var()
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVar::DroneItem(drone_item) => {
                return Texture::DroneItemTexture(drone_item.to_texture_enum());
            },
            PrimitiveVar::Block(block_texture) => {
                return Texture::BlockTexture(*block_texture);
            },
            PrimitiveVar::Cords(cords) => {
                return Texture::BlockTexture(BlockTexture::Selector);
            },
        }
    }

    pub fn to_kind(&self) -> PrimitiveVarTypeKind {
        match self {
            PrimitiveVar::DroneItem(_) => PrimitiveVarTypeKind::DroneItem,
            PrimitiveVar::Block(_) => PrimitiveVarTypeKind::Block,
            PrimitiveVar::Cords(_) => PrimitiveVarTypeKind::Cords
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            PrimitiveVar::DroneItem(drone_item) => {
                vec![
                    VarProperty {key: PropKey::Name, value: PropValue::String(drone_item.get_name().to_string()), mutible: false},
                    VarProperty {key: PropKey::Id,   value: PropValue::Num(drone_item.id() as i32),               mutible: true},
                ]
                
            },
            PrimitiveVar::Block(block_texture) => {
                vec![
                    VarProperty {key: PropKey::Name,        value: PropValue::String(block_texture.get_name().to_string()), mutible: false},
                    VarProperty {key: PropKey::Id,           value: PropValue::Num(block_texture.id() as i32),          mutible: true},
                    VarProperty {key: PropKey::Friction,    value: PropValue::Num(block_texture.friction() as i32),         mutible: false},
                    VarProperty {key: PropKey::Health,      value: PropValue::Num(block_texture.hardness() as i32),         mutible: false},

                    VarProperty {key: PropKey::Solid,       value: PropValue::Bool(block_texture.is_solid()),               mutible: false},
                    VarProperty {key: PropKey::Translucent, value: PropValue::Bool(block_texture.is_translucent()),         mutible: false},
                    VarProperty {key: PropKey::Transparent, value: PropValue::Bool(block_texture.is_transparent()),         mutible: false},
                ]
            },
            PrimitiveVar::Cords(cords) => {
                vec![
                    VarProperty {key: PropKey::Cords,        value: PropValue::Cords(*cords), mutible: false}
                ]
            },
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            PrimitiveVar::DroneItem(drone_item) => drone_item.get_name().to_string(),
            PrimitiveVar::Block(block) => block.get_name().to_string(),
            PrimitiveVar::Cords(cords) => format!("({:?})", cords),
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
            PrimitiveVar::Cords(cords) => {
                for f in cords.iter_mut() {
                    *f = 0;
                }
            },
        }
    }

    //=====================================
    // Constructors
    //=====================================

    pub fn construct_cords_var_ref(cords: [i32; 3]) -> Rc<RefCell<Var>> {
        let var = GameVar::Primitive(PrimitiveVar::Cords(cords)).wrap_into_var();
        return Rc::new(RefCell::new(var));
    }

    pub fn construct_block_var_ref(block: BlockTexture) -> Rc<RefCell<Var>> {
        let var = GameVar::Primitive(PrimitiveVar::Block(block)).wrap_into_var();
        return Rc::new(RefCell::new(var));
    }

    pub fn construct_item_var_ref(item: DroneItem) -> Rc<RefCell<Var>> {
        let var = PrimitiveVar::DroneItem(item).wrap_into_var();
        Rc::new(RefCell::new(var))
    }

    //=====================================
    // Into 
    //=====================================

    pub fn into_drone_cords(var: &Rc<RefCell<Var>>) -> [i32; 3] {
        let borrow = var.borrow();
        if let Var::Game(GameVar::Primitive(PrimitiveVar::Cords(cords))) = *borrow {
            return cords;
        }
        else {
            eprintln!("Failed to convert var {} to cords", borrow.get_name());
            return [0; 3];
        }
    }

    pub fn into_block(var: &Rc<RefCell<Var>>) -> BlockTexture {
        let borrow = var.borrow();
        if let Var::Game(GameVar::Primitive(PrimitiveVar::Block(block))) = *borrow {
            return block;
        }
        else {
            eprintln!("Failed to convert var {} to block", borrow.get_name());
            return BlockTexture::Air;
        }
    }

    pub fn into_drone_item(var: &Rc<RefCell<Var>>) -> DroneItem {
        let borrow = var.borrow();
        if let Var::Game(GameVar::Primitive(PrimitiveVar::DroneItem(item))) = *borrow {
            return item;
        }
        else {
            eprintln!("Failed to convert var {} to drone item", borrow.get_name());
            return DroneItem::Ash;
        }
    }

}

#[derive(PartialEq, Clone, Copy)]
pub enum PrimitiveVarTypeKind {
    DroneItem,
    Block,
    Cords,
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
            PrimitiveVarTypeKind::Cords => {
                return Texture::BlockTexture(BlockTexture::Selector);
            }
        }
    }

}
