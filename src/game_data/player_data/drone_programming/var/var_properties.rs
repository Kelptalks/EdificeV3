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
