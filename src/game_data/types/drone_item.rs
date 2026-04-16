use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::var::{game_vars::{game_var_type::GameVarType, primitive_var::PrimitiveVarType}, var_type::VarType}, tik_manager::drones::drone_inventory::InventorySlot, types::DroneItemTexture};

#[derive(Clone, Copy, PartialEq)]
pub enum DroneItem {
    StoneDrill = 0,
    StoneSaw = 1,
    IronDrill = 2,
    IronSaw = 3,
    IronBattery = 4,
    IronStorage = 5,
    IronCamera = 6,
    TitaniumDrill = 7,
    TitaniumSaw = 8,
    TitaniumBattery = 9,
    TitaniumStorage = 10,
    TitaniumCamera = 11,
    TNT = 12,
    Dirt = 13,
    PlantMatter = 14,
    BrownLog = 15,
    Stone = 16,
    StoneBrick = 17,
    ClayBrick = 18,
    IronOar = 19,
    IronIngot = 20,
    CopperOar = 21,
    CopperIngot = 22,
    Sand = 23,
    Glass = 24,
    TitaniumOar = 25,
    TitaniumIngot = 26,
    PurpleLens = 27,
    Ash = 28,
    Sulfur = 29,
    DroneChassis = 30,
    PurpleLog = 31,
    GoldOar = 32,
    GoldIngot = 33,
    Null = 34,
}

#[derive(Copy, Clone)]
pub struct DroneItemProperties {
    pub name: &'static str,
    pub fuel_value: u32,
    pub is_tool: bool,
    pub mine_power: u32,
    pub chop_power: u32,
    pub is_craftable: bool,
    pub craft_cost: &'static [(DroneItem, i32)],
    pub craft_time: u32,
}

static STONE_DRILL_COST: [(DroneItem, i32); 1] = [(DroneItem::Stone,     10)];
static STONE_SAW_COST:   [(DroneItem, i32); 1] = [(DroneItem::Stone,     10)];
static IRON_DRILL_COST:  [(DroneItem, i32); 1] = [(DroneItem::IronIngot,  2)];
static IRON_SAW_COST:    [(DroneItem, i32); 1] = [(DroneItem::IronIngot,  2)];

static ITEM_PROPERTIES: [DroneItemProperties; 35] = [
    DroneItemProperties { name: "Stone Drill",      fuel_value: 0,   is_tool: true,  mine_power: 5, chop_power: 2, is_craftable: true,  craft_cost: &STONE_DRILL_COST, craft_time: 100 },
    DroneItemProperties { name: "Stone Saw",        fuel_value: 0,   is_tool: true,  mine_power: 2, chop_power: 5, is_craftable: true,  craft_cost: &STONE_SAW_COST,   craft_time: 100 },
    DroneItemProperties { name: "Iron Drill",       fuel_value: 0,   is_tool: true,  mine_power: 5, chop_power: 2, is_craftable: true,  craft_cost: &IRON_DRILL_COST,  craft_time: 100 },
    DroneItemProperties { name: "Iron Saw",         fuel_value: 0,   is_tool: true,  mine_power: 2, chop_power: 5, is_craftable: true,  craft_cost: &IRON_SAW_COST,    craft_time: 100 },
    DroneItemProperties { name: "Iron Battery",     fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Iron Storage",     fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Iron Camera",      fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Titanium Drill",   fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Titanium Saw",     fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Titanium Battery", fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Titanium Storage", fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Titanium Camera",  fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "TNT",              fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Dirt",             fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Plant Matter",     fuel_value: 50,  is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Brown Log",        fuel_value: 500, is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Stone",            fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Stone Brick",      fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Clay Brick",       fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Iron Oar",         fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Iron Ingot",       fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Copper Oar",       fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Copper Ingot",     fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Sand",             fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Glass",            fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Titanium Oar",     fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Titanium Ingot",   fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Purple Lens",      fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Ash",              fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Sulfur",           fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Drone Chassis",    fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Purple Log",       fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Gold Oar",         fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Gold Ingot",       fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
    DroneItemProperties { name: "Null Item",        fuel_value: 0,   is_tool: false, mine_power: 1, chop_power: 1, is_craftable: false, craft_cost: &[],               craft_time: 0   },
];

impl DroneItem {
    pub fn get_total_items() -> usize {
        return ITEM_PROPERTIES.len();
    }

    pub fn from_id(id: u32) -> DroneItem {
        match id {
            0 => DroneItem::StoneDrill,
            1 => DroneItem::StoneSaw,
            2 => DroneItem::IronDrill,
            3 => DroneItem::IronSaw,
            4 => DroneItem::IronBattery,
            5 => DroneItem::IronStorage,
            6 => DroneItem::IronCamera,
            7 => DroneItem::TitaniumDrill,
            8 => DroneItem::TitaniumSaw,
            9 => DroneItem::TitaniumBattery,
            10 => DroneItem::TitaniumStorage,
            11 => DroneItem::TitaniumCamera,
            12 => DroneItem::TNT,
            13 => DroneItem::Dirt,
            14 => DroneItem::PlantMatter,
            15 => DroneItem::BrownLog,
            16 => DroneItem::Stone,
            17 => DroneItem::StoneBrick,
            18 => DroneItem::ClayBrick,
            19 => DroneItem::IronOar,
            20 => DroneItem::IronIngot,
            21 => DroneItem::CopperOar,
            22 => DroneItem::CopperIngot,
            23 => DroneItem::Sand,
            24 => DroneItem::Glass,
            25 => DroneItem::TitaniumOar,
            26 => DroneItem::TitaniumIngot,
            27 => DroneItem::PurpleLens,
            28 => DroneItem::Ash,
            29 => DroneItem::Sulfur,
            30 => DroneItem::DroneChassis,
            31 => DroneItem::PurpleLog,
            32 => DroneItem::GoldOar,
            33 => DroneItem::GoldIngot,
            34 => DroneItem::Null,
            _ => DroneItem::DroneChassis,
        }
    }

    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn to_texture_enum(self) -> DroneItemTexture {
        DroneItemTexture::from_id(self.id())
    }

    pub fn properties(&self) -> &'static DroneItemProperties {
        &ITEM_PROPERTIES[*self as usize]
    }

    pub fn to_fuel_value(&self) -> u32 {
        ITEM_PROPERTIES[*self as usize].fuel_value
    }

    pub fn is_tool(&self) -> bool {
        ITEM_PROPERTIES[*self as usize].is_tool
    }

    pub fn mine_power(&self) -> u32 {
        ITEM_PROPERTIES[*self as usize].mine_power
    }

    pub fn chop_power(&self) -> u32 {
        ITEM_PROPERTIES[*self as usize].chop_power
    }

    pub fn is_craftable(&self) -> bool {
        ITEM_PROPERTIES[*self as usize].is_craftable
    }

    pub fn get_craft_cost(&self) -> Vec<InventorySlot> {
        ITEM_PROPERTIES[*self as usize].craft_cost.iter().map(|(item, qty)| {
            let mut slot = InventorySlot::new();
            slot.set_item(*item);
            slot.set_quantity(*qty);
            slot
        }).collect()
    }

    pub fn get_craft_time(&self) -> u32 {
        ITEM_PROPERTIES[*self as usize].craft_time
    }

    pub fn get_name(&self) -> &str {
        return ITEM_PROPERTIES[*self as usize].name;
    }
}
