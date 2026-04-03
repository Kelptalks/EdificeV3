use crate::game_data::{player_data::drones::{drone::Drone, drone_actions::{drone_actions::DroneAction, prim_actions::drone_prim_actions::DronePrimAction}}, types::drone_item::DroneItem};

pub enum DroneInventoryAction {
    CraftItem(DroneItem),
    UseItemForFuel(DroneItem, i32),
    EquipTool(DroneItem),
}

impl From<DroneInventoryAction> for DroneAction {
    fn from(action: DroneInventoryAction) -> Self {
        DroneAction::PrimAction(DronePrimAction::DroneInventoryAction(action))
    }
}

impl DroneInventoryAction {
    pub fn execute(&self, drone: &mut Drone) {
        match self {
            DroneInventoryAction::CraftItem(drone_item) => {
                craft_item(drone, *drone_item);
            },
            DroneInventoryAction::UseItemForFuel(drone_item, quantity) => {
                use_item_for_fuel(drone, *drone_item, *quantity);
            },
            DroneInventoryAction::EquipTool(drone_item) => {
                equip_tool(drone, *drone_item);
            },
        }
    }
}


// Craft an item | Error 1 = is busy | Error 2 = Missing item | Error 3 = Item not craftable
fn craft_item(drone: &mut Drone, drone_item: DroneItem) -> u32 {
    if drone.is_busy() {
        return 1;
    }
    
    if drone_item.is_craftable() {
        let craft_cost = drone_item.get_craft_cost();
        
        // Check if has ingredients
        for slot in &craft_cost {
            let item_type = slot.get_item().unwrap();
            let item_quantity = slot.get_quantity();
            if !drone.get_inventory().has_item(item_type, item_quantity) {
                return 2;
            }
        }

        // remove ingredients from invintory
        for slot in &craft_cost {
            let item_type = slot.get_item().unwrap();
            let item_quantity = slot.get_quantity();
            if !drone.get_mut_inventory().remove_item(item_type, item_quantity) {
                return 1;
            }
        }

        // Add busy time and craft item
        drone.add_busy_time(drone_item.get_craft_time());
        drone.get_mut_inventory().add_item(drone_item, 1);
    }

    return 3;
}

// Use an item for fuel | Error 1 = missing item
fn use_item_for_fuel(drone: &mut Drone, drone_item: DroneItem, quantity: i32) -> u32{
    if drone.get_mut_inventory().remove_item(drone_item, quantity) {
        drone.add_fuel(drone_item.to_fuel_value() * quantity as u32);
        return 0;
    }
    else {
        return 1;
    }
}

// Update a drones stats based off the tools they have equiped
fn update_drone_stats(drone: &mut Drone) {
    let mut chop_power = 1;
    let mut mine_power = 1;
    for tool_index in 0..drone.get_tools().len() {
        if let Some(tool) = drone.get_tools()[tool_index] {
            mine_power += tool.mine_power();
            chop_power += tool.chop_power();
        }
    }

    drone.set_mine_power(mine_power);
    drone.set_chop_power(chop_power);
}

// Equip a tool
fn equip_tool(drone: &mut Drone, drone_item: DroneItem) {
    for tool_index in 0..drone.get_tools().len() {
        if drone.get_tools()[tool_index].is_none() {
            if drone.get_mut_inventory().remove_item(drone_item, 1) {
                drone.get_tools()[tool_index] = Some(drone_item);
                update_drone_stats(drone); // Update the drones stats after tool change
            }
            return;
        }
    }
}