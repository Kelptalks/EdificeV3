use std::{fs, sync::{Arc, RwLock}};

use mlua::{Function as LuaFunction, Lua, Result};

use crate::game_data::{World, tik_manager::drones::drone_manager::{self, DroneManager}, types::BlockType, world_task_manager::{self, world_task_manager::WorldTaskManager}};

pub struct LuaManager {
    lua: Lua,
}

impl LuaManager {
    pub fn new() -> LuaManager {
        LuaManager {
            lua: Lua::new(),
        }
    }

    pub fn register_functions(&self) -> Result<()> {
        let globals = self.lua.globals();

        // Get all drone ids
        let get_all_drone_ids = self.lua.create_function(|lua, ()| {
            let ptr: usize = lua.named_registry_value("drone_manager_ptr")?;
            let drone_manager = unsafe { &mut *(ptr as *mut DroneManager) };

            let drone_ids: Vec<u32> = drone_manager.get_all_drone_ids();

            Ok(drone_ids)
        })?;

        self.lua.globals().set("get_all_drone_ids", get_all_drone_ids)?;



        // Get block
        let scan_block_fn = self.lua.create_function(|lua, (drone_id, x, y, z): (u32, i32, i32, i32)| {
            let world_ptr: usize = lua.named_registry_value("world_ptr")?;
            let world = unsafe { &*(world_ptr as *const World) };
            
            let ptr: usize = lua.named_registry_value("drone_manager_ptr")?;
            let drone_manager = unsafe { &mut *(ptr as *mut DroneManager) };
            
            if let Some(drone) = drone_manager.get_drone_with_id(drone_id) {
                let block_value = drone.scan_block(world, [x, y, z]).id_as_u16();
                Ok(block_value)
            } else {
                Ok(BlockType::Debug.id_as_u16())
            }
        })?;

        // Register it as a global so Lua can call it
        self.lua.globals().set("scan_block", scan_block_fn)?;

        // if busy

        // mine_block

        // place_block

        // 


        Ok(())
    }

    pub fn rebuild_drone_script(&mut self) {
        let script = fs::read_to_string("Lua_scripts/main.lua")
            .expect("Failed to read script file");

        if let Err(e) = self.lua.load(&script).exec() {
            eprintln!("Failed to load Lua script: {}", e);
        }
        
    }

        pub fn tik_script(&self, world: &World, drone_manager: &mut DroneManager, world_task_manager: &mut WorldTaskManager) { 
            let globals = self.lua.globals();


            // Set as light userdata in registry
            self.lua.set_named_registry_value("world_ptr", 
                world as *const World as usize).unwrap();
            
            self.lua.set_named_registry_value("drone_manager_ptr", 
                drone_manager as *mut DroneManager as usize).unwrap();  // Fixed: *mut DroneManager
            
            self.lua.set_named_registry_value("world_task_manager_ptr", 
                world_task_manager as *mut WorldTaskManager as usize).unwrap();  // Added

            // Execute the tik function
            if let Ok(tik_fn) = globals.get::<LuaFunction>("tik") {
                if let Err(e) = tik_fn.call::<()>(()) {
                    eprintln!("Lua tik error: {}", e);
                }
            }


            // Clear pointers for safety after execution
            self.lua.set_named_registry_value("world_ptr", mlua::Value::Nil).unwrap();
            self.lua.set_named_registry_value("drone_manager_ptr", mlua::Value::Nil).unwrap();
            self.lua.set_named_registry_value("world_task_manager_ptr", mlua::Value::Nil).unwrap();
        
        }
}

