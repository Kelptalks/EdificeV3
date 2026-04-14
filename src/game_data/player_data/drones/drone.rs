
use std::cell::RefCell;
use std::rc::Rc;

use crate::game_data::game_event_manager::prelude::{EventManager, WorldEvent};
use crate::game_data::locations::world_area::WorldArea;
use crate::game_data::player_data::drone_programming::function::function_return_value::FunctionReturnValue;
use crate::game_data::player_data::drones::drone_actions::drone_actions::DroneAction;
use crate::game_data::player_data::drones::drone_actions::drone_plan::DronePlan;
use crate::game_data::player_data::locations::location::WorldLocation;
use crate::game_data::screen::widget::button::button::Button;
use crate::game_data::screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block::LairBlockMod;
use crate::game_data::texture_manager::texture::Texture;
use crate::game_data::tik_manager::drones::drone_inventory::{DroneInventory};
use crate::game_data::types::drone_item::DroneItem;
use crate::game_data::{types::BlockTexture, World};

#[derive(Clone)]
pub enum DroneDirection {
    ForwardLeft,
    ForwardRight,
    BackLeft,
    BackRight,
}

impl DroneDirection {
    pub fn to_block_id(&self) -> u16 {
        match self {
            DroneDirection::ForwardLeft => 51,
            DroneDirection::ForwardRight => 52,
            DroneDirection::BackLeft => 53,
            DroneDirection::BackRight => 54,
        }
    }

}

pub struct Drone{
    // identity
    id: u32,
    name: String,

    // Actions
    drone_plans: Vec<DronePlan>,

    // Position
    cords: [i32; 3],
    direction: DroneDirection,
    world_location: Rc<RefCell<WorldLocation>>,

    // Stats
    busy_time: u32,
    fuel: u32,
    health: u32,
    mine_power: u32,
    chop_power: u32,

    // Items
    inventory: DroneInventory,
    tools: [Option<DroneItem>; 3],

    // Changes
    moved: bool,

    // Visuals / UI
    lair_block_mods: Vec<LairBlockMod>,
}

impl Drone {
    pub fn new(cords: [i32; 3], id: u32) -> Drone {
        let drone = Drone {
            // identity
            id: id,
            name: id.to_string(),
            
            // Actions
            drone_plans: Vec::new(),

            // Position
            cords: cords,
            direction: DroneDirection::ForwardLeft,
            world_location: Rc::new(RefCell::new(WorldLocation::new(id.to_string(), WorldArea::new_with_cords([cords; 2]), id))),

            // Stats
            busy_time: 0,
            fuel: 1000000,
            health: 1,
            mine_power: 1,
            chop_power: 1,

            // Items
            inventory: DroneInventory::new(),
            tools: [None, None, None],

            // Changes
            moved: false,

            // Visuals / UI
            lair_block_mods: Vec::new(),
        };

        return drone;
    }

    //=====================================
    // Identity
    //=====================================

    pub fn get_texture(&self) -> Texture {
        return Texture::DroneItemTexture(crate::game_data::types::DroneItemTexture::DroneChassis)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    } 

    //=====================================
    // Helpers
    //=====================================

    pub fn relative_move_cords_to_direction(relative_cords: [i32; 3]) -> DroneDirection {
        if relative_cords[0] > 0 {
            return DroneDirection::ForwardRight;
        }
        else if relative_cords[0] < 0 {
            return DroneDirection::BackLeft;
        }
        else if relative_cords[1] > 0 {
            return DroneDirection::ForwardLeft;
        }
        else {
            return DroneDirection::BackRight;
        }
    }

    pub fn get_relative_world_cords(&self, relative_cords: [i32; 3]) -> [i32; 3] {
        // Calculate the world cords based of drone position 
        let drone_cords = self.cords;
        let world_cords = [
            drone_cords[0] + relative_cords[0], 
            drone_cords[1] + relative_cords[1], 
            drone_cords[2] + relative_cords[2]
        ];
        return world_cords;
    }

    pub fn if_cords_within_range(relative_cords: [i32; 3], range: u32) -> bool {
        for i in relative_cords {
            if i.abs() as u32 > range {
                return false;
            }
        }
        return true;
    }

    //=====================================
    // Visuals / UI
    //=====================================

    pub fn add_lair_block_mod(&mut self, block_mod: LairBlockMod) {
        self.lair_block_mods.push(block_mod);
    }

    pub fn get_lair_block_mods(&self) -> &Vec<LairBlockMod> {
        &self.lair_block_mods
    }

    pub fn clear_lairblock_mods(&mut self) {
        self.lair_block_mods.clear();
    }

    // Visual Direction
    pub fn set_direction(&mut self, direction: DroneDirection) {
        self.direction = direction;
    }
    pub fn get_directoin(&self) -> &DroneDirection {
        return &self.direction;
    }

    pub fn get_location(&self) -> &Rc<RefCell<WorldLocation>> {
        return &self.world_location;
    }

    //=====================================
    // Planning
    //=====================================
    
