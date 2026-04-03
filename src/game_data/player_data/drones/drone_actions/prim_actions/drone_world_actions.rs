use crate::game_data::{World, game_event_manager::prelude::{EventManager, WorldEvent}, player_data::drones::{drone::Drone, drone_actions::{drone_actions::DroneAction, prim_actions::drone_prim_actions::DronePrimAction}}, types::BlockTexture};

pub enum DroneWorldAction {
    MoveDrone([i32; 3]),
    MineBlock([i32; 3]),
    PlaceBlock([i32; 3], BlockTexture),
}

impl From<DroneWorldAction> for DroneAction {
    fn from(action: DroneWorldAction) -> Self {
        DroneAction::PrimAction(DronePrimAction::DroneWorldAction(action))
    }
}

impl DroneWorldAction {
    pub fn execute(&self, drone: &mut Drone, world: &World, event_manager: &mut EventManager) {
        match self {
            DroneWorldAction::MoveDrone(relative_cords) => {
                move_drone(drone, world, *relative_cords, event_manager);           
            },
            DroneWorldAction::MineBlock(relative_cords) => {
                mine_block(drone, *relative_cords, world, event_manager);
            },
            DroneWorldAction::PlaceBlock(relative_cords, block_texture) => {
                place_block(drone, world, event_manager, *relative_cords, *block_texture);
            },
        }
    }
}

// Mine a block relative to the drone | Error 1 = is busy | Error 2 = Cords out of range | Error 3 = Block out of range
fn move_drone(drone: &mut Drone, world: &World, relative_cords: [i32; 3], event_manager: &mut EventManager) -> u32 {
    if drone.is_busy() {
        return 1;
    }

    // Prevent x, y axis diagonal movement.
    if (relative_cords[0].abs() + relative_cords[1].abs()) > 1 {
        return 4;
    }

    // Prevent diagonal z movement if there is a block above the drone
    if relative_cords[1] == 1 {
        // Get block above drone
        let world_cords = drone.get_relative_world_cords([0, 0, 1]);
        let block_type_of_new_location = BlockTexture::from_id(world.get_world_value(world_cords));
        if block_type_of_new_location.is_solid() {
            return 3;
        }
    }

    if Drone::if_cords_within_range(relative_cords, 1) {
        let world_cords = drone.get_relative_world_cords(relative_cords);
        let block_type_of_new_location = BlockTexture::from_id(world.get_world_value(world_cords));

        // If block is not solid allow movment
        if !block_type_of_new_location.is_solid() {
            // Get block bellow to calculate move speed
            let cords_below_drone = drone.get_relative_world_cords([0, 0, -1]);
            let block_below_drone = BlockTexture::from_id(world.get_world_value(cords_below_drone));

            // Don't allow movement if falling
            if !BlockTexture::is_solid(&block_below_drone) {
                return 3;
            }

            drone.set_direction(Drone::relative_move_cords_to_direction(relative_cords));
            // Update drones cords
            event_manager.add_event(WorldEvent::ModBlock(drone.get_cords(), BlockTexture::Air).wrap_into_event()); // Clear drone in old location
            
            drone.mod_cords(relative_cords);
            
            event_manager.add_event(WorldEvent::ModBlock(drone.get_cords(), BlockTexture::from_id(drone.get_directoin().to_block_id())).wrap_into_event()); // Add drone back in new location

            // Set drone busy time based off new block below drone
            let cords_below_drone = drone.get_relative_world_cords([0, 0, -1]);
            let block_below_drone = BlockTexture::from_id(world.get_world_value(cords_below_drone));
            drone.add_busy_time(block_below_drone.friction() as u32);
            drone.set_moved(true);

            return 0;
        }
        else {
            return 4;
        }
    }
    return 2;
}

// Mine a block relative to the drone | Error 1 = is busy | Error 2 = Cords out of range
fn mine_block(drone: &mut Drone, relative_cords: [i32; 3], world: &World, event_manager: &mut EventManager) -> u32 {
    if drone.is_busy() {
        println!("Drone {} cannot mine because busy", drone.get_id());
        return 1;
    }

    // check if scan is in mine range
    if Drone::if_cords_within_range(relative_cords, 1) {
        // Get world cords
        let world_cords = drone.get_relative_world_cords(relative_cords);
        let block_to_mine = BlockTexture::from_id(world.get_world_value(world_cords));

        // Change the block to air
        event_manager.add_event(WorldEvent::ModBlock(world_cords, BlockTexture::Air).wrap_into_event());

        drone.add_busy_time(block_to_mine.hardness() as u32);

        // Add block to inventory
        drone.get_mut_inventory().add_item(block_to_mine.item(), block_to_mine.item_quantity() as i32);
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
fn place_block(drone: &mut Drone, world: &World, event_manager: &mut EventManager, relative_cords: [i32; 3], block: BlockTexture) -> u32 {
    if drone.is_busy() {
        return 1;
    }

    if Drone::if_cords_within_range(relative_cords, 1) {
        // Get world cords
        let world_cords = drone.get_relative_world_cords(relative_cords);
        let block_type_at_cords = world.get_world_value(world_cords);

        if BlockTexture::from_id(block_type_at_cords).is_solid() {
            // Pillering
            if relative_cords.iter().sum::<i32>() == 0 {
                if block == BlockTexture::Scaffolding {
                    move_drone(drone, world, [0, 0, 1], event_manager);
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
        if !drone.get_mut_inventory().remove_item(block.item(), block.item_quantity() as i32) {
            return 3;
        }

        // Set the block
        event_manager.add_event(WorldEvent::ModBlock(world_cords, block).wrap_into_event());

        return 0;
    }
    return 2;
}
