use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use crate::game_data::{World, player_data::{drone_script::var::var::Var, player_data::PlayerData}, screen::widget::world_rendering::rendering_config::cursor_config::CursorConfig};

#[derive(Clone)]
pub enum RenderMode {
    All,
    VarOnly(Var),
}

pub struct PlayViewRenderingConfig {
    world_ref: Arc<RwLock<World>>,

    render_mode: RenderMode,
    zoom: Rc<RefCell<i32>>,
    
    camera_movment_event_type: Rc<RefCell<usize>>,

    focused_var: Option<Var>,

    

    // Vars to render
    vars_to_render: Vec<Var>,

    
    // All World Location Rendering
    render_all_locations: Rc<RefCell<bool>>,

    

    cursor_config: CursorConfig

}

impl PlayViewRenderingConfig {
    pub fn new(player_data_ref: &PlayerData) -> Rc<RefCell<PlayViewRenderingConfig>> {
  
        let world_ref = player_data_ref.get_world_ref();

        let config = PlayViewRenderingConfig {
            world_ref: world_ref,
            
            zoom: Rc::new(RefCell::new(5)),
            camera_movment_event_type: Rc::new(RefCell::new(0)),

            focused_var: None,

            render_mode: RenderMode::All,

            // Vars to render
            vars_to_render: Vec::new(),

            // All World Location Rendering
            render_all_locations: Rc::new(RefCell::new(true)),


            cursor_config: CursorConfig::new(),
        };

        return Rc::new(RefCell::new(config));
    }

    pub fn set_rendering_mode(&mut self, render_mode: RenderMode) {
        self.render_mode = render_mode;
    }

    pub fn get_render_mode(&self) -> &RenderMode {
        &self.render_mode
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

    pub fn get_zoom_ref(&self) -> &Rc<RefCell<i32>> {
        return &self.zoom;
    }

    pub fn get_camera_movement_event_type_ref(&self) -> &Rc<RefCell<usize>> {
        return &self.camera_movment_event_type;
    }

    pub fn add_var_to_render(&mut self, var: Var) {
        self.vars_to_render.push(var);
    }

    //=====================================
    // Var Getters
    //=====================================

    pub fn get_focused_var(&self) -> &Option<Var> {
        &self.focused_var
    }

    pub fn set_focused_var(&mut self, var: Option<Var>) {
        self.focused_var = var
    }

    pub fn get_vars_to_render(&self) -> &Vec<Var> {
        return &self.vars_to_render;
    }

    pub fn should_render_all_location(&self) -> bool {
        return *self.render_all_locations.borrow();
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




