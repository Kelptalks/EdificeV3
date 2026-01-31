use std::sync::{Arc, RwLock};

use image::imageops::FilterType::Triangle;
use miniquad::{window, GlContext, KeyCode, KeyMods, MouseButton, RenderingBackend};

use crate::game_data::{TextureManager, 
    World, 
    debuging::debug_data::{self, DebugData}, 
    screen::{self, Camera, MainMenu, ScreenData, camera_controls, camera_data::{self, CameraData}, camera_ui::camera_ui_manager::CameraUIManager, iso_cord_tool, main_menu_world_creation::world_creation::WorldCreationMenu, render_centered_string_at_ndc, renderer::casted_block_manager::{casted_block_manager::CastedChunkManager, casted_tile::{self, CastedTile}}, screen_data::{self, CurrentMenu}, screen_task_manager::rendering_task_manager::RenderingTaskManager}, tik_manager::{self, tik_manager::TikManager}, types::UITextures, world_task_manager::{self, world_task_manager::WorldTaskManager}};




/*
####################
## Screen Manager ##
####################
This file is resposible for managing the game screen. It handles whitch menu is currently visibile.
how those menus are rendered and how those controls are processed.
*/

pub struct ScreenManager {
    // Menu Structs
    camera : Camera,
    main_menu : MainMenu,
    main_menu_world_creation: WorldCreationMenu,
    camera_ui_manager: CameraUIManager,

    // Screen Data
    screen_data: ScreenData,
}

impl ScreenManager {
    pub fn new()->Self {
        let camera = Camera::new();
        let screen_data = ScreenData::new();

        let new_screen = ScreenManager{
            // Menu Structs
            camera: camera,
            main_menu: MainMenu::new(),
            main_menu_world_creation: WorldCreationMenu::new(),
            camera_ui_manager: CameraUIManager::new(),

            // Screen Data
            screen_data: screen_data, 
        };
        
        
        return  new_screen;
    }

    //=====================================
    // Init functions
    //=====================================

    pub fn init_screen(&mut self, screen_rez: [f32; 2], ctx : &mut GlContext) {
        self.set_screen_rez(screen_rez, ctx);
        self.camera.initialize_camera(ctx);
    }

    //=====================================
    // menu Rendering
    //=====================================

    pub fn render_screen(&mut self, 
        texture_manager: &mut TextureManager, 
        world: Arc<RwLock<World>>, 
        world_rendering_task_manager: &mut RenderingTaskManager,
        tik_manager: &TikManager,
        ctx : &mut GlContext
    ){
        // If quit
        if self.screen_data.should_quit() {
            window::order_quit();
        }
        
        // If current menu is camera
        if self.screen_data.get_current_menu() == CurrentMenu::MainMenu {
            self.main_menu.render_main_menu(texture_manager, &self.screen_data);
        }
        if self.screen_data.get_current_menu() == CurrentMenu::MainMenuWorldCreation {
            self.main_menu_world_creation.render(texture_manager, &self.screen_data);
        }
        else if self.screen_data.get_current_menu() == CurrentMenu::Camera {
            // Generate the world if not yet initilized
            if !self.screen_data.is_world_initialized() {
                let world_config = self.screen_data.get_mut_world_config();
                world_config.init_world(texture_manager, &world, &mut self.camera, ctx);

                if world_config.done_initializing() {
                    self.screen_data.set_world_initialized(true);
                }
            }
            else {
                self.camera.render_camera(texture_manager, world.clone(), ctx);
                self.camera_ui_manager.render_ui(&self.screen_data, texture_manager, self.camera.get_camera_data(), &world.clone(), tik_manager);
                world_rendering_task_manager.execute_render_updates_drone(world, &mut self.camera, texture_manager);

            }
        }
        
    }

    //=====================================
    // Input Handling
    //=====================================

    // handle mouse movment
    pub fn mouse_motion_event(&mut self, x_cor: f32, y_cor: f32) {
        self.screen_data.set_mouse_pixel_cords([x_cor as i32, y_cor as i32]);
        // If current menu is camera
        if self.screen_data.get_current_menu() == CurrentMenu::MainMenu {
            let camera_data = &self.get_camera_data().clone();
            self.screen_data.re_calculate_mouse_cords(camera_data);
            self.main_menu.handle_mouse_motion_input(&self.screen_data);
        }
        else if self.screen_data.get_current_menu() == CurrentMenu::MainMenuWorldCreation {
            let camera_data = &self.get_camera_data().clone();
            self.screen_data.re_calculate_mouse_cords(camera_data);
            self.main_menu_world_creation.handle_mouse_motion_input(&self.screen_data);
        }
        else if self.screen_data.get_current_menu() == CurrentMenu::Camera {
            camera_controls::mouse_motion_event(self, x_cor, y_cor);
            self.camera_ui_manager.handle_motion_event(&self.screen_data);
        }
    }

