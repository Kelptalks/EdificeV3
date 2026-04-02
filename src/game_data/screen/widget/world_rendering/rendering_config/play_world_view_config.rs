use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use crate::game_data::{World, locations::world_area::WorldArea, player_data::{locations::location::WorldLocation, player_data::PlayerData}, screen::widget::world_rendering::rendering_config::cursor_config::CursorConfig, types::BlockTexture};

pub enum CameraMovementType {
    ShiftArea,
    SelectArea,

}

pub struct PlayViewRenderingConfig {
    world_ref: Arc<RwLock<World>>,

    zoom: Rc<RefCell<i32>>,
    
    block_ghost: Option<Rc<RefCell<BlockTexture>>>,

    camera_movment_event_type: Rc<RefCell<usize>>,

    // Focused Location Rendering
    focused_location: Option<Rc<RefCell<WorldLocation>>>,

    // All World Location Rendering
    render_all_locations: Rc<RefCell<bool>>,
    all_locations: Rc<RefCell<Vec<Rc<RefCell<WorldLocation>>>>>,

    cursor_config: CursorConfig

}

impl PlayViewRenderingConfig {
    pub fn new(player_data: &mut PlayerData) -> Rc<RefCell<PlayViewRenderingConfig>> {
        
        let world_ref = player_data.get_world_ref();
        
        let config = PlayViewRenderingConfig {
            world_ref: world_ref,
            
            zoom: Rc::new(RefCell::new(5)),
            block_ghost: None,


            camera_movment_event_type: Rc::new(RefCell::new(0)),

            // Focused Location
            focused_location: None,

            // All World Location Rendering
            render_all_locations: Rc::new(RefCell::new(true)),
            all_locations: player_data.get_mut_location_manager().get_locations_ref_vec().clone(),


            cursor_config: CursorConfig::new(player_data),
        };

        return Rc::new(RefCell::new(config));
    }

    //=====================================
    // Main Element Getters
    //=====================================

    pub fn get_cursor_config(&self) -> &CursorConfig {
        return &self.cursor_config;
    }

    pub fn get_mut_cursor_config(&mut self) -> &mut CursorConfig {
        return &mut self.cursor_config;
    }

    //=====================================
    // Ref Setters
    //=====================================

    pub fn set_block_ghost(&mut self, block_ref: Rc<RefCell<BlockTexture>>) {
        self.block_ghost = Some(block_ref.clone())
    }

    //=====================================
    // Ref World
    //=====================================

    pub fn get_world_ref(&self) -> &Arc<RwLock<World>> {
        return &self.world_ref;
    }

    //=====================================
    // Coursor
    //=====================================

    pub fn get_cursor_location_ref(&self) -> &Rc<RefCell<WorldLocation>> {
        return &self.cursor_config.get_location_ref();
    }

    pub fn get_zoom_ref(&self) -> &Rc<RefCell<i32>> {
        return &self.zoom;
    }

    pub fn get_camera_movement_event_type_ref(&self) -> &Rc<RefCell<usize>> {
        return &self.camera_movment_event_type;
    }

    //=====================================
    // Locations
    //=====================================

    pub fn get_should_render_all_locations_ref(&self) -> &Rc<RefCell<bool>> {
        return &self.render_all_locations;
    }

    pub fn should_render_all_locations(&self) -> bool {
        return *self.render_all_locations.borrow();
    }

    pub fn get_locations_to_render(&self) -> &Rc<RefCell<Vec<Rc<RefCell<WorldLocation>>>>> {
        return &self.all_locations;
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