    pub fn add_action(&mut self, action: impl Into<DroneAction>) {
        let mut plan = DronePlan::new();
        plan.add_action(action.into());
        self.drone_plans.push(plan);
    }

    pub fn add_plan(&mut self, plan: DronePlan) {
        self.drone_plans.push(plan);
    }


    //=====================================
    // Tools
    //=====================================

    pub fn get_tools(&self) -> [Option<DroneItem>; 3] {
        return self.tools;
    }
    pub fn get_mine_power(&self) -> u32 {
        return self.mine_power;
    }
    pub fn set_mine_power(&mut self, power: u32) {
        self.mine_power = power
    }

    pub fn get_chop_power(&self) -> u32 {
        return  self.chop_power;
    }
    pub fn set_chop_power(&mut self, power: u32) {
        self.chop_power = power
    }

    //=====================================
    // Invintory
    //=====================================

    // Inventory
    pub fn get_inventory(&self) -> &DroneInventory {
        return &self.inventory;
    }
    pub fn get_mut_inventory(&mut self) -> &mut DroneInventory {
        return &mut self.inventory;
    }

    // Update
    pub fn moved(&self) -> bool {
        return self.moved;
    }

    // World Data
    pub fn get_cords(&self) -> [i32; 3] {
        return self.cords;
    }
    pub fn mod_cords(&mut self, cords: [i32; 3]) {
        for (a, b) in self.cords.iter_mut().zip(cords.iter()) {
            *a += b;
        }
    }
    pub fn get_id(&self) -> u32 {
        return self.id;
    }


    //=====================================
    // Status
    //=====================================

    // Fuel
    pub fn get_fuel(&self) -> u32 {
        return self.fuel;
    }
    pub fn add_fuel(&mut self, amount: u32) {
        self.fuel += amount;
    }
    pub fn set_fuel(&mut self, amount: u32) {
        self.fuel = amount;
    }

    // Busy
    pub fn get_busy(&self) -> u32 {
        return self.busy_time;
    }
    pub fn is_busy(&self) -> bool {
        return self.busy_time != 0;
    }
    pub fn add_busy_time(&mut self, time: u32) {
        self.busy_time += time;
    }
    pub fn set_busy(&mut self, time: u32) {
        self.busy_time = time;
    }

    // Health
    pub fn get_health(&self) -> u32 {
        return self.health;
    }

    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    // Movement
    pub fn set_moved(&mut self, moved: bool) {
        self.moved = moved;
    }

    //=====================================
    // Tikking
    //=====================================

    fn handle_current_plan(&mut self, world: &World, event_manager: &mut EventManager) {
        // handle completed/failed plans after execution
        if let Some(plan) = self.drone_plans.first() {
            if plan.is_completed() || plan.has_failed() {
                self.drone_plans.remove(0);
            }
        }
        
        // Get Next action
        let action_option = self.drone_plans.first_mut().and_then(|drone_plan| {
            if drone_plan.is_completed() || drone_plan.has_failed() { None }
            else { drone_plan.pop_next_action() }
        });

        // borrow on drone_plans is fully released here
        if let Some(action) = action_option {
            let return_value = action.execute(self, world, event_manager);
            if let FunctionReturnValue::Fail(error_code) = return_value {
                if let Some(plan) = self.drone_plans.first_mut() {
                    println!("Plan Failed: {}", error_code.to_string());
                    plan.failed();
                }
            }
        }
    }

    pub fn tik_drone(&mut self, world: &World, event_manager: &mut EventManager) {
        // Update world location
        self.world_location.borrow_mut().get_mut_area().set_point_1_cords(self.cords);
        self.world_location.borrow_mut().get_mut_area().set_point_2_cords(self.cords);
        
        // If dead set kill drone and prevent actions
        if self.health == 0 {
            event_manager.add_event(WorldEvent::ModBlock(self.cords, BlockTexture::DroneDead).wrap_into_event()); // Clear drone in old location
            return;
        }
        // Prevent Actions if moved this tik
        if self.moved {
            self.moved = false;
            return;
        }
        // Prevent Actions if already busy
        if self.is_busy() {
            self.busy_time -= 1;
            self.fuel -= 1;
            return;
        }
        // Prevent Actions if out of fuel
        else if self.fuel <= 0 {
            return;
        }
        // Make Drone Fall and prevent actions
        else {
            // Make drone fall of no solid blocks below
            let mut block_below_drone = self.cords;
            block_below_drone[2] -= 1;
            let block_bellow_drone = BlockTexture::from_id(world.get_world_value(block_below_drone));
            if !block_bellow_drone.is_solid() {
                event_manager.add_event(WorldEvent::ModBlock(self.cords, BlockTexture::Air).wrap_into_event()); // Clear drone before movement
                

                self.cords[2] -= 1; // Move drone down one
                self.busy_time += 1;
                self.moved = true;

                event_manager.add_event(WorldEvent::ModBlock(self.cords, BlockTexture::from_id(self.direction.to_block_id())).wrap_into_event()); // Add drone back
                return;
            }
        }

        



        // Execute next action
        if self.drone_plans.len() > 0 {
            self.handle_current_plan(world, event_manager);
        }

        
    }
}