    // Handle mouse button press
    pub fn mouse_button_down_event(&mut self, 
        button: MouseButton, 
        tik_manager: &mut TikManager,
        world_task_manager: &mut WorldTaskManager
    ) {
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);


        let current_menu = self.screen_data.get_current_menu();
        if current_menu == CurrentMenu::MainMenu {
            self.main_menu.handle_mouse_button_down(&mut self.screen_data, button);
        }
        else if current_menu == CurrentMenu::MainMenuWorldCreation {
            self.main_menu_world_creation.handle_mouse_button_down(&mut self.screen_data, button);
        }
        else if self.screen_data.get_current_menu() == CurrentMenu::Camera {
            camera_controls::mouse_button_down_event(self, button);
            self.camera_ui_manager.handle_mouse_button_down(button, &self.screen_data, tik_manager, world_task_manager);
        }
    }

    // Handle mouse button release
    pub fn mouse_button_up_event(&mut self, button: MouseButton) {
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);
        if self.screen_data.get_current_menu() == CurrentMenu::MainMenu {
            
        }
        else if self.screen_data.get_current_menu() == CurrentMenu::Camera {
            camera_controls::mouse_button_up_event(self, button);
            self.camera_ui_manager.handle_mouse_button_up(button, &self.screen_data);
        }
    }

    // Handle key press
    pub fn key_down_event(&mut self, tik_manager: &mut TikManager, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);

        // Handle debug menu
        match keycode {
            KeyCode::F3 => {
            self.screen_data.set_debug_visibility(!self.screen_data.get_debug_visiblity());
            }
            _ => {}
        }

        if self.screen_data.get_current_menu() == CurrentMenu::MainMenu {

        }
        else if self.screen_data.get_current_menu() == CurrentMenu::Camera {
            camera_controls::key_down_event(self, keycode, keymods, repeat);
            self.camera_ui_manager.handle_key_down(keycode, tik_manager);
        }
    }

    // Handle mouse wheel 
    pub fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);
        if self.screen_data.get_current_menu() == CurrentMenu::MainMenu {
            
        }
        else if self.screen_data.get_current_menu() == CurrentMenu::Camera {
            camera_controls::mouse_wheel_event(self, _x, _y);
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================


    pub fn get_mut_camera(&mut self) -> &mut Camera {
        return &mut self.camera;
    }

    // Camera Data
    pub fn get_mut_camera_data(&mut self) -> &mut CameraData{
        return self.camera.get_mut_camera_data();
    }
    pub fn get_camera_data(&self) -> &CameraData {
        return &self.camera.get_camera_data();
    }

    pub fn set_screen_rez(&mut self, screen_rez: [f32; 2], ctx : &mut GlContext) {
        // Calculate and setup viewport and set the correct values 
        self.screen_data.set_screen_rez(screen_rez, ctx);

        // Update viewport in camera
        self.camera.update_viewport(self.screen_data.get_viewport_rez(), self.screen_data.get_viewport_offset());

        // Update window locations
        self.camera_ui_manager.window_resize_update(&self.screen_data);
    }

    pub fn get_screen_data(&self) -> &ScreenData {
        return &self.screen_data;
    }

    pub fn get_mut_screen_data(&mut self) -> &mut ScreenData {
        return &mut self.screen_data;
    }


    //=====================================
    // Mouse / Control handling
    //=====================================

    pub fn update_mouse_cords(&mut self, x_cor: f32, y_cor: f32){
        self.screen_data.set_mouse_pixel_cords([x_cor as i32, y_cor as i32]);
    }


    pub fn get_mouse_debug_casted_tile(&self) -> CastedTile {
        let screen_data = self.get_screen_data();
        let iso_cords = screen_data.get_mouse_iso_world_cords();
        let mut casted_tile = CastedTile::new(iso_cords);
        
        let triangles = casted_tile.get_mut_triangles();
        triangles[0].add_texture(crate::game_data::types::BlockType::Debug, crate::game_data::types::BlockTriangle::TopLeft);
        triangles[1].add_texture(crate::game_data::types::BlockType::Debug, crate::game_data::types::BlockTriangle::TopRight);
        return casted_tile;
    }

    //=====================================
    // Debugging
    //=====================================

    pub fn collect_debug_data(&self, debug_data: &mut DebugData) { 
        self.camera.collect_debug_data(debug_data);

        debug_data.set_mouse_tile_cords(self.screen_data.get_mouse_iso_world_cords());
    }

}
