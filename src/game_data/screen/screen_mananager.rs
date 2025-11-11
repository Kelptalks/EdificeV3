use miniquad::{GlContext, KeyCode, KeyMods, MouseButton, RenderingBackend};

use crate::game_data::{TextureManager, World, controls::CameraControls, screen::{Camera, CameraData}};




#[derive(Copy, Clone, PartialEq)]
enum CurrentMenu {
    MainMenu,
    Camera,

}

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
    camera_controls : CameraControls,



    current_menu: CurrentMenu,
    
    // Screen Data
    screen_rez: [f32; 2],
    viewport_rez: [f32; 2],
    viewport_offset: [f32; 2],

}



impl ScreenManager {
    pub fn new()->Self {

        let mut camera = Camera::new();
        camera.create_casted_chunks();

        let camera_controls = CameraControls::new();


        let new_screen = ScreenManager{
            // Menu Structs
            camera: camera,
            camera_controls: camera_controls,

            // Screen Data
            current_menu: CurrentMenu::Camera,
            screen_rez: [0.0, 0.0],
            viewport_rez: [0.0, 0.0],
            viewport_offset: [0.0, 0.0],
        };
        
        return  new_screen;
    }

    //=====================================
    // Init functions
    //=====================================

    pub fn init_screen(&mut self, screen_rez: [f32; 2], ctx : &mut GlContext) {
        self.set_screen_rez(screen_rez, ctx);
    }

    //=====================================
    // menu Rendering
    //=====================================

    pub fn render_screen(&mut self, texture_manager: &mut TextureManager, world: &World){
        // If current menu is camera
        if self.current_menu == CurrentMenu::MainMenu {
            
        }
        else if self.current_menu == CurrentMenu::Camera {
            self.camera.render_camera(texture_manager, world);
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    // handle mouse movment
    pub fn mouse_motion_event(&mut self, mouse_cords: [f32; 2]) {
        // If current menu is camera
        if self.current_menu == CurrentMenu::MainMenu {
            
        }
        else if self.current_menu == CurrentMenu::Camera {
            
        }
    }

    // Handle mouse button press
    pub fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        if self.current_menu == CurrentMenu::MainMenu {
            
        }
        else if self.current_menu == CurrentMenu::Camera {
            
        }
    }

    // Handle key press
    pub fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        if self.current_menu == CurrentMenu::MainMenu {
            
        }
        else if self.current_menu == CurrentMenu::Camera {
            
        }
    }

    // Handle mouse wheel 
    pub fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        if self.current_menu == CurrentMenu::MainMenu {
            
        }
        else if self.current_menu == CurrentMenu::Camera {
            
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_mut_camera(&mut self) -> &mut Camera {
        return &mut self.camera;
    }

    pub fn get_mut_camera_data(&mut self) -> &mut CameraData{
        return self.camera.get_mut_camera_data();
    }

    pub fn set_screen_rez(&mut self, screen_rez: [f32; 2], ctx : &mut GlContext) {
        // Calculate and setup viewport and set the correct values 
        let mut viewport_size: f32;

        // Set up square viewport to allow for consistant rendering
        if screen_rez[0] > screen_rez[1] {
            viewport_size = screen_rez[0];
            self.viewport_offset = [0.0, -(viewport_size - screen_rez[1]) / 2.0];
        } else {
            viewport_size = screen_rez[1];
            self.viewport_offset = [-(viewport_size - screen_rez[0]) / 2.0, 0.0];
        }

        self.screen_rez = screen_rez;
        self.viewport_rez = [viewport_size, viewport_size];

        // set the viewport using ctx
        ctx.apply_viewport(
            self.viewport_offset[0] as i32, 
            self.viewport_offset[1] as i32, 
            self.viewport_rez[0] as i32, 
            self.viewport_rez[1] as i32, 
        );
    }

    //=====================================
    // Conversion
    //=====================================

    pub fn ndi_cords_to_pixel_cords(&self, ndi_cords: [f32; 2]) -> [i32; 2] {
        // Convert NDI coordinates (-1.0 to 1.0) to screen pixel coordinates
        // First convert NDI to normalized viewport space (0.0 to 1.0)
        let norm_x = (ndi_cords[0] + 1.0) / 2.0;
        let norm_y = (ndi_cords[1] + 1.0) / 2.0;
        
        // Convert to viewport pixel coordinates (0 to viewport_rez)
        let viewport_x = norm_x * self.viewport_rez[0];
        let viewport_y = norm_y * self.viewport_rez[1];
        
        // Convert viewport pixels to screen pixels by adding the viewport offset
        // The offset is negative when the viewport extends off-screen, shifting visible pixels
        // For example: viewport pixel 100 with offset -50 = screen pixel 50
        let screen_x = (viewport_x + self.viewport_offset[0]) as i32;
        let screen_y = (viewport_y - self.viewport_offset[1].abs()) as i32;
        
        return [screen_x, screen_y];
    }

}
