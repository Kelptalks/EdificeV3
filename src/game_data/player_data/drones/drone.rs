
use crate::game_data::game_event_manager::prelude::{EventManager, WorldEvent};
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

    // Position
    cords: [i32; 3],
    direction: DroneDirection,

    // Stats
    busy_time: u32,
    fuel: u32,
    health: u32,
    vision_range: u32,
    modify_range: u32,
    mine_power: u32,
    chop_power: u32,

    // Items
    inventory: DroneInventory,
    tools: [Option<DroneItem>; 3],

    // Changes
    moved: bool,
}

impl Drone {
    pub fn new(cords: [i32; 3], id: u32) -> Drone {
        let mut drone = Drone {
            // identity
            id: id,

            // Position
            cords: cords,
            direction: DroneDirection::ForwardLeft,

            // Stats
            busy_time: 0,
            fuel: 1000000,
            health: 1,
            vision_range: 3,
            modify_range: 1,
            mine_power: 1,
            chop_power: 1,

            // Items
            inventory: DroneInventory::new(),
            tools: [None, None, None],

            // Changes
            moved: false,
        };

        drone.inventory.add_item(DroneItem::BrownLog, 300);
        
        drone.inventory.add_item(DroneItem::StoneDrill, 1);
        drone.equip_tool(DroneItem::StoneDrill);

        drone.inventory.add_item(DroneItem::StoneSaw, 1);
        drone.equip_tool(DroneItem::StoneSaw);

        return drone;
    }

    pub fn get_texture(&self) -> Texture {
        return Texture::DroneItemTexture(crate::game_data::types::DroneItemTexture::DroneChassis)
    }

    pub fn get_name(&self) -> String {
        self.id.to_string()
    }

    //=====================================
    // Helpers
    //=====================================

