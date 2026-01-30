use miniquad::{GlContext, RenderingBackend};

use crate::game_data::{World, screen::{camera_data::CameraData, iso_cord_tool, world_config::WorldConfig}};

#[derive(Copy, Clone, PartialEq)]
pub enum CurrentMenu {
    MainMenu,
    MainMenuWorldCreation,
    Camera,

}

pub struct ScreenData {
    // Menu
    current_menu: CurrentMenu,
    debug_visible: bool,

    // Screen Data
    screen_rez: [f32; 2],
    viewport_rez: [f32; 2],
    viewport_offset: [f32; 2],

    // Mouse Cords
    mouse_pixel_cords: [i32; 2],
    mouse_ndc_cords: [f32; 2],
    mouse_renderer_pixel_cords: [i32; 2],
    mouse_renderer_ndc_cords: [f32; 2],
    mouse_iso_world_cords: [i32; 2],

    // Button held states
    middle_mouse_held: bool,

    // Init
    world_config: Option<WorldConfig>,
    is_world_initialized: bool,

    // Quit
    quit_game : bool,
}

impl ScreenData {
    pub fn new() -> ScreenData {
        ScreenData {
            // Menu
            current_menu: CurrentMenu::MainMenu,
            debug_visible: true,

            // Screen Data
            screen_rez: [0.0, 0.0],
            viewport_rez: [0.0, 0.0],
            viewport_offset: [0.0, 0.0],

            // Mouse Cords
            mouse_pixel_cords: [0, 0],
            mouse_ndc_cords: [0.0, 0.0],
            mouse_renderer_pixel_cords: [0, 0],
            mouse_renderer_ndc_cords: [0.0, 0.0],
            mouse_iso_world_cords: [0, 0],

            middle_mouse_held: false,


            // Init
            is_world_initialized: false,
            world_config: None,

            // Quit
            quit_game: false,
        }
    }

    //=====================================
    // Menu
    //=====================================

    pub fn set_current_menu(&mut self, menu: CurrentMenu) {
        self.current_menu = menu;
    }

    pub fn get_current_menu(&self) -> CurrentMenu {
        return self.current_menu;
    }

    pub fn set_debug_visibility(&mut self, visibility: bool) {
        self.debug_visible = visibility;
    }
    pub fn get_debug_visiblity(&self) -> bool {
        return self.debug_visible;
    }

    //=====================================
    // Screen 
    //=====================================

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

    pub fn get_viewport_uv(&self) -> [f32; 4] {
        let starting_cords = self.pixel_cords_to_ndc_cords([0.0, 0.0]);
        let ending_cords = self.pixel_cords_to_ndc_cords(self.get_screen_rez());

        return [starting_cords[0], starting_cords[1], ending_cords[0], ending_cords[1]];
    }

    pub fn get_viewport_starting_ndc(&self) -> [f32; 2] {
        return self.pixel_cords_to_ndc_cords([0.0, 0.0]);
    }

    pub fn get_viewport_ending_ndc(&self) -> [f32; 2] {
        return self.pixel_cords_to_ndc_cords(self.get_screen_rez());
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

    pub fn get_screen_rez(&self) -> [f32; 2] {
        return self.screen_rez;
    }

    pub fn get_viewport_rez(&self) -> [f32; 2] {
        return self.viewport_rez;
    }

    pub fn get_viewport_offset(&self) -> [f32; 2] {
        return self.viewport_offset;
    }

    //=====================================
    // Mouse / Control handling
    //=====================================

    pub fn re_calculate_mouse_cords(&mut self, camera_data: &CameraData) {
        let x_cor = self.mouse_pixel_cords[0] as f32;
        let y_cor = self.mouse_pixel_cords[1] as f32;

        // MOUSE SCREEN NDC CORDS
        let y_pixel_offset = (self.viewport_rez[1] - self.screen_rez[1]) / 2.0; // Offset due to viewport centering
        let mouse_ndc_cords = [
            (x_cor / self.viewport_rez[0]) * 2.0 - 1.0,
            ((y_cor + y_pixel_offset) / self.viewport_rez[1]) * 2.0 - 1.0,
        ];

        // RERENDERER NDC CORDS
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
            mouse_renderer_ndc_cords[0] - camera_data.get_tile_ndc_scale(),
            mouse_renderer_ndc_cords[1],
        ];
        let iso_world_cords = iso_cord_tool::ndi_screen_cords_to_iso_cords(camera_data.get_tile_ndc_scale(), offset_mouse_ndc_cords);

        // Set all values
        self.mouse_pixel_cords = [x_cor as i32, y_cor as i32];
        self.mouse_ndc_cords = mouse_ndc_cords;
        self.mouse_renderer_ndc_cords = mouse_renderer_ndc_cords;
        self.mouse_renderer_pixel_cords = [mouse_renderer_pixel_cords[0] as i32, mouse_renderer_pixel_cords[1] as i32];
        self.mouse_iso_world_cords = [iso_world_cords[0] as i32, iso_world_cords[1] as i32];

    }

    pub fn get_mouse_pixel_cords(&self) -> [i32; 2] {
        return self.mouse_pixel_cords;
    }
    
    pub fn set_mouse_pixel_cords(&mut self, cords: [i32; 2]) {
        self.mouse_pixel_cords = cords;
    }

    pub fn get_mouse_iso_world_cords(&self) -> [i32; 2] {
        return self.mouse_iso_world_cords;
    }

    pub fn is_middle_mouse_held(&self) -> bool {
        return self.middle_mouse_held;
    }

    pub fn set_middle_mouse_held(&mut self, held: bool) {
        self.middle_mouse_held = held;
    }

    pub fn get_renderer_mouse_ndc_cords(&self) -> [f32; 2] {
        return self.mouse_renderer_ndc_cords;
    }

    pub fn get_mouse_ndc_cords(&self) -> [f32; 2] {
        return self.mouse_ndc_cords;
    }

    //=====================================
    // World Initilization
    //=====================================

    pub fn is_world_initialized(&self) -> bool {
        return self.is_world_initialized;
    }

    pub fn set_world_initialized(&mut self, initialized: bool) {
        self.is_world_initialized = initialized;
    }

    pub fn set_world_config(&mut self, world_config: WorldConfig) {
        self.world_config = Some(world_config);
    }

    pub fn get_world_config(&self) -> &WorldConfig {
        self.world_config.as_ref().unwrap()
    }

    pub fn get_mut_world_config(&mut self) -> &mut WorldConfig {
        self.world_config.as_mut().unwrap()
    }

    //=====================================
    // Quitting
    //=====================================

    pub fn quit(&mut self) {
        self.quit_game = true;
    }

    pub fn should_quit(&self) -> bool {
        return self.quit_game;
    }

}