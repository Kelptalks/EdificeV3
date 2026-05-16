
use std::{sync::{Arc, RwLock}, time::{Instant, SystemTime, UNIX_EPOCH}, u128};
use crate::game_data::prof_record;

use crate::game_data::{self, World, game_event_manager::prelude::EventManager, player_data::player_data::PlayerData, screen::screen_task_manager::rendering_task_manager::RenderingTaskManager, tik_manager::{block_updates::block_update_manager::BlockUpdateManager, drones::{drone_manager::DroneManager, lua_manager::LuaManager}, game_time::GameTime}, world, world_task_manager::world_task_manager::WorldTaskManager};

/*
#################
## Tik Manager ##
#################

*/
pub struct TikManager {
    paused: bool,

    // Tik Data
    current_tik: u64,
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
            paused: false,

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

            let game_time = GameTime::new(self.current_tik);

            let current_world = player_data.get_world_ref();
            


            let world = current_world.try_read();
            match world {
                Ok(world) => {

                    let debug = event_manager.get_mut_debug_data();
                    debug.clear("Tik");
                    debug.record("Tik", format!("Tik Time: {}", game_time.tik));
                    debug.record("Tik", format!("Second Time: {}", game_time.second));
                    debug.record("Tik", format!("Minute Time: {}", game_time.minute));
                    debug.record("Tik", format!("Hour Time: {}", game_time.hour));
                    debug.record("Tik", format!("Day Time: {}", game_time.day));

                    let t = Instant::now();
                    player_data.tik_game_entities(&game_time, &world, event_manager);
                    prof_record("  tik_game_entities", t.elapsed());
                },
                Err(_) => todo!(),
            }

            let t = Instant::now();
            player_data.get_mut_drone_manager().tik_drones(current_world.clone(), event_manager);
            prof_record("  tik_drones", t.elapsed());

            let mut world_gaurd = current_world.write().unwrap();

            let t = Instant::now();
            world_gaurd.tik(&game_time, player_data, event_manager);
            prof_record("  tik_world", t.elapsed());

            let t = Instant::now();
            event_manager.execute_world_events(&mut world_gaurd);
            prof_record("  tik_world_events", t.elapsed());

            // Decrement tiks left to execute
            total_tiks_to_execute-=1;
        }


        // End tik execution time
        let system_time_end = SystemTime::now();
        let tik_duration = system_time_end.duration_since(tik_start_time).unwrap();
        self.tik_window_exectuion_time = tik_duration.as_millis() as u32;

    }

}