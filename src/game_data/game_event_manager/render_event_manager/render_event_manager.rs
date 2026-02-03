use std::sync::{Arc, RwLock};

use crate::game_data::{World, screen::{screen_mananager::{ScreenManager}}};

/*
##################
## Render Event ##
##################


*/
pub enum RenderEvent {
    InitWorldRender(u32),       // Range 

}

impl RenderEvent {
    
    //=====================================
    // Constructors 
    //=====================================

    //=====================================
    // Execution
    //=====================================
    pub fn execute_render_event(&self, screen_mananager: &mut ScreenManager, world:&Arc<RwLock<World>>) {
        let camera = screen_mananager.get_mut_camera();
        let camera_data = &camera.get_camera_data().clone();
        match self {
            RenderEvent::InitWorldRender(range) => {
                camera.dirty_chunks_in_area(
                    &camera_data, 
                    *range as i32
                );
                camera.ray_cast_dirty_chunks(camera_data.clone().get_arc_ref(), world);
            },
        }
    }
}