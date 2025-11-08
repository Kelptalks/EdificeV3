

use miniquad::{GlContext, KeyCode, MouseButton};
use rand::{rng, Rng};

use crate::game_data::screen::renderer::{CameraData};
use crate::game_data::Controls::CameraControls;
use crate::game_data::Types::BlockType;
use crate::game_data::world::World;
use crate::game_data::texture_manager::TextureManager;
use crate::game_data::screen::Camera;


pub struct GameData {
    
    world : World,
    texture_manager : TextureManager,
    camera : Camera,
    camera_controls : CameraControls,
}

impl GameData {
    pub fn new() -> Self
    {
        
        // Set up texture manager
        let mut texture_manager = TextureManager::new();

        // Create camera
        let mut camera = Camera::new();
        camera.create_casted_chunks();

        // Create camera controls
        let mut camera_controls = CameraControls::new();

        // Set up world
        let mut world = World::new();
        world.generate_terrain();
        
        Self {
            world: world,
            texture_manager: texture_manager,
            camera: camera,
            camera_controls: camera_controls,
        }
    }


    /*####################################
        Control Handling
    ####################################*/
    pub fn handle_mouse_motion_input(&mut self, mouse_cords: [f32; 2]) {
        let camera_data = self.camera.get_camera_data();
        self.camera_controls.update_mouse_cords(camera_data, mouse_cords);
    }

    pub fn handle_key_inputs(&mut self, keycode: KeyCode) {
        let camera_data = self.camera.get_mut_camera_data();
        self.camera_controls.handle_key_inputs(camera_data, keycode);

    }
    pub fn handle_mouse_wheel_inputs(&mut self, x_scroll_distance: f32, y_scroll_distance: f32) {
        let camera_data = self.camera.get_mut_camera_data();
        self.camera_controls.handle_scroll_input(camera_data, y_scroll_distance);

    }
    pub fn handle_mouse_inputs(&mut self, button: MouseButton) {
        self.camera_controls.handle_mouse_inputs(&mut self.camera, button);
    }

    pub fn get_mut_camera_controls(&mut self) -> &mut CameraControls {
        return &mut self.camera_controls;
    }
    pub fn get_camera_controls(&self) -> &CameraControls {
        return &self.camera_controls;
    }

    pub fn get_mut_camera_data(&mut self) -> &mut CameraData {
        return self.camera.get_mut_camera_data()
    }

    pub fn init_textures(&mut self, ctx : &mut GlContext)
    {
        if (!self.texture_manager.are_textures_initialized()){
            self.texture_manager.init_textures(ctx);
        }
    }

    pub fn render_camera(&mut self, ctx: &mut GlContext) {
        self.camera.render_camera(&mut self.texture_manager, &self.world);
        //self.texture_manager.test_sprites(ctx);
        self.texture_manager.get_texture_renderer().flush(ctx);
    }
}
