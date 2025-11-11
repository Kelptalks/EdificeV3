
mod game_data;
use std::task::Context;

use game_data::GameData;
use miniquad::*;


struct GameStage {
    ctx : GlContext,
    game_data: GameData,
}

const VIRTUAL_SIZE: f32 = 1080.0;

fn calculate_viewport(screen_width: f32, screen_height: f32) -> (f32, f32, f32, f32) {
    if screen_width > screen_height {
        // Wider screen - make viewport taller to fill width, crop top/bottom
        let viewport_size = screen_width;
        let offset_y = -(viewport_size - screen_height) / 2.0;
        (0.0, offset_y, viewport_size, viewport_size)
    } else {
        // Taller screen - make viewport wider to fill height, crop left/right
        let viewport_size = screen_height;
        let offset_x = -(viewport_size - screen_width) / 2.0;
        (offset_x, 0.0, viewport_size, viewport_size)
    }
}

impl GameStage {
    pub fn new() -> GameStage {
        
        // Create graphics context
        let mut ctx = GlContext::default();

        // Create game data and init data that requires ctx
        let mut game_data = GameData::new();
        game_data.init_textures(&mut ctx);
        game_data.init_screen_manager(&mut ctx);



        
        /*
        let (viewport_x, viewport_y, viewport_width, viewport_height) = 
        calculate_viewport(1920.0, 1080.0);

        // Set the viewport (tells GPU where to render)
        ctx.apply_viewport(
            viewport_x as i32, 
            viewport_y as i32, 
            viewport_width as i32, 
            viewport_height as i32
        );
        */


        GameStage {
            ctx: ctx,
            game_data: game_data,
        }
    }
}

impl EventHandler for GameStage {
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        self.ctx.clear(Some((0.0, 0.5, 0.8, 1.0)), None, None);
        self.game_data.render_camera(&mut self.ctx);
    }

    // Input event handlers
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.game_data.handle_mouse_motion_input([x, y]);
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        // Handle mouse button press
        self.game_data.handle_mouse_inputs(button);
    }

    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        // Handle key press
        self.game_data.handle_key_inputs(keycode, keymods, repeat);
    }

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        self.game_data.handle_mouse_wheel_inputs(_x, _y);

    }

}

fn main() {
    miniquad::start(
        conf::Conf {
            window_title: "My Game".to_owned(),
            window_width: 1920,
            window_height: 1080,
            window_resizable: true,
            ..Default::default()
        },
        || Box::new(GameStage::new()), // No parameters
    );
}