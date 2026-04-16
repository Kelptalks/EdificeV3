
use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::var::{game_vars::game_var_type::GameVarType, var::{Var, VarRef}, var_properties::{PropKey, PropValue, VarProperty}, var_type::VarType}, texture_manager::texture::Texture, types::{BlockTexture, drone_item::DroneItem}};


#[derive(Clone, PartialEq)]
pub enum PrimitiveVarType {
    DroneItem(DroneItem),
    Block(BlockTexture),
    Cords([i32; 3]),
}

impl PrimitiveVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        GameVarType::Primitive(self).wrap_into_var_type()
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVarType::DroneItem(drone_item) => {
                return Texture::DroneItemTexture(drone_item.to_texture_enum());
            },
            PrimitiveVarType::Block(block_texture) => {
                return Texture::BlockTexture(*block_texture);
            },
            PrimitiveVarType::Cords(cords) => {
                return Texture::BlockTexture(BlockTexture::Selector);
            },
        }
    }

    pub fn to_kind(&self) -> PrimitiveVarTypeKind {
        match self {
            PrimitiveVarType::DroneItem(_) => PrimitiveVarTypeKind::DroneItem,
            PrimitiveVarType::Block(_) => PrimitiveVarTypeKind::Block,
            PrimitiveVarType::Cords(_) => PrimitiveVarTypeKind::Cords
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            PrimitiveVarType::DroneItem(drone_item) => {
                vec![
                    VarProperty {key: PropKey::Name, value: PropValue::String(drone_item.get_name().to_string()), mutible: false},
                    VarProperty {key: PropKey::Id,   value: PropValue::Num(drone_item.id() as i32),               mutible: false},
                ]
                
            },
            PrimitiveVarType::Block(block_texture) => {
                vec![
                    VarProperty {key: PropKey::Name,        value: PropValue::String(block_texture.get_name().to_string()), mutible: false},
                    VarProperty {key: PropKey::Id,           value: PropValue::Num(block_texture.id() as i32),              mutible: false},
                    VarProperty {key: PropKey::Friction,    value: PropValue::Num(block_texture.friction() as i32),         mutible: false},
                    VarProperty {key: PropKey::Health,      value: PropValue::Num(block_texture.hardness() as i32),         mutible: false},

                    VarProperty {key: PropKey::Solid,       value: PropValue::Bool(block_texture.is_solid()),               mutible: false},
                    VarProperty {key: PropKey::Translucent, value: PropValue::Bool(block_texture.is_translucent()),         mutible: false},
                    VarProperty {key: PropKey::Transparent, value: PropValue::Bool(block_texture.is_transparent()),         mutible: false},

                    VarProperty {key: PropKey::ItemValue,   value: PropValue::Inventory(block_texture.get_place_cost()),    mutible: false}
                ]
            },
            PrimitiveVarType::Cords(cords) => {
                vec![
                    VarProperty {key: PropKey::Cords,        value: PropValue::Cords(*cords), mutible: false}
                ]
            },
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            PrimitiveVarType::DroneItem(drone_item) => drone_item.get_name().to_string(),
            PrimitiveVarType::Block(block) => block.get_name().to_string(),
            PrimitiveVarType::Cords(cords) => format!("({:?})", cords),
        }
    }

    pub fn clear(&mut self) {
        match self {
            PrimitiveVarType::DroneItem(drone_item) => {
                *drone_item = DroneItem::Ash
            },
            PrimitiveVarType::Block(block_texture) => {
                *block_texture = BlockTexture::Air
            },
            PrimitiveVarType::Cords(cords) => {
                for f in cords.iter_mut() {
                    *f = 0;
                }
            },
        }
    }

    //=====================================
    // Constructors
    //=====================================

    pub fn construct_cords_var(cords: [i32; 3]) -> Var {
        let var = GameVarType::Primitive(PrimitiveVarType::Cords(cords)).wrap_into_var_type();
        return Var::new_with_var_type(var);
    }

    pub fn construct_block_var_ref(block: BlockTexture) -> Var {
        let var = GameVarType::Primitive(PrimitiveVarType::Block(block)).wrap_into_var_type();
        return Var::new_with_var_type(var);
    }

    pub fn construct_item_var_ref(item: DroneItem) -> Var {
        let var = PrimitiveVarType::DroneItem(item).wrap_into_var_type();
        return Var::new_with_var_type(var);
    }

    //=====================================
    // Into 
    //=====================================

    pub fn into_drone_cords(var: &Rc<RefCell<VarType>>) -> [i32; 3] {
        let borrow = var.borrow();
        if let VarType::Game(GameVarType::Primitive(PrimitiveVarType::Cords(cords))) = *borrow {
            return cords;
        }
        else {
            eprintln!("Failed to convert var {} to cords", borrow.get_name());
            return [0; 3];
        }
    }

    pub fn into_block(var: &Rc<RefCell<VarType>>) -> BlockTexture {
        let borrow = var.borrow();
        if let VarType::Game(GameVarType::Primitive(PrimitiveVarType::Block(block))) = *borrow {
            return block;
        }
        else {
            eprintln!("Failed to convert var {} to block", borrow.get_name());
            return BlockTexture::Air;
        }
    }

    pub fn into_drone_item(var: &Rc<RefCell<VarType>>) -> DroneItem {
        let borrow = var.borrow();
        if let VarType::Game(GameVarType::Primitive(PrimitiveVarType::DroneItem(item))) = *borrow {
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
                Texture::UITexture(crate::game_data::types::UITextures::ItemVarIcon)
            },
            PrimitiveVarTypeKind::Block => {
                Texture::UITexture(crate::game_data::types::UITextures::BlockVarIcon)
            },
            PrimitiveVarTypeKind::Cords => {
                Texture::BlockTexture(BlockTexture::Selector)
            }
        }
    }

}
