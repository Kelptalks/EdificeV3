use std::sync::{Arc, RwLock};

use crate::game_data::{World, screen::screen_task_manager::{self, screen_task_manager::ScreenTaskManager}, tik_manager::drones::{drone::Drone, lua_manager::LuaManager}, world_task_manager::world_task_manager::WorldTaskManager};

pub struct DroneManager{
    lua_manager: LuaManager,
    total_drones: u32,
    drones: Vec<Drone>,
}

impl DroneManager {
    pub fn new() -> DroneManager{
        let mut lua_manager = LuaManager::new();
        lua_manager.rebuild_drone_script();

        DroneManager { 
            lua_manager: lua_manager,
            total_drones: 0,
            drones: Vec::new(),
        }
    }

    pub fn tik_drones(&mut self, world: Arc<RwLock<World>>, world_task_manager: &mut WorldTaskManager, screen_task_manager: &mut ScreenTaskManager) {
        self.lua_manager.tik_script();
    
        
        for drone in &mut self.drones {
            drone.tik_drone(world.clone(), world_task_manager);
            screen_task_manager.add_drone_render_task(drone.get_cords());
        }
    
    }

    pub fn create_drone_at_cords(&mut self, cords:[i32; 3]){
        let drone = Drone::new(cords);
        self.drones.push(drone);
        self.total_drones += 1;
        println!("Created New Drone At ({:?})", cords);
    }
}