    fn relative_move_cords_to_direction(relative_cords: [i32; 3]) -> DroneDirection {
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

    fn get_relative_world_cords(&self, relative_cords: [i32; 3]) -> [i32; 3] {
        // Calculate the world cords based of drone position 
        let drone_cords = self.cords;
        let world_cords = [
            drone_cords[0] + relative_cords[0], 
            drone_cords[1] + relative_cords[1], 
            drone_cords[2] + relative_cords[2]
        ];
        return world_cords;
    }

    fn if_cords_within_range(relative_cords: [i32; 3], range: u32) -> bool {
        for i in relative_cords {
            if i.abs() as u32 > range {
                return false;
            }
        }
        return true;
    }


    //=====================================
    // Drone Getters
    //=====================================

    // Tool
    pub fn get_tools(&self) -> [Option<DroneItem>; 3] {
        return self.tools;
    }
    pub fn get_mine_power(&self) -> u32 {
        return self.mine_power;
    }
    pub fn get_chop_power(&self) -> u32 {
        return  self.chop_power;
    }

    // Inventory
    pub fn get_inventory(&self) -> &DroneInventory {
        return &self.inventory;
    }
    pub fn get_inventory_mut(&mut self) -> &mut DroneInventory {
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

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    // Fuel
    pub fn get_fuel(&self) -> u32 {
        return self.fuel;
    }

    pub fn get_vision_range(&self) -> u32 {
        return self.vision_range;
    }

    // Busy
    pub fn get_busy(&self) -> u32 {
        return self.busy_time;
    }
    pub fn is_busy(&self) -> bool {
        return self.busy_time != 0;
    }

    // Health
    pub fn get_health(&self) -> u32 {
        return self.health;
    }
    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    // Get a block based of cords relative to the drone
    pub fn scan_block(&self, world: &World, relative_cords: [i32; 3]) -> BlockTexture {
        // check if scan is in vision range
        if Drone::if_cords_within_range(relative_cords, self.vision_range){
            // Calculate the world cords based of drone position 
            let world_cords = self.get_relative_world_cords(relative_cords);

            // Get block from world
            let block = BlockTexture::from_id(world.get_world_value(world_cords));
            return block;
        }
        else {
            return BlockTexture::Air;
        }
    }

    //=====================================
    // Inventory Actions
    //=====================================

    // Craft an item | Error 1 = is busy | Error 2 = Missing item | Error 3 = Item not craftable
    pub fn craft_item(&mut self, drone_item: DroneItem) -> u32 {
        if self.is_busy() {
            return 1;
        }
        
        if drone_item.is_craftable() {
            let craft_cost = drone_item.get_craft_cost();
            
            // Check if has ingredients
            for slot in &craft_cost {
                let item_type = slot.get_item().unwrap();
                let item_quantity = slot.get_quantity();
                if !self.inventory.has_item(item_type, item_quantity) {
                    return 2;
                }
            }

            // remove ingredients from invintory
            for slot in &craft_cost {
                let item_type = slot.get_item().unwrap();
                let item_quantity = slot.get_quantity();
                if !self.inventory.remove_item(item_type, item_quantity) {
                    return 1;
                }
            }

            // Add busy time and craft item
            self.busy_time += drone_item.get_craft_time();
            self.inventory.add_item(drone_item, 1);
        }

        return 3;
    }

    // Use an item for fuel | Error 1 = missing item
    pub fn use_item_for_fuel(&mut self, drone_item: DroneItem, quantity: i32) -> u32{
        if self.inventory.remove_item(drone_item, quantity) {
            self.fuel += drone_item.to_fuel_value() * quantity as u32;
            return 0;
        }
        else {
            return 1;
        }
    }

    // Update a drones stats based off the tools they have equiped
    pub fn update_drone_stats(&mut self) {
        self.chop_power = 1;
        self.mine_power = 1;
        for tool_index in 0..self.tools.len() {
            if let Some(tool) = self.tools[tool_index] {
                self.mine_power += tool.mine_power();
                self.chop_power += tool.chop_power();
            }
        }
    }

    // Equip a tool
    pub fn equip_tool(&mut self, drone_item: DroneItem) {
        for tool_index in 0..self.tools.len() {
            if self.tools[tool_index].is_none() {
                if self.inventory.remove_item(drone_item, 1) {
                    self.tools[tool_index] = Some(drone_item);
                    self.update_drone_stats(); // Update the drones stats after tool change
                }
                return;
            }
        }
    }

    //=====================================
    // World Actions
    //=====================================

        // Mine a block relative to the drone | Error 1 = is busy | Error 2 = Cords out of range | Error 3 = Block out of range
    pub fn move_drone(&mut self, world: &World, relative_cords: [i32; 3], event_manager: &mut EventManager) -> u32{
        if self.is_busy() {
            return 1;
        }
        
        // Prevent x, y axis diagonal movement.
        if (relative_cords[0].abs() + relative_cords[1].abs()) > 1 {
            return 4;
        }

        // Prevent diagonal z movement if there is a block above the drone
        if relative_cords[1] == 1 {
            // Get block above drone
            let world_cords = self.get_relative_world_cords([0, 0, 1]);
            let block_type_of_new_location = BlockTexture::from_id(world.get_world_value(world_cords));
            if block_type_of_new_location.is_solid() {
                return 3;
            }
        }

        if Drone::if_cords_within_range(relative_cords, 1){
            let world_cords = self.get_relative_world_cords(relative_cords);
            let block_type_of_new_location = BlockTexture::from_id(world.get_world_value(world_cords));

            // If block is not solid allow movment
            if !block_type_of_new_location.is_solid() {
                // Get block bellow to calculate move speed
                let cords_below_drone = self.get_relative_world_cords([0, 0, -1]);
                let block_below_drone = BlockTexture::from_id(world.get_world_value(cords_below_drone));

                // Don't allow movement if falling
                if !BlockTexture::is_solid(&block_below_drone) {
                    return 3;
                }

                self.direction = Drone::relative_move_cords_to_direction(relative_cords);
                // Update drones cords
                event_manager.add_event(WorldEvent::ModBlock(self.cords, BlockTexture::Air).wrap_into_event()); // Clear drone in old location
                for i in 0..3{
                    self.cords[i] += relative_cords[i];
                }
                event_manager.add_event(WorldEvent::ModBlock(self.cords, BlockTexture::from_id(self.direction.to_block_id())).wrap_into_event()); // Add drone back in new location 

                // Set drone busy time based off new block below drone
                let cords_below_drone = self.get_relative_world_cords([0, 0, -1]);
                let block_below_drone = BlockTexture::from_id(world.get_world_value(cords_below_drone));
                self.busy_time += block_below_drone.friction() as u32;
                self.moved = true;

                return 0;
            }
            else {
                return 4;
            }   
        }
        return 2;
    }

    // Mine a block relative to the drone | Error 1 = is busy | Error 2 = Cords out of range
    pub fn mine_block(&mut self, relative_cords: [i32; 3], world: &World, event_manager: &mut EventManager) -> u32{
        if self.is_busy() {
            println!("Drone {} cannot mine because busy", self.id);
            return 1;
        }

        // check if scan is in mine range
        if Drone::if_cords_within_range(relative_cords, self.modify_range) {
            // Get world cords
            let world_cords = self.get_relative_world_cords(relative_cords);
            let block_to_mine = BlockTexture::from_id(world.get_world_value(world_cords));

            // Change the block to air
            event_manager.add_event(WorldEvent::ModBlock(world_cords, BlockTexture::Air).wrap_into_event());

            self.busy_time += block_to_mine.hardness() as u32;

            // Add block to inventory
            self.inventory.add_item(block_to_mine.item(), block_to_mine.item_quantity() as i32);
            return 0;
        }

        return 2;
    }

    /// Place a block relative to the drone
    /// 
    /// # Error Codes
    ///     - Error 1 = is busy
    ///     - Error 2 = Cords out of range
    ///     - Error 3 = Missing items
    ///     - Error 4 = Cannot place block as solid block is in the way
    ///     - Error 5 = Cannot piller with block type
    /// 
    pub fn place_block(&mut self, relative_cords: [i32; 3],  world: &World, event_manager: &mut EventManager, block: BlockTexture) -> u32{
        if self.is_busy() {
            return 1;
        }
        
        if Drone::if_cords_within_range(relative_cords, self.modify_range) {
            // Get world cords
            let world_cords = self.get_relative_world_cords(relative_cords);
            let block_type_at_cords = world.get_world_value(world_cords);


            if BlockTexture::from_id(block_type_at_cords).is_solid() {
                // Pillering
                if relative_cords.iter().sum::<i32>() == 0 {
                    if block == BlockTexture::Scaffolding {
                        self.move_drone(world, [0, 0, 1], event_manager);
                    }
                    else {
                        return 5
                    }
                }
                else {
                    return 4 
                }
            }

            // Remove the requried item from the invintory
            if !self.inventory.remove_item(block.item(), block.item_quantity() as i32) {
                return 3;
            }

            // Set the block

            event_manager.add_event(WorldEvent::ModBlock(world_cords, block).wrap_into_event());
            

            return 0;
        }
        return 2;
    }

    //=====================================
    // Tikking
    //=====================================

    pub fn tik_drone(&mut self, world: &World, event_manager: &mut EventManager) {
        // If dead set drone
        if self.health == 0 {
            event_manager.add_event(WorldEvent::ModBlock(self.cords, BlockTexture::DroneDead).wrap_into_event()); // Clear drone in old location
            return;
        }
        
        if self.moved {
            self.moved = false;
            return;
        }
        if self.is_busy() {
            self.busy_time -= 1;
            self.fuel -= 1;
            return;
        }
        else if self.fuel <= 0 {
            return;
        }
        // Ready for next action
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
            }
        }
    }
}