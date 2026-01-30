use miniquad::{KeyCode, KeyMods};

use crate::game_data::{TextureManager, tik_manager::{self, tik_manager::TikManager}, types::UITextures};

pub struct TikUI {
    // Controls
    current_tik_speed: usize,
    tik_speeds: Vec<u128>,

    // rendering
    speed_triangle_scale: f32,
    speed_triangle_spacing: f32,
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
        tik_speeds.push(1000000);
        tik_speeds.push(100000);
        tik_speeds.push(10000);
        tik_speeds.push(1000);
        
        // Calculate scales
        let speed_triangle_scale = 0.07;
        let speed_triangle_spacing = 0.02;
        let scale = [
            (speed_triangle_scale + speed_triangle_spacing) * tik_speeds.len() as f32,
            speed_triangle_scale
        ];

        TikUI {
            // Controls
            current_tik_speed: 2,
            tik_speeds:tik_speeds,

            // Rendering
            scale: scale,
            speed_triangle_scale: speed_triangle_scale,
            speed_triangle_spacing: speed_triangle_spacing,
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

    

    //=====================================
    // Controls
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager) {    
        // Render all the triangles
        let spaced_draw_x_offset = self.speed_triangle_spacing + self.speed_triangle_scale;
        for i in 0..self.tik_speeds.len() {
            let draw_ndc = [
                self.ndc[0] + (spaced_draw_x_offset * i as f32),
                self.ndc[1],
            ];
            if i <= self.current_tik_speed {
                texture_manager.render_ui_element(UITextures::SpeedButton_Down, draw_ndc, self.speed_triangle_scale);
            }
            else {
                texture_manager.render_ui_element(UITextures::SpeedButton, draw_ndc, self.speed_triangle_scale);
            }
        }

    }

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
            }
            _=> {}
        }
    }

    //=====================================
    // Controls
    //=====================================

    
}