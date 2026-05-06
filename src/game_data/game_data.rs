

use miniquad::{GlContext, KeyCode, KeyMods, MouseButton};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use crate::game_data::game_event_manager::game_event_manager::EventManager;
use crate::game_data::player_data::player_data::PlayerData;
use crate::game_data::{log_indent, log_init, log_unindent};
use crate::game_data::screen::screen_task_manager::rendering_task_manager::RenderingTaskManager;
use crate::game_data::screen::screen_mananager::ScreenManager;
use crate::game_data::tik_manager::tik_manager::TikManager;
use crate::game_data::world::World;
use crate::game_data::texture_manager::TextureManager;
use crate::game_data::world_task_manager::world_task_manager::WorldTaskManager;


pub struct GameData {
    // Other
    event_manager: EventManager,
    player_data: PlayerData,

    // World
    world : Arc<RwLock<World>>,
    world_task_manager: WorldTaskManager,
    tik_manager: TikManager,

    // Rendering
    texture_manager : TextureManager,
    screen_manager: ScreenManager,
    drone_rendering_task_manager: RenderingTaskManager,

}

impl GameData {
    pub fn new() -> Self
    {
        
        // Set up texture manager
        let texture_manager = TextureManager::new();

        // Set up world
        let world = World::new();
        let world_task_manager = WorldTaskManager::new();
        
        // Wrap world in Arc<RwLock> for thread-safe access
        let world = Arc::new(RwLock::new(world));

        // Create screen manager and configure it with the thread pool
        let screen_manager = ScreenManager::new();
        let drone_rendering_task_manager = RenderingTaskManager::new();

        // set up tik managers
        let tik_manager = TikManager::new(world.clone());


        Self {
            // Other
            event_manager: EventManager::new(),
            player_data: PlayerData::new(world.clone()),

            // World
            world: world,
            world_task_manager: world_task_manager,
            tik_manager: tik_manager,

            // Rendering
            texture_manager: texture_manager,
            screen_manager: screen_manager,
            drone_rendering_task_manager: drone_rendering_task_manager,

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
        screen.key_down_event(&mut self.event_manager, &mut self.tik_manager, keycode, keymods, repeat);
    }

    pub fn handle_mouse_wheel_inputs(&mut self, x_scroll_distance: f32, y_scroll_distance: f32) {
        // New
        let screen = &mut self.screen_manager;
        screen.mouse_wheel_event(x_scroll_distance, y_scroll_distance);

    }
    pub fn handle_mouse_inputs(&mut self, button: MouseButton) {
        // New
        let screen = &mut self.screen_manager;
        screen.mouse_button_down_event(button, &mut self.tik_manager, &mut self.world_task_manager, &mut self.event_manager);
    }

    pub fn handle_mouse_button_up(&mut self, button: MouseButton) {
        // New
        let screen = &mut self.screen_manager;
        screen.mouse_button_up_event(&mut self.event_manager, button);
    }

    //=====================================
    // Getters / Setters
    //=====================================

    //=====================================
    // Init functions
    //=====================================

    pub fn init_textures(&mut self, ctx : &mut GlContext)
    {
        if !self.texture_manager.are_textures_initialized(){
            // Get start time
            let init_start_time = SystemTime::now();

            // Init textures
            self.texture_manager.init_textures(ctx);

            // Get end time
            let system_time_end = SystemTime::now();
            let init_duration = system_time_end.duration_since(init_start_time).unwrap();
            let init_duration_ms = init_duration.as_millis();

            // Log time to init
            let log_message = format!("Textures Initilized ({}ms)", init_duration_ms);
            log_init(&log_message);
        }
    }

    pub fn init_screen_manager(&mut self, ctx : &mut GlContext){
        let init_start_time = SystemTime::now();

        log_indent();
        let (w, h) = miniquad::window::screen_size();
        self.screen_manager.init_screen(&mut self.event_manager, [w, h], ctx);
        log_unindent();

        // Get end time
        let system_time_end = SystemTime::now();
        let init_duration = system_time_end.duration_since(init_start_time).unwrap();
        let init_duration_ms = init_duration.as_millis();

        // Log time to init
        let log_message = format!("Screen Manager Initilized ({}ms)", init_duration_ms);
        log_init(&log_message);
        
    } 

    //=====================================
    // Rendering
    //=====================================

    pub fn render_camera(&mut self, ctx: &mut GlContext) {
        let frame_start_time = SystemTime::now();


        self.screen_manager.render_screen(
            &mut self.texture_manager, 
            self.world.clone(), 
            &mut self.drone_rendering_task_manager, 
            &self.tik_manager,
            &mut self.player_data,
            &mut self.event_manager,
            ctx,
        );
        
        // Tik managing
        self.tik_manager.new_update_tik_manager(&mut self.event_manager, &mut self.player_data);
        //self.tik_manager.update_tik_manager(&mut self.world_task_manager, &mut self.drone_rendering_task_manager);

        let screen_mananager = &mut self.screen_manager;
        
        // Test sprite sheet
        // self.texture_manager.test_sprites(ctx);


        // Render frame time | Eventualy create a debug window under screen for this
        let system_time_end = SystemTime::now();
        let frame_duration = system_time_end.duration_since(frame_start_time).unwrap();
        let frame_duration_ms = frame_duration.as_millis();
        


        // Update game events
        self.event_manager.execute_dispatch_events();
        self.event_manager.dispatch_input_events(screen_mananager);
        self.event_manager.execute_player_data_events(&mut self.player_data);

        //Get world gaurd
        let mut world_guard = match self.world.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                // Lock was poisoned, but we can still access the data
                eprintln!("Warning: World lock was poisoned, recovering...");
                poisoned.into_inner()
            }
        };
        self.event_manager.execute_world_events(&mut world_guard);
        
        drop(world_guard);

        self.event_manager.execute_render_events(screen_mananager, &self.player_data);
        self.event_manager.execute_widget_events();
        
        
        let screen_data = self.screen_manager.get_screen_data();

        self.texture_manager.flush(screen_data, ctx);

        // Clear inputs for this frame
        self.screen_manager.get_mut_screen_data().clear_inputs();


    }
}
