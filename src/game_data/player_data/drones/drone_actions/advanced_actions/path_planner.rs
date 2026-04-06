use crate::game_data::{World, player_data::drones::{drone::Drone, drone_actions::drone_actions::DroneAction}, screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block::LairBlockMod, types::BlockTexture};



pub fn plan_path_to_cords(drone: &mut Drone, world: &World, cords: [i32; 3]) {

    println!("Pathing to Cords");

    let cords = drone.get_cords();
    drone.add_lair_block_mod(LairBlockMod::SetBlock(BlockTexture::Selector, cords));

}