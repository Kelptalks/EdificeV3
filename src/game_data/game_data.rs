

use miniquad::{GlContext, KeyCode, MouseButton};
use rand::{rng, Rng};

use crate::game_data::screen::renderer::{CameraData};
use crate::game_data::controls::CameraControls;
use crate::game_data::screen;
use crate::game_data::screen::screen_mananager::ScreenManager;
use crate::game_data::world::World;
use crate::game_data::texture_manager::TextureManager;
use crate::game_data::screen::Camera;


pub struct GameData {
    
    world : World,
    texture_manager : TextureManager,
    screen_manager: ScreenManager,
    camera_controls : CameraControls,
}

impl GameData {
    pub fn new() -> Self
    {
        
        // Set up texture manager
        let mut texture_manager = TextureManager::new();


        // Create camera controls
        let mut camera_controls = CameraControls::new();

        // Set up world
        let mut world = World::new();
        world.generate_terrain();

        // Create screen manager
        let mut screen_manager = ScreenManager::new();
        
        Self {
            world: world,
            texture_manager: texture_manager,
            screen_manager: screen_manager,
            camera_controls: camera_controls,

        }
    }


    //=====================================
    // Control Handling 
    //=====================================
    pub fn handle_mouse_motion_input(&mut self, mouse_cords: [f32; 2]) {
        let camera_data = self.screen_manager.get_mut_camera_data();
        self.camera_controls.update_mouse_cords(camera_data, mouse_cords);
    }

    pub fn handle_key_inputs(&mut self, keycode: KeyCode) {
        let camera_data = self.screen_manager.get_mut_camera_data();
        self.camera_controls.handle_key_inputs(camera_data, keycode);

    }
    pub fn handle_mouse_wheel_inputs(&mut self, x_scroll_distance: f32, y_scroll_distance: f32) {
        let camera_data = self.screen_manager.get_mut_camera_data();
        self.camera_controls.handle_scroll_input(camera_data, y_scroll_distance);

    }
    pub fn handle_mouse_inputs(&mut self, button: MouseButton) {
        self.camera_controls.handle_mouse_inputs(&mut self.screen_manager.get_mut_camera(), button);
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_mut_camera_controls(&mut self) -> &mut CameraControls {
        return &mut self.camera_controls;
    }
    pub fn get_camera_controls(&self) -> &CameraControls {
        return &self.camera_controls;
    }

    pub fn get_mut_camera_data(&mut self) -> &mut CameraData {
        return self.screen_manager.get_mut_camera_data()
    }

    //=====================================
    // Init functions
    //=====================================

    pub fn init_textures(&mut self, ctx : &mut GlContext)
    {
        if (!self.texture_manager.are_textures_initialized()){
            self.texture_manager.init_textures(ctx);
        }
    }

    pub fn init_screen_manager(&mut self, ctx : &mut GlContext){
        self.screen_manager.init_screen([1920.0, 1080.0], ctx);
    } 

    //=====================================
    // Rendering
    //=====================================

    pub fn render_camera(&mut self, ctx: &mut GlContext) {
        
        self.screen_manager.render_screen(&mut self.texture_manager, &self.world);


        //self.camera.render_camera(&mut self.texture_manager, &self.world);
        //self.texture_manager.test_sprites(ctx);
        self.texture_manager.get_texture_renderer().flush(ctx);
    }
}
