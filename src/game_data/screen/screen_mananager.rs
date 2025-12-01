use std::sync::RwLock;

use image::imageops::FilterType::Triangle;
use miniquad::{GlContext, KeyCode, KeyMods, MouseButton, RenderingBackend};

use crate::game_data::{TextureManager, World, screen::{Camera, camera_controls, camera_data::CameraData, iso_cord_tool, renderer::casted_block_manager::casted_tile::{self, CastedTile}}};




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

pub struct ControlManager {
    mouse_pixel_cords: [i32; 2],
    mouse_ndc_cords: [f32; 2],
    mouse_renderer_pixel_cords: [i32; 2],
    mouse_renderer_ndc_cords: [f32; 2],
    mouse_iso_world_cords: [i32; 2],
}

impl ControlManager {
    pub fn new() -> Self {
        Self {
            mouse_pixel_cords: [0, 0],
            mouse_ndc_cords: [0.0, 0.0],
            mouse_renderer_pixel_cords: [0, 0],
            mouse_renderer_ndc_cords: [0.0, 0.0],
            mouse_iso_world_cords: [0, 0],
        }
    }

    pub fn get_mouse_pixel_cords(&self) -> [i32; 2] {
        return self.mouse_pixel_cords;
    }

    pub fn get_mouse_ndc_cords(&self) -> [f32; 2] {
        return self.mouse_ndc_cords;
    }

    pub fn get_renderer_mouse_pixel_cords(&self) -> [i32; 2] {
        return self.mouse_renderer_pixel_cords;
    }

    pub fn get_renderer_mouse_ndc_cords(&self) -> [f32; 2] {
        return self.mouse_renderer_ndc_cords;
    }

    pub fn get_mouse_iso_world_cords(&self) -> [i32; 2] {
        return self.mouse_iso_world_cords;
    }
    
}

pub struct ScreenManager {
    // Menu Structs
    camera : Camera,

    current_menu: CurrentMenu,
    
    // Screen Data
    screen_rez: [f32; 2],
    viewport_rez: [f32; 2],
    viewport_offset: [f32; 2],
    
    control_manager: ControlManager,


}



