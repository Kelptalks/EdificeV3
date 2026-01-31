use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{ScreenData, camera_data::CameraData, text::render_string_at_ndc}, tik_manager::drones::drone::Drone, types::{DroneUITexture, FontType}};

pub struct MiniWindow {
    // Rendering
    ndc_cords: [f32; 2],
    x_scale: f32,
    y_scale: f32,

    // Stat rendering
    stat_start_ndc_cords : [f32; 2],
    tool_start_ndc_cords : [f32; 2],
    item_slot_ndc_spacing_scale : [f32; 2],

    // Controls
    pressed: bool,
}

impl MiniWindow {
    pub fn new() -> MiniWindow {
        MiniWindow {
            // Rendering
            ndc_cords: [0.0, 0.0],
            x_scale: 0.0,
            y_scale: 0.0,

            // Stat rendering
            stat_start_ndc_cords: [0.01666666666, 0.07142857142],

            tool_start_ndc_cords: [0.44166666666, 0.17857142857],
            item_slot_ndc_spacing_scale: [0.15, 0.64285714285],

            // Controls
            pressed: false,
        }  
    }

    //=====================================
    // Setters / Getters
    //=====================================

    pub fn is_pressed(&self) -> bool {
        return self.pressed;
    }
    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }


    pub fn get_scale(&self) -> f32 {
        return self.x_scale;
    }
    pub fn set_scale(&mut self, scale: f32) {
        self.x_scale = scale;

        let y_scale = DroneUITexture::DroneMiniWindow.get_y_to_x_ratio() * self.x_scale;
        self.y_scale = y_scale;
    }

    pub fn get_ndc_cords(&self) -> [f32; 2] {
        return self.ndc_cords;
    }
    
    pub fn update_drone_ndc_cords(&mut self, drone_world_cords: [i32; 3], camera_data: &CameraData) {
        let drone_ndc_cords = camera_data.world_to_ndc_cords(drone_world_cords);
        let centered_ndc_draw_cords = [drone_ndc_cords[0] - self.x_scale /2.0, drone_ndc_cords[1] - self.x_scale / 3.0];
        self.ndc_cords = centered_ndc_draw_cords;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, camera_data: &CameraData, drone: &Drone) {
        self.update_drone_ndc_cords(drone.get_cords(), camera_data);
        texture_manager.render_drone_ui_element(DroneUITexture::DroneMiniWindow, self.ndc_cords, self.x_scale);


        // Render Tools
        let scale = (self.item_slot_ndc_spacing_scale[0] * self.x_scale) * 0.75;
        let centering_offset = (self.item_slot_ndc_spacing_scale[0] * self.x_scale * 0.25) / 2.0;

        let tool_slots = drone.get_tools();
        let start_ndc_cords = self.tool_start_ndc_cords;

        for i in 0..tool_slots.len() {
            if let Some(item) = tool_slots[i] {
                let draw_location = [
                    (start_ndc_cords[0] + (i as f32 * self.item_slot_ndc_spacing_scale[0])) * self.x_scale + self.ndc_cords[0] + centering_offset,
                    (start_ndc_cords[1] * self.y_scale) + self.ndc_cords[1] + centering_offset,
                ];

                texture_manager.render_drone_item(item.to_texture_enum().unwrap(), draw_location, scale);
            }
        }


        // Render stats
        let scale = self.x_scale / 30.0;
        let spacing_scale = scale * 1.25;
        
        let mut draw_cords = [
            self.ndc_cords[0] + self.stat_start_ndc_cords[0] * self.x_scale,
            self.ndc_cords[1] + self.stat_start_ndc_cords[1] * self.y_scale
        ];

        // Render id
        let drone_id_stat = format!("ID: {}", drone.get_id());
        render_string_at_ndc(texture_manager, drone_id_stat, FontType::Basic, scale, draw_cords);

        // Render fuel
        draw_cords[1] += spacing_scale;
        let drone_id_stat = format!("Fuel: {}", drone.get_fuel());
        render_string_at_ndc(texture_manager, drone_id_stat, FontType::Basic, scale, draw_cords);




    }

    //=====================================
    // Controls
    //=====================================

    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        // if on mini_window
        
    }

    pub fn mouse_button_down_event(&mut self, button: MouseButton, screen_data: &ScreenData) {
        // Gotta calculate if mouse is on window based off scale using src rect y to x racio

        if button == MouseButton::Left {
            let y_scale = DroneUITexture::DroneMiniWindow.get_y_to_x_ratio() * self.x_scale;
            let end_ndc_cords = [self.ndc_cords[0] + self.x_scale, self.ndc_cords[1] + y_scale];

            let mouse_ndc = screen_data.get_mouse_ndc();

            let x_in_window = mouse_ndc[0] > self.ndc_cords[0] && mouse_ndc[0] < end_ndc_cords[0];
            let y_in_window = mouse_ndc[1] > self.ndc_cords[1] && mouse_ndc[1] < end_ndc_cords[1];

            if x_in_window && y_in_window {
                self.pressed = true;
            }
        }

    }

}