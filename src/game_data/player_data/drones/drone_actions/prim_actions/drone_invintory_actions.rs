use std::{cell::RefCell, fmt::format, rc::Rc};

use crate::game_data::{player_data::{drone_script::var::{game_vars::primitive_var::PrimitiveVarType, var::Var, var_type::VarType}, drones::{drone::Drone, drone_actions::{drone_actions::{DroneAction, DroneActionError}, prim_actions::drone_prim_actions::DronePrimAction}}}, types::drone_item::DroneItem};

#[derive(Clone)]
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
    pub fn execute(&self, drone: &mut Drone) -> Var {
        match self {
            DroneInventoryAction::CraftItem(drone_item) => {
                craft_item(drone, *drone_item)
            },
            DroneInventoryAction::UseItemForFuel(drone_item, quantity) => {
                use_item_for_fuel(drone, *drone_item, *quantity)
            },
            DroneInventoryAction::EquipTool(drone_item) => {
                equip_tool(drone, *drone_item)
            },
        }
    }


    pub fn wrap_into_action(self) -> DroneAction {
        DronePrimAction::DroneInventoryAction(self).wrap_into_action()
    }

    pub fn get_all_actions() -> Vec<DroneAction> {
        let mut all_actions = Vec::new();
    
        all_actions.push(DroneInventoryAction::CraftItem(DroneItem::Null).wrap_into_action());
        all_actions.push(DroneInventoryAction::UseItemForFuel(DroneItem::Null, 0).wrap_into_action());
        all_actions.push(DroneInventoryAction::EquipTool(DroneItem::Null).wrap_into_action());
        
        return all_actions;
    }

    //=====================================
    // Identity
    //=====================================

    pub fn get_name(&self) -> String {
        match self {
            DroneInventoryAction::CraftItem(drone_item) => {
                format!("CraftItem")
            },
            DroneInventoryAction::UseItemForFuel(drone_item, amount) => {
                format!("UseItemForFuel")
            },
            DroneInventoryAction::EquipTool(drone_item) => {
                format!("EquipTool")
            },
        }
    }

    //=====================================
    // Function Construction
    //=====================================

    // Creates a set of var refrenses 
    // Why: used to get the Vars needed for constructing Functions
    pub fn get_param_var_types(&self) -> Vec<VarType> {
        let mut params = Vec::new();

        match self {
            DroneInventoryAction::CraftItem(drone_item) => {
                params.push(PrimitiveVarType::DroneItem(*drone_item).wrap_into_var_type());
            },
            DroneInventoryAction::UseItemForFuel(drone_item, _) => {
                params.push(PrimitiveVarType::DroneItem(*drone_item).wrap_into_var_type());
            },
            DroneInventoryAction::EquipTool(drone_item) => {
                params.push(PrimitiveVarType::DroneItem(*drone_item).wrap_into_var_type());
            },
        }

        return params;
    }

    pub fn set_params_from_vars(&mut self, params: &Vec<Rc<RefCell<VarType>>>) {
        match self {
            DroneInventoryAction::CraftItem(drone_item) => {
                *drone_item = PrimitiveVarType::into_drone_item(&params[0]);
            },
            DroneInventoryAction::UseItemForFuel(drone_item, _) => {
                *drone_item = PrimitiveVarType::into_drone_item(&params[0]);
            },
            DroneInventoryAction::EquipTool(drone_item) => {
                *drone_item = PrimitiveVarType::into_drone_item(&params[0]);
            },
        }
    }

}

//=====================================
// Execution
//=====================================


// Craft an item | Error 1 = is busy | Error 2 = Missing item | Error 3 = Item not craftable
fn craft_item(drone: &mut Drone, drone_item: DroneItem) -> Var {
    if drone.is_busy() {
        return DroneActionError::Busy.wrap_into_var_type().create_var()
    }
    
    if drone_item.is_craftable() {
        let craft_cost = drone_item.get_craft_cost();
        
        // Check if has ingredients
        for slot in &craft_cost {
            let item_type = slot.get_item().unwrap();
            let item_quantity = slot.get_quantity();
            if !drone.get_inventory().has_item(item_type, item_quantity) {
                return DroneActionError::MissingItem.wrap_into_var_type().create_var()
            }
        }

        // remove ingredients from invintory
        for slot in &craft_cost {
            let item_type = slot.get_item().unwrap();
            let item_quantity = slot.get_quantity();
            if !drone.get_mut_inventory().remove_item(item_type, item_quantity) {
                return DroneActionError::MissingItem.wrap_into_var_type().create_var()
            }
        }

        // Add busy time and craft item
        drone.add_busy_time(drone_item.get_craft_time());
        drone.get_mut_inventory().add_item(drone_item, 1);
        
        return DroneActionError::Ok.wrap_into_var_type().create_var()
    }
    else {

        return DroneActionError::UncraftableItem.wrap_into_var_type().create_var()
    }
}

// Use an item for fuel | Error 1 = missing item
fn use_item_for_fuel(drone: &mut Drone, drone_item: DroneItem, quantity: i32) -> Var {
    if drone.get_mut_inventory().remove_item(drone_item, quantity) {
        drone.add_fuel(drone_item.to_fuel_value() * quantity as u32);
        return DroneActionError::Ok.wrap_into_var_type().create_var();
    }
    else {
        return DroneActionError::MissingItem.wrap_into_var_type().create_var();
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
fn equip_tool(drone: &mut Drone, drone_item: DroneItem) -> Var {
    for tool_index in 0..drone.get_tools().len() {
        if drone.get_tools()[tool_index].is_none() {
            if drone.get_mut_inventory().remove_item(drone_item, 1) {
                drone.get_tools()[tool_index] = Some(drone_item);
                update_drone_stats(drone); // Update the drones stats after tool change
                return DroneActionError::Ok.wrap_into_var_type().create_var();
            }
            return DroneActionError::MissingItem.wrap_into_var_type().create_var();
        }
    }
    return DroneActionError::MissingSlot.wrap_into_var_type().create_var();
}