use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::locations::location::WorldLocation, tik_manager::drones::drone_inventory::InventorySlot, types::drone_item::DroneItem};

pub enum PropKey {
    // All
    Name,
    Id,
    

    // Dynamic
    Cords,
    Location,
    InventorySlots,

    // Drones
    Health,
    Fuel,
    Busy,
    Tools,
    MinePower,
    ChopPower,

    // Block
    Friction,
    Transparent,
    Translucent,
    Solid,
}

impl PropKey {
    pub fn to_name(&self) -> String{
        match self {
            PropKey::Name => "Name".to_string(),
            PropKey::Id => "ID".to_string(),
            PropKey::Cords => "Cords".to_string(),
            PropKey::Location => "Location".to_string(),
            PropKey::InventorySlots => "Invintory".to_string(),
            PropKey::Health => "Health".to_string(),
            PropKey::Fuel => "Fuel".to_string(),
            PropKey::Busy => "Busy".to_string(),
            PropKey::Tools => "Tools".to_string(),
            PropKey::MinePower => "MinePower".to_string(),
            PropKey::ChopPower => "ChopPower".to_string(),
            PropKey::Friction => "Friction".to_string(),
            PropKey::Transparent => "Transparent".to_string(),
            PropKey::Translucent => "Translucent".to_string(),
            PropKey::Solid => "Solid".to_string(),
        }
    }
}

pub enum PropValue {

    // Location
    Location(Rc<RefCell<WorldLocation>>),
    
    // Prims
    String(String),
    Cords([i32; 3]),
    Num(i32),
    Bool(bool),

    ItemVec(Vec<DroneItem>),
    Inventory(Vec<InventorySlot>)

}

pub struct VarProperty {
    pub key: PropKey, 
    pub value: PropValue,
    pub mutible: bool,
}

pub enum PropRequest {
    Set(PropKey, PropValue)
} 
