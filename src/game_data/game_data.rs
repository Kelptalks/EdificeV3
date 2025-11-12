

use miniquad::{GlContext, KeyCode, KeyMods, MouseButton};
use rand::{rng, Rng};

use crate::game_data::screen::renderer::{CameraData};
use crate::game_data::screen;
use crate::game_data::screen::screen_mananager::ScreenManager;
use crate::game_data::tik_manager::tik_manager::TikManager;
use crate::game_data::world::World;
use crate::game_data::texture_manager::TextureManager;
use crate::game_data::screen::Camera;


pub struct GameData {
    
    world : World,
    texture_manager : TextureManager,
    screen_manager: ScreenManager,
    tik_manager: TikManager,
}

impl GameData {
    pub fn new() -> Self
    {
        
        // Set up texture manager
        let mut texture_manager = TextureManager::new();

        // Set up world
        let mut world = World::new();
        world.generate_terrain();

        // Create screen manager
        let mut screen_manager = ScreenManager::new();

        // set up tik manager
        let mut tik_manager = TikManager::new();


        Self {
            world: world,
            texture_manager: texture_manager,
            screen_manager: screen_manager,
            tik_manager: tik_manager,

        }
    }


    //=====================================
    // Control Handling 
    //=====================================
    pub fn handle_mouse_motion_input(&mut self, x_cor: f32, y_cor: f32) {
        // New
        let screen = &mut self.screen_manager;
        screen.mouse_motion_event(x_cor, y_cor);
        
    }

    pub fn handle_key_inputs(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        // New
        let screen = &mut self.screen_manager;
        screen.key_down_event(keycode, keymods, repeat);
    }

    pub fn handle_mouse_wheel_inputs(&mut self, x_scroll_distance: f32, y_scroll_distance: f32) {
        // New
        let screen = &mut self.screen_manager;
        screen.mouse_wheel_event(x_scroll_distance, y_scroll_distance);

    }
    pub fn handle_mouse_inputs(&mut self, button: MouseButton) {
        // New
        let screen = &mut self.screen_manager;
        screen.mouse_button_down_event(button);
    }

    //=====================================
    // Getters / Setters
    //=====================================

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

        self.tik_manager.update_tik_manager();

        //self.camera.render_camera(&mut self.texture_manager, &self.world);
        //self.texture_manager.test_sprites(ctx);
        self.texture_manager.get_texture_renderer().flush(ctx);
    }
}
