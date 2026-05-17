
use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::var::{game_vars::game_var_type::{GameVarKind, GameVarType}, prim_vars::prim_var_type::PrimitiveVarType, var::Var, var_properties::{PropKey, VarProperty}, var_type::{VarKind, VarType}}, screen::widget::prelude::VarSlot, texture_manager::texture::Texture, types::{BlockTexture, UITextures, drone_item::DroneItem}};
use crate::game_data::player_data::drones::drone_inventory::InventorySlot;


#[derive(Clone, PartialEq)]
pub enum PrimitiveGameVarType {
    DroneItem(DroneItem),
    Block(BlockTexture),
    Cords([i32; 3]),
    Inventory(Vec<InventorySlot>)
}

impl PrimitiveGameVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        GameVarType::Primitive(self).wrap_into_var_type()
    }

    pub fn create_var(self) -> Var {
        self.wrap_into_var_type().create_var()
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveGameVarType::DroneItem(drone_item) => {
                return Texture::DroneItemTexture(drone_item.to_texture_enum());
            },
            PrimitiveGameVarType::Block(block_texture) => {
                return Texture::BlockTexture(*block_texture);
            },
            PrimitiveGameVarType::Cords(_cords) => {
                return UITextures::CordsIcon.wrap_into_texture();
            },
            PrimitiveGameVarType::Inventory(inventory_slots) => {
                return UITextures::ScallingIconMidCenter.wrap_into_texture()
            },
        }
    }

    pub fn to_kind(&self) -> PrimitiveGameVarTypeKind {
        match self {
            PrimitiveGameVarType::DroneItem(_) => PrimitiveGameVarTypeKind::DroneItem,
            PrimitiveGameVarType::Block(_) => PrimitiveGameVarTypeKind::Block,
            PrimitiveGameVarType::Cords(_) => PrimitiveGameVarTypeKind::Cords,
            PrimitiveGameVarType::Inventory(_) => PrimitiveGameVarTypeKind::Inventory,
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            PrimitiveGameVarType::DroneItem(drone_item) => {
                vec![
                    VarProperty {key: PropKey::Name, value: PrimitiveVarType::String(drone_item.get_name().to_string()).create_var(), mutible: false},
                    VarProperty {key: PropKey::Id,   value: PrimitiveVarType::Num(drone_item.id() as i32).create_var(),              mutible: false},
                ]

            },
            PrimitiveGameVarType::Block(block_texture) => {
                vec![
                    VarProperty {key: PropKey::Name,        value: PrimitiveVarType::String(block_texture.get_name().to_string()).create_var(), mutible: false},
                    VarProperty {key: PropKey::Id,          value: PrimitiveVarType::Num(block_texture.id() as i32).create_var(),              mutible: false},
                    VarProperty {key: PropKey::Friction,    value: PrimitiveVarType::Num(block_texture.friction() as i32).create_var(),        mutible: false},
                    VarProperty {key: PropKey::Health,      value: PrimitiveVarType::Num(block_texture.hardness() as i32).create_var(),        mutible: false},

                    VarProperty {key: PropKey::Solid,       value: PrimitiveVarType::Bool(block_texture.is_solid()).create_var(),              mutible: false},
                    VarProperty {key: PropKey::Translucent, value: PrimitiveVarType::Bool(block_texture.is_translucent()).create_var(),        mutible: false},
                    VarProperty {key: PropKey::Transparent, value: PrimitiveVarType::Bool(block_texture.is_transparent()).create_var(),        mutible: false},

                    VarProperty {key: PropKey::ItemValue, value: PrimitiveGameVarType::Inventory(block_texture.get_place_cost()).create_var(), mutible: false}
                ]
            },
            PrimitiveGameVarType::Cords(cords) => {
                vec![
                    VarProperty {key: PropKey::Cords, value: PrimitiveGameVarType::construct_cords_var(*cords), mutible: false}
                ]
            },
            PrimitiveGameVarType::Inventory(inventory) => {
                vec![
                    VarProperty {
                        key: PropKey::InventorySlots,
                        value: PrimitiveVarType::Num(inventory.len() as i32).create_var(),
                        mutible: false,
                    }
                ]
            }
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            PrimitiveGameVarType::DroneItem(drone_item) => drone_item.get_name().to_string(),
            PrimitiveGameVarType::Block(block) => block.get_name().to_string(),
            PrimitiveGameVarType::Cords(cords) => format!("({:?})", cords),
            PrimitiveGameVarType::Inventory(slots) => format!("Inventory ({} slots)", slots.len()),
        }
    }

    pub fn clear(&mut self) {
        match self {
            PrimitiveGameVarType::DroneItem(drone_item) => {
                *drone_item = DroneItem::Ash
            },
            PrimitiveGameVarType::Block(block_texture) => {
                *block_texture = BlockTexture::Air
            },
            PrimitiveGameVarType::Cords(cords) => {
                for f in cords.iter_mut() {
                    *f = 0;
                }
            },
            PrimitiveGameVarType::Inventory(slots) => slots.clear(),
        }
    }

    //=====================================
    // Constructors
    //=====================================

    pub fn construct_cords_var(cords: [i32; 3]) -> Var {
        let var = GameVarType::Primitive(PrimitiveGameVarType::Cords(cords)).wrap_into_var_type();
        return Var::new_with_var_type(var);
    }

    pub fn construct_block_var_ref(block: BlockTexture) -> Var {
        let var = GameVarType::Primitive(PrimitiveGameVarType::Block(block)).wrap_into_var_type();
        return Var::new_with_var_type(var);
    }

    pub fn construct_item_var_ref(item: DroneItem) -> Var {
        let var = PrimitiveGameVarType::DroneItem(item).wrap_into_var_type();
        return Var::new_with_var_type(var);
    }

    pub fn construct_inventory_var(slots: Vec<InventorySlot>) -> Var {
        let var = PrimitiveGameVarType::Inventory(slots).wrap_into_var_type();
        return Var::new_with_var_type(var);
    }

    //=====================================
    // Into 
    //=====================================

    pub fn into_drone_cords(var: &Rc<RefCell<VarType>>) -> [i32; 3] {
        let borrow = var.borrow();
        if let VarType::Game(GameVarType::Primitive(PrimitiveGameVarType::Cords(cords))) = *borrow {
            return cords;
        }
        else {
            eprintln!("Failed to convert var {} to cords", borrow.get_name());
            return [0; 3];
        }
    }

    pub fn into_block(var: &Rc<RefCell<VarType>>) -> BlockTexture {
        let borrow = var.borrow();
        if let VarType::Game(GameVarType::Primitive(PrimitiveGameVarType::Block(block))) = *borrow {
            return block;
        }
        else {
            eprintln!("Failed to convert var {} to block", borrow.get_name());
            return BlockTexture::Air;
        }
    }

    pub fn into_drone_item(var: &Rc<RefCell<VarType>>) -> DroneItem {
        let borrow = var.borrow();
        if let VarType::Game(GameVarType::Primitive(PrimitiveGameVarType::DroneItem(item))) = *borrow {
            return item;
        }
        else {
            eprintln!("Failed to convert var {} to drone item", borrow.get_name());
            return DroneItem::Ash;
        }
    }

}

#[derive(PartialEq, Clone, Copy)]
pub enum PrimitiveGameVarTypeKind {
    DroneItem,
    Block,
    Cords,
    Inventory,
}

impl PrimitiveGameVarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveGameVarTypeKind::DroneItem => {
                Texture::UITexture(crate::game_data::types::UITextures::ItemVarIcon)
            },
            PrimitiveGameVarTypeKind::Block => {
                Texture::UITexture(crate::game_data::types::UITextures::BlockVarIcon)
            },
            PrimitiveGameVarTypeKind::Cords => {
                return UITextures::CordsIcon.wrap_into_texture();
            },
            PrimitiveGameVarTypeKind::Inventory => {
                return Texture::UITexture(crate::game_data::types::UITextures::ScallingIconMidCenter);
            },
        }
    }

    pub fn wrap_into_var_kind(self) -> VarKind {
        VarKind::Game(GameVarKind::Primitive(self))
    }
}
