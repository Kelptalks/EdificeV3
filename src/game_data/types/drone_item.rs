use crate::game_data::{tik_manager::drones::drone_inventory::InventorySlot, types::DroneItemTexture};

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
}

impl DroneItem {
    pub fn from_id(id: u32) -> Option<DroneItem> {
        match id {
            0 => Some(DroneItem::StoneDrill),
            1 => Some(DroneItem::StoneSaw),
            2 => Some(DroneItem::IronDrill),
            3 => Some(DroneItem::IronSaw),
            4 => Some(DroneItem::IronBattery),
            5 => Some(DroneItem::IronStorage),
            6 => Some(DroneItem::IronCamera),
            7 => Some(DroneItem::TitaniumDrill),
            8 => Some(DroneItem::TitaniumSaw),
            9 => Some(DroneItem::TitaniumBattery),
            10 => Some(DroneItem::TitaniumStorage),
            11 => Some(DroneItem::TitaniumCamera),
            12 => Some(DroneItem::TNT),
            13 => Some(DroneItem::Dirt),
            14 => Some(DroneItem::PlantMatter),
            15 => Some(DroneItem::BrownLog),
            16 => Some(DroneItem::Stone),
            17 => Some(DroneItem::StoneBrick),
            18 => Some(DroneItem::ClayBrick),
            19 => Some(DroneItem::IronOar),
            20 => Some(DroneItem::IronIngot),
            21 => Some(DroneItem::CopperOar),
            22 => Some(DroneItem::CopperIngot),
            23 => Some(DroneItem::Sand),
            24 => Some(DroneItem::Glass),
            25 => Some(DroneItem::TitaniumOar),
            26 => Some(DroneItem::TitaniumIngot),
            27 => Some(DroneItem::PurpleLens),
            28 => Some(DroneItem::Ash),
            29 => Some(DroneItem::Sulfur),
            30 => Some(DroneItem::DroneChassis),
            31 => Some(DroneItem::PurpleLog),
            32 => Some(DroneItem::GoldOar),
            33 => Some(DroneItem::GoldIngot),            
            _ => None,
        }
    }

    pub fn get_id(&self) -> u32 {
        *self as u32
    }

    pub fn to_texture_enum(self) -> Option<DroneItemTexture>{
        return DroneItemTexture::from_id(self.get_id());
    }

    //=====================================
    // Fuel
    //=====================================

    pub fn to_fuel_value(&self) -> u32 {
        match self {
            DroneItem::PlantMatter => {
                return 50;
            }
            DroneItem::BrownLog => {
                return 500;
            }
            _ => {
                return 0;
            }
        }
    }

    //=====================================
    // Tool managment
    //=====================================

    pub fn is_tool(&self) -> bool {
        match self {
            DroneItem::StoneDrill => {
                return true;
            }
            DroneItem::StoneSaw => {
                return true;
            }
            DroneItem::IronSaw => {
                return true;
            }
            DroneItem::IronDrill => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn mine_power(&self) -> u32 {
        if self.is_tool() {
            match self {
                DroneItem::StoneDrill => {
                    return 5;
                }
                DroneItem::StoneSaw => {
                    return 2;
                }
                DroneItem::IronSaw => {
                    return 2;
                }
                DroneItem::IronDrill => {
                    return 5;
                }
                _ => {
                    return 1;
                }
            }
        }
        return 1;
    }

    pub fn chop_power(&self) -> u32 {
        if self.is_tool() {
            match self {
                DroneItem::StoneDrill => {
                    return 2;
                }
                DroneItem::StoneSaw => {
                    return 5;
                }
                DroneItem::IronSaw => {
                    return 5;
                }
                DroneItem::IronDrill => {
                    return 2;
                }
                _ => {
                    return 1;
                }
            }
        }
        return 1;
    }

    //=====================================
    // Crafting
    //=====================================

    pub fn is_craftable(&self) -> bool {
        match self {
            DroneItem::StoneDrill => {
                return true;
            }
            DroneItem::StoneSaw => {
                return true;
            }
            DroneItem::IronSaw => {
                return true;
            }
            DroneItem::IronDrill => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    // get the craft cost of an item
    pub fn get_craft_cost(&self) -> Vec<InventorySlot> {
        let mut craft_cost:Vec<InventorySlot> = Vec::new();
        match self {
            DroneItem::StoneDrill => {
                let mut ingredient = InventorySlot::new();
                ingredient.set_item(DroneItem::Stone);
                ingredient.set_quantity(10);
                craft_cost.push(ingredient);
            }
            DroneItem::StoneSaw => {
                let mut ingredient = InventorySlot::new();
                ingredient.set_item(DroneItem::Stone);
                ingredient.set_quantity(10);
                craft_cost.push(ingredient);
            }
            DroneItem::IronSaw => {
                let mut ingredient = InventorySlot::new();
                ingredient.set_item(DroneItem::IronIngot);
                ingredient.set_quantity(2);
                craft_cost.push(ingredient);
            }
            DroneItem::IronDrill => {
                let mut ingredient = InventorySlot::new();
                ingredient.set_item(DroneItem::IronIngot);
                ingredient.set_quantity(2);
                craft_cost.push(ingredient);
            }
            _ => {
                
            }
        }

        return craft_cost;
    }

    pub fn get_craft_time(&self) -> u32 {
        match self {
            DroneItem::StoneDrill => {
                return 100;
            }
            DroneItem::StoneSaw => {
                return 100;
            }
            DroneItem::IronSaw => {
                return 100;
            }
            DroneItem::IronDrill => {
                return 100;
            }
            _ => {
                return 0;
            }
        }
    }
}