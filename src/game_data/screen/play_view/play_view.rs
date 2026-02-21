use std::sync::{Arc, RwLock};

use miniquad::{KeyCode, MouseButton};

use crate::game_data::{TextureManager, World, debuging::debug_data::DebugData, game_event_manager::{game_event_manager::GameEventManager, render_event_manager::render_event_manager::RenderEvent}, screen::{ScreenData, play_view::{building_gui::building_gui_manager::BuildingGUIManager, world_rendering::play_world_renderer::PlayWorldRender}, screen_data}, texture_manager, types::UITextures, world};
/*
##############
## PlayView ##
##############
Manages the view when spectating drones, building, or
any other low level block interactions

*/
pub struct PlayView {
    play_world_renderer: PlayWorldRender,
    building_gui_manager: BuildingGUIManager,

    
}

impl PlayView {
    pub fn new() -> PlayView {
        PlayView {
            play_world_renderer: PlayWorldRender::new(),
            building_gui_manager: BuildingGUIManager::new(),



        }
    }



    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        // Resize GUI
        self.building_gui_manager.window_resize_update(screen_data);

        let gui_width_ocupied = self.building_gui_manager.get_gui_pos()[2];
        let screen_width_remaining = 1.0 - gui_width_ocupied;

        let center_of_remaining_space = [
            gui_width_ocupied + (screen_width_remaining / 2.0),
            0.0,
        ];

        self.play_world_renderer.set_ndc_center_cords(center_of_remaining_space);

    }

    pub fn render_view(
        &mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager, 
        world: &Arc<RwLock<World>>
    ) {
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::VoidBackground, screen_data.get_viewport_uv());
        
        
        self.play_world_renderer.render_view(screen_data, texture_manager, world);    
        self.building_gui_manager.render_view(screen_data, texture_manager);

    }


    //=====================================
    // Controls
    //=====================================

    pub fn key_down_event(&mut self, event_manager: &mut GameEventManager, keycode: KeyCode) {
        self.play_world_renderer.key_down_event(keycode);
        
        match keycode {
            KeyCode::M => {
                event_manager.add_render_event(RenderEvent::ChangeMenu(screen_data::CurrentMenu::Camera));
            }
            _ => {
            }
        }

    }

    pub fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        self.play_world_renderer.mouse_wheel_event(_x, _y);
    }

    pub fn mouse_motion_event(&mut self, screen_data: &ScreenData) {
        self.play_world_renderer.mouse_motion_event(screen_data);   
    }

    pub fn mouse_button_down_event(&mut self, event_manager: &mut GameEventManager, screen_data: &ScreenData, button: MouseButton) {
        self.play_world_renderer.mouse_button_down_event(event_manager, screen_data, button);
        self.building_gui_manager.mouse_button_down_event(event_manager, screen_data, button);
    }

    pub fn mouse_button_up_event(&mut self, event_manager: &mut GameEventManager, screen_data: &ScreenData, button: MouseButton) {
        self.play_world_renderer.mouse_button_up_event(event_manager, screen_data, button);   
    }

    //=====================================
    // Debugging
    //=====================================

    pub fn collect_debug_data(&self, debug_data: &mut DebugData) {
        self.play_world_renderer.collect_debug_data(debug_data);
    }

}