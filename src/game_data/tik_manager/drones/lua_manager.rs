use std::{fs};

use mlua::{Function as LuaFunction, Lua, Result};

use crate::game_data::{World, tik_manager::drones::drone_manager::DroneManager, types::{BlockType, drone_item::DroneItem}, world_task_manager::world_task_manager::WorldTaskManager};

pub struct LuaManager {
    lua: Lua,
}

const DRONE_MANAGER_PTR: &str = "drone_manager_ptr";
const WORLD_PTR: &str = "world_ptr";
const WORLD_TASK_MANAGER_PTR: &str = "world_task_manager_ptr";

/*
#################
## Lua Manager ##
#################
This file is resposible for managing the registration of lua functions
and managing calling of the lua tik script
*/
impl LuaManager {
    pub fn new() -> LuaManager {
        LuaManager {
            lua: Lua::new(),
        }
    }

    //=====================================
    // Init
    //=====================================

    pub fn register_functions(&self) -> Result<()> {
        self.register_drone_query_functions()?;
        self.register_drone_action_functions()?;
        self.register_block_getter_functions()?;
        Ok(())
    }


    pub fn rebuild_drone_script(&mut self) {
        self.lua = Lua::new();

        // Set up the Lua package path FIRST
        if let Err(e) = self.lua.load(r#"
            package.path = "./Lua_scripts/?.lua;./Lua_scripts/?/init.lua;" .. package.path
        "#).exec() {
            eprintln!("Failed to set Lua package path: {}", e);
        }

        let script = fs::read_to_string("Lua_scripts/main.lua")
            .expect("Failed to read script file");

        if let Err(e) = self.lua.load(&script).exec() {
            eprintln!("Failed to load Lua script: {}", e);
        }

        // Register functions
        let _ = self.register_functions();
        
    }

    //=====================================
    // Pointer getters
    //=====================================

