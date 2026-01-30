
use std::{sync::{Arc, RwLock}, time::{SystemTime, UNIX_EPOCH}, u128};

use crate::game_data::{World, debuging::debug_data::DebugData, screen::{Camera, screen_task_manager::drone_rendering_task_manager::DroneRenderingTaskManager}, tik_manager::drones::{drone_manager::DroneManager, lua_manager::LuaManager}, world_task_manager::world_task_manager::WorldTaskManager};

pub struct TikManager {
    paused: bool,

    // Tik Data
    current_tik: u32,
    tik_rate: u128,
    last_tik_millis: u128,
    tik_exectuion_time: u32,


    // Drones
    drone_manager: DroneManager,
    world: Arc<RwLock<World>>,
    lua_manager: LuaManager,
    
}

impl TikManager {
    pub fn new(world: Arc<RwLock<World>>) -> Self {
        let mut lua_manager = LuaManager::new();
        lua_manager.rebuild_drone_script();
        lua_manager.register_functions();
        
        Self {
            paused: true,

            // Tik Data
            current_tik: 0,
            tik_rate: 10,
            last_tik_millis: 0,
            tik_exectuion_time: 0,

            // Drones
            drone_manager: DroneManager::new(),
            world: world,
            lua_manager: lua_manager,
        }
    }

    pub fn get_drone_manager(&self) -> &DroneManager {
        return &self.drone_manager;
    }

    pub fn unpause(&mut self){
        self.paused = false;
    }

    pub fn set_tik_rate(&mut self, new_tik_rate: u128) {
        self.tik_rate = new_tik_rate;
    }

    // Called every frame to update the tik
    pub fn update_tik_manager(&mut self, world_task_manager: &mut WorldTaskManager, screen_task_manager: &mut DroneRenderingTaskManager) {
        
        
        if self.paused {
            return
        }
        
        let current_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

        // Execute a tik
        if current_millis > (self.last_tik_millis + self.tik_rate) {
            // Start tik execution time
            let tik_start_time = SystemTime::now();
            
            // Update tik time
            self.current_tik += 1;
            self.last_tik_millis = current_millis;

            
            // Temp Drone Creation
            if self.current_tik == 5 {

                self.drone_manager.create_drone_at_cords(screen_task_manager, [-25, -25, 0], "Drone 1".to_string());

                // Test navigation 
                if false {
                    let distance = 200;
                    self.drone_manager.create_drone_at_cords(screen_task_manager, [distance, distance, 0], "Drone 1".to_string());
                    self.drone_manager.create_drone_at_cords(screen_task_manager, [-distance, distance, 0], "Drone 2".to_string());
                    self.drone_manager.create_drone_at_cords(screen_task_manager, [distance, -distance, 0], "Drone 3".to_string());
                    self.drone_manager.create_drone_at_cords(screen_task_manager, [-distance, -distance, 0], "Drone 4".to_string());

                    self.drone_manager.create_drone_at_cords(screen_task_manager, [distance, 0, 0], "Drone 5".to_string());
                    self.drone_manager.create_drone_at_cords(screen_task_manager, [-distance, 0, 0], "Drone 6".to_string());
                    self.drone_manager.create_drone_at_cords(screen_task_manager, [0, -distance, 0], "Drone 7".to_string());
                    self.drone_manager.create_drone_at_cords(screen_task_manager, [-0, distance, 0], "Drone 8".to_string());
                }
            }

            // Tik drones
            self.drone_manager.tik_drones(self.world.clone(), world_task_manager, screen_task_manager);

            // Run lua tik function
            let world_gaurd = self.world.read().unwrap(); // get world lock for lua execution
            self.lua_manager.tik_script(&world_gaurd, &mut self.drone_manager, world_task_manager);
            drop(world_gaurd); // Drop gaurd after done running script

            // Execute the tasks to update the events that happend this tik
            world_task_manager.execute_tasks(self.world.clone(), screen_task_manager);

            // End tik execution time
            let system_time_end = SystemTime::now();
            let tik_duration = system_time_end.duration_since(tik_start_time).unwrap();
            self.tik_exectuion_time = tik_duration.as_millis() as u32;
        }
        

    }

    pub fn update_debug_data(&self, debug_data: &mut DebugData) {
        debug_data.set_current_tik(self.current_tik);
        debug_data.set_tik_execution_time(self.tik_exectuion_time);
    }


}