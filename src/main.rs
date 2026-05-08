
use EdificeV3::game_data::GameData;
use image::EncodableLayout;
use image::imageops::FilterType;
use miniquad::{conf::Icon, *};


struct GameStage {
    ctx : GlContext,
    game_data: GameData,
}


impl GameStage {
    pub fn new() -> GameStage {


        // Create graphics context
        let mut ctx = GlContext::default();

        // Create game data and init data that requires ctx
        let mut game_data = GameData::new();
        game_data.init_textures(&mut ctx);
        game_data.init_screen_manager(&mut ctx);

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
        self.ctx.clear(Some((0.0, 0.0, 0.1, 1.0)), None, None);
        self.game_data.render_camera(&mut self.ctx);
    }

    // Input event handlers
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.game_data.handle_mouse_motion_input(x, y);
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

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        // Called when a mouse button is released
        self.game_data.handle_mouse_button_up(button);
    }

}

pub fn get_icon() -> Option<Icon> {
    let icon_bytes = include_bytes!("../Assets/game_icon.png");
    let img = image::load_from_memory(icon_bytes).unwrap().to_rgba8();
    
    // Create correctly-sized icons by resizing the source image to 16x16, 32x32 and 64x64
    let small_img = image::imageops::resize(&img, 16, 16, FilterType::Lanczos3);
    let medium_img = image::imageops::resize(&img, 32, 32, FilterType::Lanczos3);
    let big_img = image::imageops::resize(&img, 64, 64, FilterType::Lanczos3);

    let small_bytes = small_img.as_bytes();
    let medium_bytes = medium_img.as_bytes();
    let big_bytes = big_img.as_bytes();

    // Fixed-size arrays: 16*16*4 = 1024, 32*32*4 = 4096, 64*64*4 = 16384
    let mut small = [0u8; 16 * 16 * 4];
    let mut medium = [0u8; 32 * 32 * 4];
    let mut big = [0u8; 64 * 64 * 4];

    small.copy_from_slice(small_bytes);
    medium.copy_from_slice(medium_bytes);
    big.copy_from_slice(big_bytes);

    return Some(Icon { small, medium, big });
}

fn main() {
    
    

    miniquad::start(
        conf::Conf {
            window_title: "Edifice V3".to_owned(),
            window_width: 2560,
            window_height: 1440,
            window_resizable: false,
            icon: get_icon(),
            ..Default::default()
        },
        || Box::new(GameStage::new()), // No parameters
    );
}
