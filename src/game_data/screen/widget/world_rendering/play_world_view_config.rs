use std::{cell::RefCell, rc::{self, Rc}, sync::{Arc, RwLock}};

use crate::game_data::{World, player_data::locations::location::WorldLocation, types::BlockTexture};

pub enum CameraMovementType {
    ShiftArea,
    SelectArea,

}

pub struct PlayViewRendingConfig {
    world_ref: Arc<RwLock<World>>,

    location: Rc<RefCell<WorldLocation>>,
    zoom: Rc<RefCell<i32>>,
    
    block_ghost: Option<Rc<RefCell<BlockTexture>>>,
    
    render_cursur: bool,
    render_location_out_line: bool,

    camera_movment_event_type: Rc<RefCell<usize>>,
}

impl PlayViewRendingConfig {
    pub fn new(world_ref: Arc<RwLock<World>>, location: Rc<RefCell<WorldLocation>>, ) -> Rc<RefCell<PlayViewRendingConfig>> {
        let config = PlayViewRendingConfig {
            world_ref: world_ref,
            
            location: location,
            zoom: Rc::new(RefCell::new(5)),

            block_ghost: None,

            render_cursur: false,
            render_location_out_line: false,

            camera_movment_event_type: Rc::new(RefCell::new(0)),
        };

        return Rc::new(RefCell::new(config));
    }

    //=====================================
    // Ref Setters
    //=====================================

    pub fn set_block_ghost(&mut self, block_ref: Rc<RefCell<BlockTexture>>) {
        self.block_ghost = Some(block_ref.clone())
    }

    //=====================================
    // Ref Getters
    //=====================================

    pub fn get_world_ref(&self) -> &Arc<RwLock<World>> {
        return &self.world_ref;
    }

    pub fn get_location_ref(&self) -> &Rc<RefCell<WorldLocation>> {
        return &self.location;
    }

    pub fn get_zoom_ref(&self) -> &Rc<RefCell<i32>> {
        return &self.zoom;
    }

    pub fn get_camera_movement_event_type_ref(&self) -> &Rc<RefCell<usize>> {
        return &self.camera_movment_event_type;
    }

    //=====================================
    // Value Getters
    //=====================================

    pub fn get_zoom(&self) -> i32 {
        return *self.zoom.borrow();
    }

    pub fn get_block_ghost(&self) -> BlockTexture {
        if let Some(block_ghost_ref) = &self.block_ghost {
            return *block_ghost_ref.borrow();
        }
        else {
            return BlockTexture::Air;
        }

    }
}




