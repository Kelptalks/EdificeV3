
use std::{sync::{Arc, RwLock}, time::{SystemTime, UNIX_EPOCH}, u128};

use crate::game_data::{World, debuging::debug_data::DebugData, game_event_manager::{event_manager, prelude::EventManager}, player_data::player_data::PlayerData, screen::{Camera, screen_task_manager::rendering_task_manager::RenderingTaskManager}, tik_manager::{block_updates::block_update_manager::{self, BlockUpdateManager}, drones::{drone_manager::DroneManager, lua_manager::LuaManager}}, world, world_task_manager::world_task_manager::WorldTaskManager};

/*
#################
## Tik Manager ##
#################

*/
pub struct TikManager {
    paused: bool,

    // Tik Data
    current_tik: u32,
    tik_rate: u128,
    last_tik_micros: u128,


    // Drones
    drone_manager: DroneManager,
    lua_manager: LuaManager,

    // Block Updates
    block_update_manager: BlockUpdateManager, 

    // World
    world: Arc<RwLock<World>>,

    // Debug
    tik_window_exectuion_time: u32,
    tiks_this_window: u32,
    
}

impl TikManager {
    pub fn new(world: Arc<RwLock<World>>) -> Self {
        let mut lua_manager = LuaManager::new();
        lua_manager.rebuild_drone_script();
        

        Self {
            paused: true,

            // Tik Data
            current_tik: 0,
            tik_rate: 10000,
            last_tik_micros: 0,

            // Drones
            drone_manager: DroneManager::new(),
            lua_manager: lua_manager,

            // Block Updates
            block_update_manager: BlockUpdateManager::new(),

            // World
            world: world,

            // Debug
            tik_window_exectuion_time: 0,
            tiks_this_window: 0,
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================
    
    // Lua
    pub fn get_mut_lua_manager(&mut self) -> &mut LuaManager {
        return &mut self.lua_manager;
    }

    // Drone manager
    pub fn get_drone_manager(&self) -> &DroneManager {
        return &self.drone_manager;
    }
    
    pub fn get_mut_drone_manager(&mut self) -> &mut DroneManager {
        return &mut self.drone_manager;
    }

    pub fn pause(&mut self){
        self.paused = !self.paused;
        self.last_tik_micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    }
    pub fn is_paused(&self) -> bool {
        return self.paused;
    }

    pub fn set_tik_rate(&mut self, new_tik_rate: u128) {
        self.tik_rate = new_tik_rate;
    }

    //=====================================
    // Init functions
    //=====================================

    pub fn new_update_tik_manager(&mut self, event_manager: &mut EventManager, player_data: &mut PlayerData) {
        if self.paused {
            return
        }

        let current_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

        // Execute a tik
        let mut total_tiks_to_execute = (current_millis - self.last_tik_micros) / self.tik_rate;
        // Cap total_tiks_to_execute
        if total_tiks_to_execute > 50 {
            total_tiks_to_execute = 50;
        }
        
        self.tiks_this_window = total_tiks_to_execute as u32;
        
        // Execute the number of tiks required this frame
        let tik_start_time = SystemTime::now(); // Start tik execution timer
        while total_tiks_to_execute > 0 {
            // Update tik time
            self.current_tik += 1;
            self.last_tik_micros = current_millis;

            let current_world = player_data.get_world_ref();
            player_data.get_mut_drone_manager().tik_drones(current_world.clone(), event_manager);


            // Decrement tiks left to execute
            total_tiks_to_execute-=1;
        }

        // End tik execution time
        let system_time_end = SystemTime::now();
        let tik_duration = system_time_end.duration_since(tik_start_time).unwrap();
        self.tik_window_exectuion_time = tik_duration.as_millis() as u32;

    }

    // Called every frame to update the tik
    pub fn update_tik_manager(&mut self, world_task_manager: &mut WorldTaskManager, screen_task_manager: &mut RenderingTaskManager) {
        if self.paused {
            return
        }
        
        let current_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

        // Execute a tik
        let mut total_tiks_to_execute = (current_millis - self.last_tik_micros) / self.tik_rate;
        // Cap total_tiks_to_execute
        if total_tiks_to_execute > 50 {
            total_tiks_to_execute = 50;
        }
        
        self.tiks_this_window = total_tiks_to_execute as u32;
        
        // Execute the number of tiks required this frame
        let tik_start_time = SystemTime::now(); // Start tik execution timer
        while total_tiks_to_execute > 0 {
            // Update tik time
            self.current_tik += 1;
            self.last_tik_micros = current_millis;

            let world_gaurd = self.world.read().unwrap(); // get world lock for lua execution

            // Tik Blocks
            if self.current_tik % 100 == 0 {
                self.block_update_manager.tik_blocks(&world_gaurd, world_task_manager);
            }

            // Tik drones
            self.drone_manager.tik_drones(self.world.clone(), world_task_manager);

            // Run lua tik function
            self.lua_manager.tik_script(&world_gaurd, &mut self.drone_manager, world_task_manager);
            
            // Block updates: Add all modified blocks and tik
            self.block_update_manager.update_blocks(&world_gaurd, world_task_manager);
            
            drop(world_gaurd); // Drop gaurd after done running script
            
            // Execute the tasks to update the events that happend this tik
            world_task_manager.execute_tasks(self.world.clone(), screen_task_manager);



            // Decrement tiks left to execute
            total_tiks_to_execute-=1;
        }

        // End tik execution time
        let system_time_end = SystemTime::now();
        let tik_duration = system_time_end.duration_since(tik_start_time).unwrap();
        self.tik_window_exectuion_time = tik_duration.as_millis() as u32;
        

    }

    pub fn update_debug_data(&self, debug_data: &mut DebugData) {
        debug_data.set_current_tik(self.current_tik);
        debug_data.set_tiks_this_window(self.tiks_this_window);
        debug_data.set_tik_window_execution_time(self.tik_window_exectuion_time);
    }


}