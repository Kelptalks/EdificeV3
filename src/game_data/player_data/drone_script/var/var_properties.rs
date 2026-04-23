use std::{cell::RefCell, collections::{HashMap, btree_map::IterMut}, rc::Rc, thread::sleep};

use mlua::Value;

use crate::game_data::{game_event_manager::prelude::Event, player_data::{drone_script::var::{var::Var, var_type::VarType}, locations::location::WorldLocation}, screen::widget::{self, drone_programming::var_slot::var_prop_widgets::{invintory_display_prop_widget::InvintoryDisplayPropWidget, num_prop_widget::NumDisplayPropWidget, text_display_prop_widget::TextDisplayPropWidget, var_prop_widget::VarPropWidget}, panel::panel::Panel, text::{header::TextDisplay, text_input::TextInput}, widget::{Widget, WidgetType}}, tik_manager::drones::drone_inventory::InventorySlot, types::drone_item::DroneItem};





#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum PropKey {
    // All
    Name,
    Id,
    
    // Dynamic
    Cords,
    InventorySlots,
    Script,

    // Drones
    Health,
    Fuel,
    BusyTime,
    Tools,
    MinePower,
    ChopPower,

    // Block
    Friction,
    Transparent,
    Translucent,
    Solid,

    // Items
    ItemValue
}

impl PropKey {
    pub fn to_name(&self) -> String{
        match self {
            PropKey::Name => "Name".to_string(),
            PropKey::Id => "ID".to_string(),
            PropKey::Cords => "Cords".to_string(),
            PropKey::InventorySlots => "Invintory".to_string(),
            PropKey::Health => "Health".to_string(),
            PropKey::Fuel => "Fuel".to_string(),
            PropKey::BusyTime => "Busy".to_string(),
            PropKey::Tools => "Tools".to_string(),
            PropKey::MinePower => "MinePower".to_string(),
            PropKey::ChopPower => "ChopPower".to_string(),
            PropKey::Friction => "Friction".to_string(),
            PropKey::Transparent => "Transparent".to_string(),
            PropKey::Translucent => "Translucent".to_string(),
            PropKey::Solid => "Solid".to_string(),
            PropKey::ItemValue => "Cost".to_string(),
            PropKey::Script => "Script".to_string(),
        }
    }

    pub fn to_value(&self) -> PropValue {
        match self {
            PropKey::Name => PropValue::String(" ".to_string()),
            PropKey::Id => PropValue::Num(0),
            PropKey::Cords => PropValue::Cords([0; 3]),
            PropKey::InventorySlots => PropValue::Inventory(Vec::new()),
            PropKey::Health => PropValue::Num(0),
            PropKey::Fuel => PropValue::Num(0),
            PropKey::BusyTime => PropValue::Num(0),
            PropKey::Tools => PropValue::Bool(false),
            PropKey::MinePower => PropValue::Num(0),
            PropKey::ChopPower => PropValue::Num(0),
            PropKey::Friction => PropValue::Num(0),
            PropKey::Transparent => PropValue::Bool(false),
            PropKey::Translucent => PropValue::Bool(false),
            PropKey::Solid => PropValue::Bool(false),
            PropKey::ItemValue => PropValue::Inventory(Vec::new()),
            PropKey::Script => PropValue::Var(None),
        }
    }

    pub fn get_all() -> Vec<PropKey> {
        vec![
            PropKey::Name,
            PropKey::Id,
            PropKey::Cords,
            PropKey::InventorySlots,
            PropKey::Health,
            PropKey::Fuel,
            PropKey::BusyTime,
            PropKey::Tools,
            PropKey::MinePower,
            PropKey::ChopPower,
            PropKey::Friction,
            PropKey::Transparent,
            PropKey::Translucent,
            PropKey::Solid,
            PropKey::ItemValue,
            PropKey::Script,
        ]
    }

    pub fn create_widget(&self, mutable: bool) -> WidgetType {
        self.to_value().to_widget(self, mutable)
    }


}

#[derive(Clone)]
pub enum PropValue {
    // Prims
    String(String),
    Cords([i32; 3]),
    Num(i32),
    Bool(bool),

    ItemVec(Vec<DroneItem>),
    Inventory(Vec<InventorySlot>),


    Var(Option<Rc<RefCell<VarType>>>)
}

impl PropValue {


    fn to_widget(&self, prop_key: &PropKey, mutable: bool) -> WidgetType {
        match self {
            PropValue::Num(_) => {
                NumDisplayPropWidget::new(*prop_key, mutable).wrap_into_widget()
            }
            PropValue::Inventory(_) => {
                InvintoryDisplayPropWidget::new(*prop_key, mutable).wrap_into_widget()
            }
            PropValue::Var(_) => {
                VarPropWidget::new(*prop_key, mutable).wrap_into_widget()
            }
            _ => {
                TextDisplayPropWidget::new(*prop_key, mutable).wrap_into_widget()
            }
        }
    }

    pub fn into_num(&self) -> i32 {
        if let PropValue::Num(num) = self {
            *num
        }
        else {
            0
        }
    }

    pub fn into_string(&self) -> String{
        match self {
            PropValue::String(string) => string.clone(),
            PropValue::Cords(cords) => format!("{:?}", cords),
            PropValue::Num(num) => num.to_string(),
            PropValue::Bool(bool) => bool.to_string(),
            PropValue::ItemVec(drone_items) => {
                let mut string = "".to_string();
                for item in drone_items {
                    string.push_str(&format!("{}, ", item.get_name()));
                }
                string
            },
            PropValue::Inventory(inventory_slots) => {
                let mut string = "".to_string();
                for slot in inventory_slots {
                    let item_option = slot.get_item();
                    if let Some(item) = item_option {
                        let item_amount = slot.get_quantity();

                        string.push_str(&format!("{}({})  , ", item.get_name(), item_amount));
                    }
     


                }
                string
            },
            PropValue::Var(script_ref_option) => {
                if let Some(script_ref) = script_ref_option {
                    format!("Script: {}", script_ref.borrow().get_name())
                }
                else {
                    "Null Script".to_string()
                }
            }
        }
    }

    pub fn into_var(&self) -> &Option<Rc<RefCell<VarType>>> {
        match self {
            PropValue::Var(var) => {
                return var
            }
            _ => {
                &None
            }
        }
    }


    pub fn update_widget(self, widget_type: &mut WidgetType) -> Vec<VarPropModRequest> {
        
        if let WidgetType::VarPropValWidget(widget) = widget_type {
            widget.update_with_val(self)
        }
        else {
            Vec::new()
        }
    }
}

pub struct VarProperty {
    pub key: PropKey, 
    pub value: Var,
    pub mutible: bool,
}

impl VarProperty {
    pub fn update_widget(&self, widget: &mut WidgetType) -> Vec<VarPropModRequest> {
        
        if let WidgetType::VarPropValWidget(widget) = widget {
            return widget.update_with_var(&self.value);
        }
        
        Vec::new()
    }
}


#[derive(Clone)]
pub enum VarPropModRequest {
    Set(PropKey, Var),
    Add(PropKey, Var),
}

impl VarPropModRequest {
    pub fn get_name(&self) -> String {
        match self {
            VarPropModRequest::Set(prop_key, prop_value) => "Set".to_string(),
            VarPropModRequest::Add(prop_key, prop_value) => "Add".to_string(),
        }
    }
}

