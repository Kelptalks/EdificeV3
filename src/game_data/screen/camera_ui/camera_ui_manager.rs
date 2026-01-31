use std::{collections::HashMap, sync::{Arc, RwLock}};

use miniquad::{KeyCode, KeyMods, MouseButton};

use crate::game_data::{TextureManager, 
    World, 
    screen::{Button, ScreenData, camera_data::CameraData, camera_ui::{drone_ui::drone_ui::DroneUI, tik_ui::tik_ui::TikUI}, render_centered_string_at_ndc, text::render_string_at_ndc}, 
    tik_manager::tik_manager::TikManager, types::{BlockType, FontType, UITextures}};


/*
####################
## Camera Buttons ##
####################

*/
pub struct CameraButtons {
    // Buttons
    button_toggle_drone_ui: Button,
    drone_ui_visible: bool,

    button_kill_all_drones: Button,
    button_spawn_drones: Button,

    // Data
    button_rebuild_lua_script: Button,

}

impl CameraButtons {
    pub fn new() -> CameraButtons {
        CameraButtons {
            // buttons
            button_toggle_drone_ui: Button::new_blank(UITextures::ButtonCircle),
            button_rebuild_lua_script: Button::new_blank(UITextures::ButtonCircle),
            button_kill_all_drones: Button::new_blank(UITextures::ButtonCircle),
            button_spawn_drones: Button::new_blank(UITextures::ButtonCircle),

            // data
            drone_ui_visible: true,
        }
    }

    //=====================================
    // Getters
    //=====================================

    // Return all buttons as a mutable slice
    pub fn get_buttons_mut(&mut self) -> [&mut Button; 4] {
        [
            &mut self.button_toggle_drone_ui,
            &mut self.button_rebuild_lua_script,
            &mut self.button_kill_all_drones,
            &mut self.button_spawn_drones,
        ]
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn resize_buttons(&mut self, screen_data: &ScreenData) {
        let viewport_starting_ndc = screen_data.get_viewport_starting_ndc();
        let viewport_ending_ndc = screen_data.get_viewport_ending_ndc();

        let buttons = self.get_buttons_mut();

        let total_buttons = buttons.len() as f32;
        let scale= 0.07;
        let spacing = 0.02;        

        // Center buttons in bottom center
        let total_button_ndc_scale = (scale + spacing) * total_buttons;
        let mut button_ndc = [
            -total_button_ndc_scale / 2.0,
            viewport_ending_ndc[1] - (scale + spacing)
        ];

        // Update buttons 
        for button in buttons {
            button.set_ndc(button_ndc);
            button.set_scale(scale);
            button_ndc[0] += spacing + scale; // add space
        }


        // Set button block visuals
        self.button_toggle_drone_ui.set_block(BlockType::DroneBotRight);
        self.button_rebuild_lua_script.set_block(BlockType::Debug);
        self.button_kill_all_drones.set_block(BlockType::DroneDead);
        self.button_spawn_drones.set_block(BlockType::DroneBotRight);

        // Set button text
        self.button_toggle_drone_ui.set_text("Toggle Drone UI".to_string());
        self.button_rebuild_lua_script.set_text("Rebuild Lua Scripts".to_string());
        self.button_kill_all_drones.set_text("Kill All Drones".to_string());
        self.button_spawn_drones.set_text("Toggle Drone UI".to_string());



    }

    pub fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        // Render buttons
        let buttons = self.get_buttons_mut();
        for button in buttons {
            button.render_button(texture_manager);
        }


        // Render text if mouse on button
        let scale = self.button_toggle_drone_ui.get_scale() / 4.0;
        let ndc = [
            screen_data.get_mouse_ndc()[0],
            screen_data.get_mouse_ndc()[1] - scale,
        ];
        if self.button_toggle_drone_ui.is_mouse_on_button() {
            render_centered_string_at_ndc(texture_manager, 
                "Toggle Drone UI".to_string(), 
                FontType::Basic, 
                scale, 
                ndc
            );
        }
        // Rebuild lua scripts
        if self.button_rebuild_lua_script.is_mouse_on_button() {
            render_centered_string_at_ndc(texture_manager, 
                "Rebuild Lua Scripts".to_string(), 
                FontType::Basic, 
                scale, 
                ndc
            );
        }
    }

