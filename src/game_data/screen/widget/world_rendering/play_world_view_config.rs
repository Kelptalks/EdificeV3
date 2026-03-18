use std::{cell::RefCell, rc::{self, Rc}, sync::{Arc, RwLock}};

use crate::game_data::{World, drone_programming::var::game_vars::game_var_type::GameVarType, player_data::locations::location::WorldLocation};

pub struct PlayViewRendingConfig {
    world_ref: Arc<RwLock<World>>,

    location: Rc<RefCell<WorldLocation>>,
    zoom: Rc<RefCell<i32>>,
    
    block_ghost: Option<Rc<RefCell<GameVarType>>>,
    
    render_cursur: bool,
    render_location_out_line: bool,
}

impl PlayViewRendingConfig {
    pub fn new(world_ref: Arc<RwLock<World>>, location: Rc<RefCell<WorldLocation>>, ) -> PlayViewRendingConfig {
        PlayViewRendingConfig {
            world_ref: world_ref,
            
            location: location,
            zoom: Rc::new(RefCell::new(5)),

            block_ghost: None,

            render_cursur: false,
            render_location_out_line: false,
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_world_ref(&self) -> &Arc<RwLock<World>> {
        return &self.world_ref;
    }

    pub fn get_location_ref(&self) -> &Rc<RefCell<WorldLocation>> {
        return &self.location;
    }

    pub fn get_zoom(&self) -> i32 {
        return *self.zoom.borrow();
    }
}




