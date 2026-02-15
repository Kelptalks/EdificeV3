use std::sync::{Arc, RwLock};

use miniquad::KeyCode;

use crate::game_data::{TextureManager, World, debuging::debug_data::DebugData, screen::{ScreenData, play_view::world_rendering::play_world_renderer::PlayWorldRender, screen_data}, texture_manager, types::UITextures, world};
/*
##############
## PlayView ##
##############
Manages the view when spectating drones, building, or
any other low level block interactions

*/
pub struct PlayView {
    play_world_renderer: PlayWorldRender
}

impl PlayView {
    pub fn new() -> PlayView {
        PlayView {
            play_world_renderer: PlayWorldRender::new(),

        }
    }



    //=====================================
    // Rendering
    //=====================================

    pub fn render_view(&mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager, 
        world: &Arc<RwLock<World>>
    ) {
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::MirrorBackground, screen_data.get_viewport_uv());

        self.play_world_renderer.render_view(texture_manager, world);


    }


    //=====================================
    // Controls
    //=====================================

    
    // Key
    pub fn key_down_event(&mut self, keycode: KeyCode) {
        match keycode {
            // Up and down
            KeyCode::Q => {
                self.play_world_renderer.rotate_left();
            }
            KeyCode::E => {
                self.play_world_renderer.rotate_right();
            }
            
            // Up and down
            KeyCode::S => {
                self.play_world_renderer.mod_cords([1, 1, 0]);
            }
            KeyCode::W => {
                self.play_world_renderer.mod_cords([-1, -1, 0])
            }

            // Foward and back
            KeyCode::A => {
                self.play_world_renderer.mod_cords([-1, 1, 0]);
            }
            KeyCode::D => {
                self.play_world_renderer.mod_cords([1, -1, 0])
            }

            KeyCode::Minus => {
                self.play_world_renderer.mod_zoom(-1);
            }
            KeyCode::Equal => {
                self.play_world_renderer.mod_zoom(1);
            }
            
            _ => {
            }
        }

    }


    // Mouse
    pub fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        if _y > 0.0 {
            self.play_world_renderer.mod_cords([0, 0, 1]);
        }
        else if _y < 0.0 {
            self.play_world_renderer.mod_cords([0, 0, -1])
        }
    }

    //=====================================
    // Debugging
    //=====================================

    pub fn collect_debug_data(&self, debug_data: &mut DebugData) {
        self.play_world_renderer.collect_debug_data(debug_data);
    }

}