use std::{fs, sync::{Arc, RwLock}};

use mlua::{Function as LuaFunction, Lua};

use crate::game_data::World;

pub struct LuaManager {
    lua: Lua,
}

impl LuaManager {
    pub fn new() -> LuaManager {
        LuaManager {
            lua: Lua::new(),
        }
    }

    pub fn register_functions(&self, world: Arc<RwLock<World>>) {
        let globals = self.lua.globals();




    }

    pub fn rebuild_drone_script(&mut self) {
        let script = fs::read_to_string("Lua_scripts/main.lua")
            .expect("Failed to read script file");

        if let Err(e) = self.lua.load(&script).exec() {
            eprintln!("Failed to load Lua script: {}", e);
        }
        
    }

    pub fn tik_script(&self) { 
        let globals = self.lua.globals();
        
        // Rust figures it out from usage
        // Rust figures it out from usage
        if let Ok(tik_fn) = globals.get::<LuaFunction>("tik") {
            if let Err(e) = tik_fn.call::<()>(()) {
                eprintln!("Lua tik error: {}", e);
            }
        }
    
    }
}

