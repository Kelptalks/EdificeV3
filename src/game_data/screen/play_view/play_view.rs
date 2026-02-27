use std::sync::{Arc, RwLock};

use miniquad::{KeyCode, MouseButton};

use crate::game_data::{TextureManager, World, debuging::debug_data::DebugData, game_event_manager::{game_event_manager::GameEventManager, render_event_manager::render_event_manager::RenderEvent}, player_data::{self, locations::location_manager::LocationManager, player_data::PlayerData}, screen::{Button, ScreenData, play_view::{gui::{building_manager_gui::BuildingGUIManager, drone_manager_gui::DroneGUIManager, location_manager_gui::LocationManagerGUI}, play_view_data::{PlayMode, PlayViewData}, world_rendering::play_world_renderer::PlayWorldRender}, screen_data}, texture_manager, types::{BlockTexture, UITextures}, world};
/*
##############
## PlayView ##
##############
Manages the view when spectating drones, building, or
any other low level block interactions

*/
pub struct PlayView {
    play_view_data: PlayViewData,
    play_world_renderer: PlayWorldRender,


    building_manager_gui: BuildingGUIManager,
    button_building_gui_manager: Button,

    drone_manager_gui: DroneGUIManager,
    button_drone_manager_gui: Button,

    location_manager_gui: LocationManagerGUI,
    button_location_manager_gui: Button,

}

