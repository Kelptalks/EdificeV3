use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::locations::location::WorldLocation, screen::widget::{text::header::TextDisplay, widget::WidgetType}, tik_manager::drones::drone_inventory::InventorySlot, types::drone_item::DroneItem};

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

impl VarProperty {

    fn prop_value_to_text_display(prop_key: &PropKey, prop_value: &PropValue) -> WidgetType {
        if let PropValue::String(string) = prop_value {
            let text = format!("{}: {}", prop_key.to_name(), string);
            TextDisplay::new(text).wrap_into_widget()
        }
        else if let PropValue::Num(number) = prop_value {
            let text = format!("{}: {}", prop_key.to_name(), number);
            TextDisplay::new(text).wrap_into_widget()
        }
        else if let PropValue::Bool(bool) = prop_value {
            let text = format!("{}: {}", prop_key.to_name(), bool);
            TextDisplay::new(text).wrap_into_widget()
        }
        else if let PropValue::Cords(cords) = prop_value {
            let text = format!("{}: {:?}", prop_key.to_name(), cords);
            TextDisplay::new(text).wrap_into_widget()
        }
        else {
            let text = format!("{}: MISSING NUM VALUE", prop_key.to_name());
            TextDisplay::new(text).wrap_into_widget()
        }
    }

    pub fn into_widget(&self) -> WidgetType {
        match self.value {
            PropValue::Num(_) => {
                Self::prop_value_to_text_display(&self.key, &self.value)
            }
            PropValue::String(_) => {
                Self::prop_value_to_text_display(&self.key, &self.value)
            }
            PropValue::Bool(_) => {
                Self::prop_value_to_text_display(&self.key, &self.value)
            }
            PropValue::Cords(_) => {
                Self::prop_value_to_text_display(&self.key, &self.value)
            }
            _ => {
                let text = format!("Widget not implemented for key({})", self.key.to_name());
                return TextDisplay::new(text).wrap_into_widget();
            }
        }
    } 

}

pub enum PropRequest {
    Set(PropKey, PropValue)
} 
