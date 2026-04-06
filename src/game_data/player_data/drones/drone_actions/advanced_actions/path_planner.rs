use crate::game_data::{World, player_data::drones::{drone::Drone, drone_actions::{drone_actions::DroneAction, drone_plan::DronePlan, prim_actions::{drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}}}, screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block::LairBlockMod, types::BlockTexture};





pub fn plan_path_to_cords(drone: &mut Drone, world: &World, cords: [i32; 3]) -> u32 {

    println!("Pathing to Cords");

    let mut plan = DronePlan::new();
    plan.add_action(DroneWorldAction::MoveDrone([1, 0, 0]).into());
    plan.add_action(DroneWorldAction::MoveDrone([0, 1, 0]).into());
    plan.add_action(DroneWorldAction::MoveDrone([0, 1, 0]).into());
    plan.add_action(DroneWorldAction::MoveDrone([0, 1, 0]).into());
    plan.add_action(DroneWorldAction::MoveDrone([0, 1, 0]).into());
    plan.add_action(DroneWorldAction::MoveDrone([1, 0, 0]).into());


    drone.add_plan(plan);
    drone.add_lair_block_mod(LairBlockMod::SetBlock(BlockTexture::PathingHighlight, cords));

    
    return 0;
}