impl ScreenManager {
    pub fn new()->Self {
        let mut camera = Camera::new();


        let new_screen = ScreenManager{
            // Menu Structs
            camera: camera,

            // Screen Data
            current_menu: CurrentMenu::Camera,
            screen_rez: [0.0, 0.0],
            viewport_rez: [0.0, 0.0],
            viewport_offset: [0.0, 0.0],

            // Control
            control_manager: ControlManager::new(),
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

    pub fn render_screen(&mut self, texture_manager: &mut TextureManager, world: &RwLock<World>){
        // If current menu is camera
        if self.current_menu == CurrentMenu::MainMenu {
        
        }
        else if self.current_menu == CurrentMenu::Camera {
            self.camera.render_camera(texture_manager, world);
        }
    }

    //=====================================
    // Input Handling
    //=====================================

    // handle mouse movment
    pub fn mouse_motion_event(&mut self, x_cor: f32, y_cor: f32) {
        self.update_mouse_cords(x_cor, y_cor);
        self.re_calculate_mouse_cords();
        // If current menu is camera
        if self.current_menu == CurrentMenu::MainMenu {
            
        }
        else if self.current_menu == CurrentMenu::Camera {
            
        }
    }

    // Handle mouse button press
    pub fn mouse_button_down_event(&mut self, button: MouseButton) {
        self.re_calculate_mouse_cords();
        if self.current_menu == CurrentMenu::MainMenu {
            
        }
        else if self.current_menu == CurrentMenu::Camera {
            camera_controls::mouse_button_down_event(self);
        }
    }

    // Handle key press
    pub fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        self.re_calculate_mouse_cords();
        if self.current_menu == CurrentMenu::MainMenu {

        }
        else if self.current_menu == CurrentMenu::Camera {
            camera_controls::key_down_event(self, keycode, keymods, repeat);
        }
    }

    // Handle mouse wheel 
    pub fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        self.re_calculate_mouse_cords();
        if self.current_menu == CurrentMenu::MainMenu {
            
        }
        else if self.current_menu == CurrentMenu::Camera {
            camera_controls::mouse_wheel_event(self, _x, _y);
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

    pub fn get_camera_data (&self) -> &CameraData{
        return self.camera.get_camera_data();
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

    pub fn get_control_manager(&self) -> &ControlManager {
        return &self.control_manager;
    }

    pub fn pixel_cords_to_ndc_cords(&self, pixel_cords: [f32; 2]) -> [f32; 2] {
        let centered_pixel_cords = [
            pixel_cords[0] - (self.screen_rez[0] / 2.0),
            pixel_cords[1] - (self.screen_rez[1] / 2.0),
        ];

        let ndc_cords = [
            (centered_pixel_cords[0]) / (self.viewport_rez[0] / 2.0),
            (centered_pixel_cords[1]) / (self.viewport_rez[1] / 2.0)
        ];

        return ndc_cords;
    }

    //=====================================
    // Mouse / Control handling
    //=====================================

    pub fn update_mouse_cords(&mut self, x_cor: f32, y_cor: f32){
        self.control_manager.mouse_pixel_cords = [x_cor as i32, y_cor as i32];
    }

    pub fn re_calculate_mouse_cords(&mut self) {
        let x_cor = self.control_manager.mouse_pixel_cords[0] as f32;
        let y_cor = self.control_manager.mouse_pixel_cords[1] as f32;

        // MOUSE SCREEN NDC CORDS
        let y_pixel_offset = (self.viewport_rez[1] - self.screen_rez[1]) / 2.0; // Offset due to viewport centering
        let mouse_ndc_cords = [
            (x_cor / self.viewport_rez[0]) * 2.0 - 1.0,
            ((y_cor + y_pixel_offset) / self.viewport_rez[1]) * 2.0 - 1.0,
        ];

        // RERENDERER NDC CORDS
        let camera_data = self.get_camera_data();
        let draw_offset = camera_data.get_ndc_draw_offset();
        let mouse_renderer_ndc_cords = [
            (mouse_ndc_cords[0] - draw_offset[0]),
            (mouse_ndc_cords[1] - draw_offset[1]),
        ];

        // RENDERE PIXEL CORDS
        let mouse_renderer_pixel_cords = [
            mouse_renderer_ndc_cords[0] * (self.viewport_rez[0] / 2.0),
            mouse_renderer_ndc_cords[1] * (self.viewport_rez[1] / 2.0),
        ];

        // ISO CORDS
        // offset mouse cords slightly to get accurate iso cords
        let offset_mouse_ndc_cords = [
            mouse_renderer_ndc_cords[0] - camera_data.get_tile_ndi_scale(),
            mouse_renderer_ndc_cords[1],
        ];
        let iso_world_cords = iso_cord_tool::ndi_screen_cords_to_iso_cords(camera_data.get_tile_ndi_scale(), offset_mouse_ndc_cords);

        // Set all values
        self.control_manager.mouse_pixel_cords = [x_cor as i32, y_cor as i32];
        self.control_manager.mouse_ndc_cords = mouse_ndc_cords;
        self.control_manager.mouse_renderer_ndc_cords = mouse_renderer_ndc_cords;
        self.control_manager.mouse_renderer_pixel_cords = [mouse_renderer_pixel_cords[0] as i32, mouse_renderer_pixel_cords[1] as i32];
        self.control_manager.mouse_iso_world_cords = [iso_world_cords[0] as i32, iso_world_cords[1] as i32];

    }

    pub fn get_mouse_debug_casted_tile(&self) -> CastedTile {
        let control_manager = self.get_control_manager();
        let iso_cords = control_manager.get_mouse_iso_world_cords();
        let mut casted_tile = CastedTile::new(iso_cords);
        
        let triangles = casted_tile.get_mut_triangles();
        triangles[0].add_texture(crate::game_data::types::BlockType::Debug, crate::game_data::types::BlockTriangle::TopLeft);
        triangles[1].add_texture(crate::game_data::types::BlockType::Debug, crate::game_data::types::BlockTriangle::TopRight);
        return casted_tile;
    }
}