    //=====================================
    // Controls
    //=====================================

    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        // Buttons
        self.button_toggle_drone_ui.handle_mouse_motion_input(screen_data);
        self.button_rebuild_lua_script.handle_mouse_motion_input(screen_data);
    }

    pub fn handle_mouse_button_down(&mut self, mouse_button: MouseButton, tik_manager: &mut TikManager) {
        // buttons
        if mouse_button == MouseButton::Left {
            // Hide drone ui
            if self.button_toggle_drone_ui.is_mouse_on_button() {
                self.drone_ui_visible = !self.drone_ui_visible;
            }
            // Rebuild lua scripts
            if self.button_rebuild_lua_script.is_mouse_on_button() {
                tik_manager.get_mut_lua_manager().rebuild_drone_script();
            }
        }
    }

}

/*
#######################
## Camera UI Manager ##
######################

*/

pub struct CameraUIManager {
    
    // Buttons
    camera_buttons: CameraButtons,
    tik_ui: TikUI,
    drone_ui_map: HashMap<u32, DroneUI>,
}

impl CameraUIManager {
    pub fn new() -> CameraUIManager {
        CameraUIManager {
            drone_ui_map: HashMap::new(),
            camera_buttons: CameraButtons::new(),
            tik_ui: TikUI::new()
        }
    }

    //=====================================
    // Rendering
    //=====================================


    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        let viewport_starting_ndc = screen_data.get_viewport_starting_ndc();
        let viewport_ending_ndc = screen_data.get_viewport_ending_ndc();

        self.camera_buttons.resize_buttons(screen_data);

        // Update tik
        let tik_ui_x_scale = self.tik_ui.get_scale()[0];
        self.tik_ui.set_ndc([viewport_ending_ndc[0] - tik_ui_x_scale, viewport_starting_ndc[1]]);
    }

    pub fn render_ui(&mut self, screen_data: &ScreenData, texture_manager: &mut TextureManager, camera_data: &CameraData, world: &Arc<RwLock<World>>, tik_manager: &TikManager) {
        // Render Camera buttons
        self.camera_buttons.render(texture_manager, screen_data);

        // Render drone UI
        if self.camera_buttons.drone_ui_visible {
            let drone_manager = tik_manager.get_drone_manager();
            let drone_ids = drone_manager.get_all_drone_ids();
            let world_gaurd = world.read().unwrap();
            for drone_id in drone_ids {
                if let Some(drone_ui) = self.drone_ui_map.get_mut(&drone_id) {
                    if let Some(drone) = drone_manager.get_drone_with_id(drone_id){
                        drone_ui.render_drone_ui(texture_manager, camera_data, &world_gaurd, drone);
                    }
                }
                else {
                    self.drone_ui_map.insert(drone_id, DroneUI::new());
                }
            }
        }

        // Render Tik Speed
        self.tik_ui.render(texture_manager);

    }


    //=====================================
    // Controls
    //=====================================
    
    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        // Buttons
        self.camera_buttons.handle_motion_event(screen_data);
        
        // Drone UI
        for (id, drone_ui) in &mut self.drone_ui_map.iter_mut() {
            drone_ui.handle_motion_event(screen_data);
        }
    }

    pub fn handle_mouse_button_down(&mut self, mouse_button: MouseButton, screen_data: &ScreenData, tik_manager: &mut TikManager) {
        // buttons
        self.camera_buttons.handle_mouse_button_down(mouse_button, tik_manager);

        // drone_ui
        for (id, drone_ui) in &mut self.drone_ui_map.iter_mut() {
            drone_ui.mouse_button_down_event(mouse_button, screen_data);
        }
    }
    
    pub fn handle_mouse_button_up(&mut self, mouse_button_up: MouseButton, screen_data: &ScreenData) {
        for (id, drone_ui) in &mut self.drone_ui_map.iter_mut() {
            drone_ui.handle_mouse_button_up(mouse_button_up, screen_data);
        }
    }

    pub fn handle_key_down(&mut self, keycode: KeyCode, tik_manager: &mut TikManager) {
        self.tik_ui.handle_key_down(keycode, tik_manager);
        
    }


    
}