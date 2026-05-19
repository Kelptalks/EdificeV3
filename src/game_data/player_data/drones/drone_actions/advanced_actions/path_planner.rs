use std::collections::HashMap;

use crate::game_data::{
    World, 
    player_data::{drone_script::var::var::Var, drones::{drone::Drone, drone_actions::{drone_actions::DroneActionError, drone_plan::DronePlan, prim_actions::drone_world_actions::DroneWorldAction}}}, screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block::LairBlockMod, types::BlockTexture};


const DIRECTIONS_ALLOWED: [[i32; 3]; 12] = [
    // Cardinal XY
    [-1, 0, 0], [1, 0, 0],
    [0, -1, 0], [0, 1, 0],

    // Cardinal + Z
    [-1, 0, 1], [1, 0, 1],
    [0, -1, 1], [0, 1, 1],

    // Cardinal - Z
    [-1, 0, -1], [1, 0, -1],
    [0, -1, -1], [0, 1, -1],
];


fn get_movement_weight(world: &World, cords: [i32; 3]) -> u64 {
    let block_type = world.get_world_value_as_block(cords);

    let mut block_under_cords = cords;
    block_under_cords[2] -=1;

    let block_under = world.get_world_value_as_block(block_under_cords);

    if block_under.is_solid() {
        if !block_type.is_solid() {
            let weight = block_type.friction();
            return weight as u64;
        }
    }
    return 99999999
}

fn get_distance_weight(node_cords: [i32; 3], goal_cords: [i32; 3]) -> u64 {
    node_cords.iter()
        .zip(goal_cords.iter())
        .map(|(a, b)| (a - b).unsigned_abs() as u64)
        .sum()
}


#[derive(Clone, Copy)]
struct Node {
    world_cords: [i32; 3],
    source_node_cords: [i32; 3],
    friction_weight: u64,  // accumulated travel cost only
    distance_weight: u64,  // heuristic to goal, not accumulated
    explored: bool,
}

impl Node {
    pub fn new(source_cords: [i32; 3], cords: [i32; 3], friction_weight: u64, distance_weight: u64) -> Node {
        Node {
            world_cords: cords,
            source_node_cords: source_cords,
            friction_weight,
            distance_weight,
            explored: false,
        }
    }

    pub fn total_weight(&self) -> u64 {
        self.friction_weight + self.distance_weight
    }
}
struct NodeMap {
    node_map: HashMap<u64, Node>,
}

impl NodeMap {
    pub fn new() -> NodeMap {
        NodeMap { node_map: HashMap::new() }
    }



    pub fn explore_node(&mut self, world: &World, goal_cords: [i32; 3], node: Node) {
        let source_node_cords = node.world_cords;

        for directoin in DIRECTIONS_ALLOWED {

            let cords = [
                source_node_cords[0] + directoin[0],
                source_node_cords[1] + directoin[1],
                source_node_cords[2] + directoin[2],
            ];

            let movement_weight = get_movement_weight(world, cords) as u64;
            if movement_weight >= 100000 { continue; }

            // Only friction accumulates from parent, distance is always fresh
            let friction_weight = node.friction_weight + movement_weight;
            let distance_weight = get_distance_weight(cords, goal_cords) * 10;

            let k = cords_to_key(cords);

            if let Some(existing_node) = self.node_map.get_mut(&k) {
                if !existing_node.explored && friction_weight < existing_node.friction_weight {
                    existing_node.source_node_cords = source_node_cords;
                    existing_node.friction_weight = friction_weight;
                    existing_node.distance_weight = distance_weight;
                }
            } else {
                let new_node = Node::new(source_node_cords, cords, friction_weight, distance_weight);
                self.node_map.insert(k, new_node);
            }
        }
    }

    pub fn get_cheapest_unexplored_node(&self) -> Node {
        self.node_map.values()
            .filter(|n| !n.explored)
            .copied()
            .min_by_key(|n| n.total_weight())
            .unwrap_or(Node::new([0,0,0], [0,0,0], 99999999, 99999999))
    }
}

pub fn get_path(world: &World, start_cords: [i32; 3], goal_cords: [i32; 3]) -> Vec<[i32; 3]> {
    let max_checks = 10000;
    let mut checks = 0;

    let mut node_map = NodeMap::new();
    let mut current_node = Node::new(start_cords, start_cords, 0, 0);
    node_map.node_map.insert(cords_to_key(current_node.world_cords), current_node);

    while current_node.world_cords != goal_cords {
        if checks >= max_checks {
            eprintln!("TOO MANY CHECKS | PATHING FAILED");
            return Vec::new();
        }
        checks += 1;

        current_node = node_map.get_cheapest_unexplored_node();
        node_map.explore_node(world, goal_cords, current_node);
        // Mark current node as explored
        if let Some(n) = node_map.node_map.get_mut(&cords_to_key(current_node.world_cords)) {
            n.explored = true;
        }
    }


    // Construct path
    let mut path = Vec::new();
    while current_node.world_cords != start_cords {
        let next_node_option = node_map.node_map.get(&cords_to_key(current_node.source_node_cords));
        if let Some(next_node) = next_node_option {

            let path_directions = [
                current_node.world_cords[0] - next_node.world_cords[0],
                current_node.world_cords[1] - next_node.world_cords[1],
                current_node.world_cords[2] - next_node.world_cords[2],
            ];

            path.push(path_directions);
            
            current_node = *next_node;
        }
        else {
            eprintln!("Drone_path_planner.rs: Path Exited early due to null node");
            return path;
        }

    }

    
    return path;
}

fn cords_to_key(cords: [i32; 3]) -> u64 {
    const BITS: u32 = 21;
    const MASK: u64 = (1 << BITS) - 1; // 0x1FFFFF

    let x = (cords[0] as u64) & MASK;
    let y = (cords[1] as u64) & MASK;
    let z = (cords[2] as u64) & MASK;

    x | (y << BITS) | (z << (BITS * 2))
}

pub fn plan_path_to_cords(drone: &mut Drone, world: &World, goal_cords: [i32; 3]) -> Var {
    // Clear debug blocks
    drone.clear_lairblock_mods();
    let mut plan = DronePlan::new();
    
    let mut path_directions = get_path(world, drone.get_cords(), goal_cords);

    if path_directions.len() == 0 {
        return DroneActionError::FailedToPath.wrap_into_var_type().create_var()
    }

    // Create drone actions from direction
    while let Some(direction) = path_directions.pop() {
        plan.add_action(DroneWorldAction::MoveDrone(direction).into());
    }



    

    drone.add_plan(plan);
    drone.add_lair_block_mod(LairBlockMod::AddOverlayTexture(BlockTexture::PathingHighlight, goal_cords));

    return DroneActionError::Ok.wrap_into_var_type().create_var()
}