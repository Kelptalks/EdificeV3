use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use crate::game_data::{World, locations::world_area::WorldArea, player_data::{drone_script::var::var_type::VarType, locations::location::WorldLocation, player_data::PlayerData}, screen::widget::world_rendering::rendering_config::cursor_config::CursorConfig, types::BlockTexture};

pub enum CameraMovementType {
    ShiftArea,
    SelectArea,

}

pub struct PlayViewRenderingConfig {
    world_ref: Arc<RwLock<World>>,

    zoom: Rc<RefCell<i32>>,
    
    camera_movment_event_type: Rc<RefCell<usize>>,

    focused_var: Option<Rc<RefCell<VarType>>>,

    // Vars to render
    vars_to_render: Vec<Rc<RefCell<VarType>>>,

    
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
            camera_movment_event_type: Rc::new(RefCell::new(0)),

            focused_var: None,

            // Vars to render
            vars_to_render: Vec::new(),

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


    pub fn add_var_to_render(&mut self, var: Rc<RefCell<VarType>>) {
        self.vars_to_render.push(var);
    }

    //=====================================
    // Var Getters
    //=====================================

    pub fn get_focused_var(&self) -> &Option<Rc<RefCell<VarType>>> {
        &self.focused_var
    }

    pub fn set_focused_var(&mut self, var: Option<Rc<RefCell<VarType>>>) {
        self.focused_var = var
    }

    pub fn get_vars_to_render(&self) -> &Vec<Rc<RefCell<VarType>>> {
        return &self.vars_to_render;
    }

    pub fn should_render_all_location(&self) -> bool {
        return *self.render_all_locations.borrow();
    }

    pub fn get_locations_to_render(&self) -> &Rc<RefCell<Vec<Rc<RefCell<WorldLocation>>>>> {
        return &self.all_locations;
    }


    pub fn get_should_render_all_locations_ref(&self) -> &Rc<RefCell<bool>> {
        return &self.render_all_locations;
    }

    //=====================================
    // Value Getters
    //=====================================

    pub fn get_zoom(&self) -> i32 {
        return *self.zoom.borrow();
    }

}