impl PlayView {
    pub fn new() -> PlayView {
        PlayView {

            play_view_data: PlayViewData::new(),
            
            // Rendering
            play_world_renderer: PlayWorldRender::new(),
            
            // UI
            building_manager_gui: BuildingGUIManager::new(),
            button_building_gui_manager: Button::new_blank(UITextures::ButtonCircle),

            drone_manager_gui: DroneGUIManager::new(),
            button_drone_manager_gui: Button::new_blank(UITextures::ButtonCircle),

            location_manager_gui: LocationManagerGUI::new(),
            button_location_manager_gui: Button::new_blank(UITextures::ButtonCircle),
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_menu_selection_buttons(&mut self) -> [&mut Button; 3] {
        return [
            &mut self.button_building_gui_manager,
            &mut self.button_drone_manager_gui,
            &mut self.button_location_manager_gui
        ];
    }

    pub fn set_play_mode(&mut self, screen_data: &ScreenData, play_mode: PlayMode) {
        self.play_view_data.set_play_mode(play_mode);
        self.re_center_play_view(screen_data);
    }

    //=====================================
    // Rendering
    //=====================================

    /// recenters the play view and menu selection buttons based off the GUI's width
    /// 
    /// # Why
    /// when menus are changed the width of each menu is diffrent requiring the world rendering
    /// to be recentered in the empty space and menu buttons selection buttons to be recentered
    /// on the edge of the gui
    pub fn re_center_play_view(&mut self, screen_data: &ScreenData) {
        let mut gui_width_ocupied = 0.0;
        let current_play_mode = self.play_view_data.get_play_mode();
        
        match current_play_mode {
            super::play_view_data::PlayMode::BuildManager => {
                gui_width_ocupied = self.building_manager_gui.get_gui_pos()[2];
            },
            super::play_view_data::PlayMode::DroneManager => {
                gui_width_ocupied = self.drone_manager_gui.get_gui_pos()[2];
            },
            PlayMode::LocationManager => {
                gui_width_ocupied = self.location_manager_gui.get_gui_pos()[2];
            },
        }
        

        let screen_width_remaining = 1.0 - gui_width_ocupied;
        let center_of_remaining_space = [
            gui_width_ocupied + (screen_width_remaining / 2.0),
            0.0,
        ];

        self.play_world_renderer.set_ndc_center_cords(center_of_remaining_space);

        // Fix button locatoin
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        let button_scale = 0.075;
        let button_spacing = button_scale / 4.0;
        let button_step = button_scale + button_spacing;

        let mut button_ndc = [
            gui_width_ocupied,
            screen_start_ndc[1] + button_spacing + self.play_view_data.get_panel_padding_scale()
        ];
        
        let buttons = self.get_menu_selection_buttons();
        for button in buttons {
            button.set_scale(button_scale);
            button.set_ndc(button_ndc);

            button_ndc[1] += button_step;
        }
        self.button_building_gui_manager.set_scale(button_scale);


    }

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        // GUI
        self.building_manager_gui.window_resize_update(screen_data, &self.play_view_data);
        self.drone_manager_gui.window_resize_update(screen_data, &self.play_view_data);
        self.location_manager_gui.window_resize_update(screen_data, &self.play_view_data);

        self.re_center_play_view(screen_data);

        // Button appearance Setup
        // Blocks
        self.button_building_gui_manager.set_block(BlockTexture::Grass);
        self.button_drone_manager_gui.set_block(BlockTexture::DroneBotRight);
        self.button_location_manager_gui.set_block(BlockTexture::Selector);

        // Text
        self.button_building_gui_manager.set_text("Building Manager".to_string());
        self.button_drone_manager_gui.set_text("Drones Manager".to_string());
        self.button_location_manager_gui.set_text("Location Manager".to_string());


    }

    pub fn render_view(
        &mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager, 
        world: &Arc<RwLock<World>>,
        player_data: &mut PlayerData,
    ) {
        // Render background
        texture_manager.render_ui_element_with_pos(UITextures::VoidBackground, screen_data.get_viewport_uv());
        
        
        // Render world
        self.play_world_renderer.render_view(screen_data, 
            &mut self.play_view_data,
            texture_manager, 
            world
        );    

        // Render ui selection buttons
        let buttons = self.get_menu_selection_buttons();
        for button in buttons {
            button.render_button(texture_manager, screen_data);
        }

        // Render ui
        match self.play_view_data.get_play_mode() {
            super::play_view_data::PlayMode::BuildManager => {
                self.building_manager_gui.render(screen_data, texture_manager);
            },
            super::play_view_data::PlayMode::DroneManager => {
                self.drone_manager_gui.render(screen_data, texture_manager);
            },
            PlayMode::LocationManager => {
                self.location_manager_gui.render(screen_data, texture_manager);
            },
        }
    }


    //=====================================
    // Controls
    //=====================================

    pub fn key_down_event(&mut self, event_manager: &mut GameEventManager, keycode: KeyCode) {
        self.play_world_renderer.key_down_event(&mut self.play_view_data, keycode);
        
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
        self.play_world_renderer.mouse_button_down_event(event_manager, &self.play_view_data, screen_data, button);

        if button == MouseButton::Left {
            if self.button_building_gui_manager.is_mouse_on_button() {
                self.set_play_mode(screen_data, PlayMode::BuildManager);
            }
            else if self.button_drone_manager_gui.is_mouse_on_button() {
                self.set_play_mode(screen_data, PlayMode::DroneManager);
            }
            else if self.button_location_manager_gui.is_mouse_on_button() {
                self.set_play_mode(screen_data, PlayMode::LocationManager);
            }
        }


        match self.play_view_data.get_play_mode() {
            super::play_view_data::PlayMode::BuildManager => {
                self.building_manager_gui.mouse_button_down_event(event_manager, &mut self.play_view_data, screen_data, button);       
            },
            super::play_view_data::PlayMode::DroneManager => {
    
            },
            PlayMode::LocationManager => {

            },
        }
    }

    pub fn mouse_button_up_event(&mut self, event_manager: &mut GameEventManager, screen_data: &ScreenData, button: MouseButton) {
        self.play_world_renderer.mouse_button_up_event(event_manager, screen_data, button);   
    }

    //=====================================
    // Debugging
    //=====================================

    pub fn collect_debug_data(&self, debug_data: &mut DebugData) {
        self.play_view_data.collect_debug_data(debug_data);
    }

}