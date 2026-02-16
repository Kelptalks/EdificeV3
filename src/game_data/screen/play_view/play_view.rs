use std::sync::{Arc, RwLock};

use miniquad::KeyCode;

use crate::game_data::{TextureManager, World, debuging::debug_data::DebugData, game_event_manager::{game_event_manager::GameEventManager, render_event_manager::render_event_manager::RenderEvent}, screen::{ScreenData, play_view::{gui::gui_manager::GUIManager, world_rendering::play_world_renderer::PlayWorldRender}, screen_data}, texture_manager, types::UITextures, world};
/*
##############
## PlayView ##
##############
Manages the view when spectating drones, building, or
any other low level block interactions

*/
pub struct PlayView {
    play_world_renderer: PlayWorldRender,
    gui_manager: GUIManager,
}

impl PlayView {
    pub fn new() -> PlayView {
        PlayView {
            play_world_renderer: PlayWorldRender::new(),
            gui_manager: GUIManager::new(),
        }
    }



    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        // Resize GUI
        self.gui_manager.window_resize_update(screen_data);
        
        // Offset the world rendering based off GUI Size
        self.play_world_renderer.set_x_offset(self.gui_manager.get_ending_ndc()[0] / 2.0);
    }

    pub fn render_view(&mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager, 
        world: &Arc<RwLock<World>>
    ) {
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::VoidBackground, screen_data.get_viewport_uv());
        
    
        self.play_world_renderer.render_view(texture_manager, world);    
        self.gui_manager.render_view(screen_data, texture_manager);

    }


    //=====================================
    // Controls
    //=====================================

    
    // Key
    pub fn key_down_event(&mut self, event_manager: &mut GameEventManager, keycode: KeyCode) {
        match keycode {
            // Rotate camera
            KeyCode::Q => {
                self.play_world_renderer.rotate_left();
            }
            KeyCode::E => {
                self.play_world_renderer.rotate_right();
            }
            
            // Vertical Camera Movement
            KeyCode::S => {
                self.play_world_renderer.mod_cords([1, 1, 0]);
            }
            KeyCode::W => {
                self.play_world_renderer.mod_cords([-1, -1, 0])
            }

            // Horozontal Camera Movemnent
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


            KeyCode::M => {
                event_manager.add_render_event(RenderEvent::ChangeMenu(screen_data::CurrentMenu::PlayView));
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