    fn get_drone_manager(lua: &Lua) -> Result<&'static mut DroneManager> {
        let ptr: usize = lua.named_registry_value(DRONE_MANAGER_PTR)?;
        Ok(unsafe { &mut *(ptr as *mut DroneManager) })
    }
    
    fn get_world(lua: &Lua) -> Result<&'static World> {
        let ptr: usize = lua.named_registry_value(WORLD_PTR)?;
        Ok(unsafe { &*(ptr as *const World) })
    }
    
    fn get_world_task_manager(lua: &Lua) -> Result<&'static mut WorldTaskManager> {
        let ptr: usize = lua.named_registry_value(WORLD_TASK_MANAGER_PTR)?;
        Ok(unsafe { &mut *(ptr as *mut WorldTaskManager) })
    }

    //=====================================
    // Drone functions
    //=====================================

    fn register_drone_query_functions(&self) -> Result<()> {
        let globals = self.lua.globals();
        
        // Get drone cords
        let get_drone_cords = self.lua.create_function(|lua, (drone_id): (u32)| {
            let drone_manager = Self::get_drone_manager(lua)?;
            if let Some(drone) = drone_manager.get_drone_with_id(drone_id) {
                return Ok(drone.get_cords());
            }
            Ok([0, 0, 0])
        })?;
        globals.set("rust_get_drone_cords", get_drone_cords)?;

        // Get all drone ids
        let get_all_drone_ids = self.lua.create_function(|lua, ()| {
            let drone_manager = Self::get_drone_manager(lua)?;
            Ok(drone_manager.get_all_drone_ids())
        })?;
        globals.set("rust_get_all_drone_ids", get_all_drone_ids)?;
        
        // Is drone busy
        let is_busy = self.lua.create_function(|lua, drone_id: u32| {
            let drone_manager = Self::get_drone_manager(lua)?;
            
            if let Some(drone) = drone_manager.get_drone_with_id(drone_id) {
                Ok(drone.is_busy())
            } else {
                println!("Cannot find drone of id {}", drone_id);
                Ok(false)
            }
        })?;
        globals.set("rust_is_busy", is_busy)?;

        // get range
        let get_vision_range = self.lua.create_function(|lua, drone_id: u32| {
            let drone_manager = Self::get_drone_manager(lua)?;
            
            if let Some(drone) = drone_manager.get_drone_with_id(drone_id) {
                return Ok(drone.get_vision_range() as i32)
            } else {
                println!("Cannot find drone of id {}", drone_id);
                Ok(-1)
            }
        })?;
        globals.set("rust_get_vision_range", get_vision_range)?;

        // get range
        let get_fuel = self.lua.create_function(|lua, drone_id: u32| {
            let drone_manager = Self::get_drone_manager(lua)?;
            
            if let Some(drone) = drone_manager.get_drone_with_id(drone_id) {
                return Ok(drone.get_fuel() as i32)
            } else {
                println!("Cannot find drone of id {}", drone_id);
                Ok(-1)
            }
        })?;
        globals.set("rust_get_fuel", get_fuel)?;

        // Scan block
        let scan_block = self.lua.create_function(|lua, (drone_id, x, y, z): (f32, f32, f32, f32)| {
            let world = Self::get_world(lua)?;
            let drone_manager = Self::get_drone_manager(lua)?;
            
            if let Some(drone) = drone_manager.get_drone_with_id(drone_id as u32) {
                let block_value = drone.scan_block(world, [x as i32, y as i32, z as i32]).id_as_u16();
                Ok(block_value)
            } else {
                println!("Cannot find drone of id {}", drone_id);
                Ok(BlockType::Debug.id_as_u16())
            }
        })?;
        globals.set("rust_scan_block", scan_block)?;
        
        // Has item amount
        let get_item_amount = self.lua.create_function(|lua, (drone_id, item_id): (u32, u32)|{
            let drone_manager = Self::get_drone_manager(lua)?;
            let mut amount = 0;
            if let Some(drone) = drone_manager.get_drone_with_id_mut(drone_id) {
                if let Some(item) = DroneItem::from_id(item_id) {
                    amount = drone.get_inventory().has_item_amount(item);
                }
            }

            Ok(amount)
        })?;
        globals.set("rust_get_item_amount", get_item_amount)?;

        Ok(())
    }
    
    fn register_drone_action_functions(&self) -> Result<()> {
        let globals = self.lua.globals();
        
        // Move drone
        let move_drone = self.lua.create_function(|lua, (drone_id, x, y, z): (u32, f32, f32, f32)| {
            let world = Self::get_world(lua)?;
            let drone_manager = Self::get_drone_manager(lua)?;
            let world_task_manager = Self::get_world_task_manager(lua)?;
            
            if let Some(drone) = drone_manager.get_drone_with_id_mut(drone_id) {
                return Ok(drone.move_drone(world, [x as i32, y as i32, z as i32], world_task_manager));
            } else {
                println!("Move Drone Failed | Cannot find drone of id {}", drone_id);
            }
            
            Ok(5)
        })?;
        globals.set("rust_move", move_drone)?;
        
        // Mine block
        let mine_block = self.lua.create_function(|lua, (drone_id, x, y, z): (u32, i32, i32, i32)| {
            let world = Self::get_world(lua)?;
            let drone_manager = Self::get_drone_manager(lua)?;
            let world_task_manager = Self::get_world_task_manager(lua)?;
            
            if let Some(drone) = drone_manager.get_drone_with_id_mut(drone_id) {
                drone.mine_block([x, y, z], world, world_task_manager);
            } else {
                println!("Mine Block Failed | Cannot find drone of id {}", drone_id);
            }
            
            Ok(())
        })?;
        globals.set("rust_mine_block", mine_block)?;

        // Mine block
        let place_block = self.lua.create_function(|lua, (drone_id, x, y, z, block_id): (u32, i32, i32, i32, u16)| {
            let drone_manager = Self::get_drone_manager(lua)?;
            let world_task_manager = Self::get_world_task_manager(lua)?;
            
            if let Some(drone) = drone_manager.get_drone_with_id_mut(drone_id) {
                drone.place_block([x, y, z], world_task_manager, BlockType::from_id(block_id));
            } else {
                println!("Place Block Failed | Cannot find drone of id {}", drone_id);
            }
            
            Ok(())
        })?;
        globals.set("rust_place_block", place_block)?;

        // Use item for fuel
        let use_item_for_fuel = self.lua.create_function(|lua, (drone_id, item_id, quantity): (u32, u32, i32)|{
            let drone_manager = Self::get_drone_manager(lua)?;
            if let Some(drone) = drone_manager.get_drone_with_id_mut(drone_id) {
                if let Some(item) = DroneItem::from_id(item_id) {
                    drone.use_item_for_fuel(item, quantity);
                }
            }

            Ok(())
        })?;
        globals.set("rust_use_item_for_fuel", use_item_for_fuel)?;
        
        // Has quantity of item checked 
        let get_item_quantity = self.lua.create_function(|lua, (drone_id, item_id): (u32, u32)|{
            let drone_manager = Self::get_drone_manager(lua)?;
            if let Some(drone) = drone_manager.get_drone_with_id_mut(drone_id) {
                if let Some(item) = DroneItem::from_id(item_id) {
                    let inventory = drone.get_inventory();
                    return Ok(inventory.has_item_amount(item));
                }
            }

            Ok(0)
        })?;
        globals.set("rust_get_item_quantity", get_item_quantity)?;

        // Craft an item
        let craft_item = self.lua.create_function(|lua, (drone_id, item_id): (u32, u32)|{
            let drone_manager = Self::get_drone_manager(lua)?;
            if let Some(drone) = drone_manager.get_drone_with_id_mut(drone_id) {
                if let Some(item) = DroneItem::from_id(item_id) {
                    drone.craft_item(item);
                }
            }

            Ok(())
        })?;
        globals.set("rust_craft_item", craft_item)?;



        Ok(())
    }

    fn register_block_getter_functions(&self) -> Result<()> {
        let globals = self.lua.globals();
        
        // Move drone
        let block_is_solid = self.lua.create_function(|lua, (block_id): (u32)| {
            Ok(BlockType::from_id(block_id as u16).is_solid())
        })?;
        globals.set("rust_block_is_solid", block_is_solid)?;


        Ok(())
    }
    
    //=====================================
    // Tik script calling
    //=====================================

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
        self.lua.set_named_registry_value(WORLD_PTR, mlua::Value::Nil).unwrap();
        self.lua.set_named_registry_value(DRONE_MANAGER_PTR, mlua::Value::Nil).unwrap();
        self.lua.set_named_registry_value(WORLD_TASK_MANAGER_PTR, mlua::Value::Nil).unwrap();
    }

}

