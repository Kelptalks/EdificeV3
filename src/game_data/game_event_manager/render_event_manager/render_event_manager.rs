use std::sync::{Arc, RwLock};

use crate::game_data::{World, game_event_manager::game_event_manager::EventTools, screen::{screen_data::CurrentMenu, screen_mananager::ScreenManager}};

/*
##################
## Render Event ##
##################


*/
pub enum RenderEvent {
    InitWorldRender(u32),       // Range 
    ChangeMenu(CurrentMenu),    // Current menu
    Clear,                    // None
}

impl RenderEvent {
    
    //=====================================
    // Constructors 
    //=====================================

    //=====================================
    // Execution
    //=====================================
    pub fn execute_render_event(&self, event_tools: &mut EventTools, screen_mananager: &mut ScreenManager, world:&Arc<RwLock<World>>) {
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
            RenderEvent::ChangeMenu(current_menu) => {
                let screen_data = screen_mananager.get_mut_screen_data();
                screen_data.set_current_menu(*current_menu);
            }
            RenderEvent::Clear => {
                camera.clear();
            }
        }
    }
}