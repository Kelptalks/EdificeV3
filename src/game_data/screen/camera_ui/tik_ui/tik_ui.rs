use miniquad::{KeyCode, KeyMods};

use crate::game_data::{TextureManager, screen::ui_elements, tik_manager::{self, tik_manager::TikManager}, types::UITextures};

pub struct TikUI {
    // Controls
    current_tik_speed: usize,
    tik_speeds: Vec<u128>,
    paused: bool,

    // rendering
    speed_triangle_scale: f32,
    speed_triangle_spacing: f32,

    pause_scale: f32,

    scale: [f32; 2],
    ndc: [f32; 2],
}

impl TikUI {
    //=====================================
    // Controls
    //=====================================

    pub fn new()-> TikUI {
        // Setup diffrent tik speeds
        let mut tik_speeds:Vec<u128> = Vec::new();
        tik_speeds.push(50000);
        tik_speeds.push(10000);
        tik_speeds.push(5000);
        tik_speeds.push(1000);
        tik_speeds.push(500);
        
        // Init speed scale
        let speed_triangle_scale = 0.07;
        let speed_triangle_spacing = speed_triangle_scale / 5.0;
        
        // Init pause scale
        let pause_scale = 0.1;

        // Init UI element scale
        let scale = [
            (speed_triangle_scale + speed_triangle_spacing) * tik_speeds.len() as f32,
            speed_triangle_scale
        ];

        TikUI {
            // Controls
            current_tik_speed: 0,
            tik_speeds:tik_speeds,
            paused: true,

            // Rendering
            scale: scale,
            speed_triangle_scale: speed_triangle_scale,
            speed_triangle_spacing: speed_triangle_spacing,
            pause_scale: pause_scale,
            ndc: [0.0, 0.0],
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_scale(&self) -> [f32; 2] {
        self.scale
    }

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc
    }
    
    pub fn mod_tik_speed(&mut self, value: i32) {
        let new_tik_speed = (self.current_tik_speed as i32 + value) as usize;

        // If tik new tik speed out of range
        if new_tik_speed >= self.tik_speeds.len() {
            return;
        }
        // in range set tik speed
        else {
            self.current_tik_speed = new_tik_speed;
        }

    }

    //=====================================
    // Rendering
    //=====================================

    fn render_pause_button(&mut self, texture_manager: &mut TextureManager) {
        // Render pause / play 
        let pause_ndc = [
            self.ndc[0] - (self.speed_triangle_spacing + self.pause_scale as f32),
            self.ndc[1],
        ];
        let mut ui_element = UITextures::Play;
        if self.paused {
            ui_element = UITextures::Pause;
        }
        texture_manager.render_ui_element(ui_element, pause_ndc, self.pause_scale);
    }

    fn render_speed_triangles(&mut self, texture_manager: &mut TextureManager) {
        let spaced_draw_x_offset = self.speed_triangle_spacing + self.speed_triangle_scale;
        // Render all the speed triangles
        for i in 0..self.tik_speeds.len() {
            let draw_ndc = [
                self.ndc[0] + (spaced_draw_x_offset * i as f32),
                self.ndc[1],
            ];
            if i <= self.current_tik_speed {
                texture_manager.render_ui_element(UITextures::Speed_Down, draw_ndc, self.speed_triangle_scale);
            }
            else {
                texture_manager.render_ui_element(UITextures::Speed, draw_ndc, self.speed_triangle_scale);
            }
        }
    } 

    pub fn render(&mut self, texture_manager: &mut TextureManager) {    
        self.render_pause_button(texture_manager);
        self.render_speed_triangles(texture_manager);
    }

    //=====================================
    // Controls
    //=====================================

    pub fn handle_key_down(&mut self, keycode: KeyCode, tik_manager: &mut TikManager) {
        match keycode {
            KeyCode::Period => {
                self.mod_tik_speed(1);
                tik_manager.set_tik_rate(self.tik_speeds[self.current_tik_speed]);
            }
            KeyCode::Comma => {
                self.mod_tik_speed(-1);
                tik_manager.set_tik_rate(self.tik_speeds[self.current_tik_speed]);
            }
            KeyCode::Space => {
                tik_manager.pause();
                self.paused = tik_manager.is_paused(); // Update paused for rendering
                tik_manager.set_tik_rate(self.tik_speeds[self.current_tik_speed]);
            }
            _=> {}
        }
    }

    
}