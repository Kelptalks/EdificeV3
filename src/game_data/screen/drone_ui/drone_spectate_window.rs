use std::{fmt::format, sync::{Arc, RwLock}};

use miniquad::MouseButton;

use crate::game_data::{TextureManager, World, screen::{ScreenData, iso_cord_tool, render_centered_string_at_ndi_cords, text::render_string_at_ndi_cords}, tik_manager::drones::{drone::Drone, drone_inventory}, types::{BlockType, DroneUITexture}};

pub struct SpectateWindow {
    ndc_cords: [f32; 2],
    end_ndc_cords: [f32; 2],
    x_scale: f32,
    y_scale: f32,
    visible: bool,

    // Dragging
    bar_grabbed: bool,
    window_ndc_bar_grabbed_cords: [f32; 2],

    // Statas
    stats_ndc_start_cords: [f32; 2],

    // tools
    tool_ndc_start_cords: [f32; 2],

    // Inventory
    inventory_ndc_start_cords: [f32; 2],
    item_slot_ndc_spacing_scale: [f32; 2],

    // Drone spectate data
    block_display_window_ndc_cords: [f32; 2],
    spectate_zoom: i32,
}

impl SpectateWindow {
    pub fn new() -> SpectateWindow {
        SpectateWindow {
            // rendering data
            ndc_cords: [0.0, 0.0],
            end_ndc_cords: [0.0, 0.0],
            x_scale: 0.7,
            y_scale: 0.0,
            visible: false,

            // Dragging
            bar_grabbed: false,
            window_ndc_bar_grabbed_cords: [0.0, 0.0],

            // Tool
            tool_ndc_start_cords: [0.76045627376, 0.10714285714],

            // Stats
            stats_ndc_start_cords: [0.76045627376, 0.61734693877],

            // Inventory
            inventory_ndc_start_cords: [0.76045627376, 0.27551020408],
            item_slot_ndc_spacing_scale: [0.072243346, 0.09693877551],

            // Drone spectate data
            block_display_window_ndc_cords: [0.37, 0.5],
            spectate_zoom: 2,
        }
    }


    //=====================================
    // Setters / Getters
    //=====================================

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    pub fn is_visible(&self) -> bool {
        return self.visible;
    }

    pub fn set_ndc_cords(&mut self, ndc_cords: [f32; 2]) {
        self.ndc_cords = ndc_cords;
        let y_scale = DroneUITexture::DroneSpectateWindow.get_y_to_x_ratio() * self.x_scale;
        self.y_scale = y_scale;

        self.end_ndc_cords = [self.ndc_cords[0] + self.x_scale, self.ndc_cords[1] + y_scale];

    }

    pub fn get_mouse_pixel_cords_on_window(&self, mouse_ndc: [f32; 2]) -> [f32; 2]{
        // Calculate pixel cords for clicking buttons
        let src_rect = DroneUITexture::DroneSpectateWindow.ui_texture_to_sprite_sheet_src_rect();
        let x_mouse_pixel_cords_on_window = ((mouse_ndc[0] - self.ndc_cords[0]) / self.x_scale) * src_rect[2] as f32;
        let y_mouse_pixel_cords_on_window = ((mouse_ndc[1] - self.ndc_cords[1]) / self.y_scale) * src_rect[3] as f32;
        return [x_mouse_pixel_cords_on_window, y_mouse_pixel_cords_on_window];
    }

    
    //=====================================
    // Rendering
    //=====================================

    pub fn render(&self, texture_manager: &mut TextureManager, world: &World, drone: &Drone) {
        texture_manager.render_drone_ui_element(DroneUITexture::DroneSpectateWindow, self.ndc_cords, self.x_scale);

        let scale = self.y_scale / (6.5 * self.spectate_zoom as f32);
        let x_draw_location = ((self.block_display_window_ndc_cords[0] * self.x_scale) + self.ndc_cords[0]) - scale;
        let y_draw_location = ((self.block_display_window_ndc_cords[1] * self.y_scale) + self.ndc_cords[1]) - scale;

        // Render blocks around drone
        for z_block_cor in -self.spectate_zoom..=self.spectate_zoom {
            for x_block_cor in -self.spectate_zoom..=self.spectate_zoom {
                for y_block_cor in -self.spectate_zoom..=self.spectate_zoom {
                    let draw_offset = iso_cord_tool::casted_to_ndc_cords(
                        scale,
                        [x_block_cor - z_block_cor, y_block_cor - z_block_cor]
                    );

                    let final_draw_location = [draw_offset[0] + x_draw_location, draw_offset[1] + y_draw_location];
                    let drone_cords = drone.get_cords();


                    let block_cords = [
                        drone_cords[0] + x_block_cor,
                        drone_cords[1] + y_block_cor,
                        drone_cords[2] + z_block_cor,
                    ];
                    let block = BlockType::from_id(world.get_world_value(block_cords));

                    texture_manager.render_block(block, final_draw_location, scale);
                }
            }   
        }


        // Setup item rendering values
        let scale = (self.item_slot_ndc_spacing_scale[0] * self.x_scale) * 0.75;
        let centering_offset = (self.item_slot_ndc_spacing_scale[0] * self.x_scale * 0.25) / 2.0;

        // Render invintory 
        let drone_inventory = drone.get_inventory();
        let text_scale = scale / 3.0;
        let start_ndc_cords = self.inventory_ndc_start_cords;
        let text_offset = scale / 2.0;
        
        for y_slot in 0..3 {
            for x_slot in 0..3 {
                let slot_index = y_slot * 3 + x_slot;
                if let Some(inventory_slot) = drone_inventory.get_slot_at_index(slot_index) {
                    if let Some(drone_item) = inventory_slot.get_item() {
                        let draw_location = [
                            (start_ndc_cords[0] + (x_slot as f32 * self.item_slot_ndc_spacing_scale[0])) * self.x_scale + self.ndc_cords[0] + centering_offset,
                            (start_ndc_cords[1] + (y_slot as f32 * self.item_slot_ndc_spacing_scale[1])) * self.y_scale + self.ndc_cords[1] + centering_offset,
                        ];

                        texture_manager.render_drone_item(drone_item.to_texture_enum().unwrap(), draw_location, scale);

                        let text_draw_location = [
                            draw_location[0] + text_offset,
                            draw_location[1] + centering_offset,
                        ];
                        render_centered_string_at_ndi_cords(texture_manager, inventory_slot.get_quantity().to_string(), "Basic".to_string(), text_scale, text_draw_location);
                    }
                }
            }
        }

        // Render tools
        let tool_slots = drone.get_tools();
        let start_ndc_cords = self.tool_ndc_start_cords;

        for i in 0..tool_slots.len() {
            if let Some(item) = tool_slots[i] {
                let draw_location = [
                    (start_ndc_cords[0] + (i as f32 * self.item_slot_ndc_spacing_scale[0])) * self.x_scale + self.ndc_cords[0] + centering_offset,
                    (start_ndc_cords[1] * self.y_scale) + self.ndc_cords[1] + centering_offset,
                ];

                texture_manager.render_drone_item(item.to_texture_enum().unwrap(), draw_location, scale);
            }
            
        }

        // render stats
        let scale = self.x_scale / 70.0;
        let spacing_scale = scale * 1.25;
        
        let mut draw_cords = [
            self.ndc_cords[0] + self.stats_ndc_start_cords[0] * self.x_scale,
            self.ndc_cords[1] + self.stats_ndc_start_cords[1] * self.y_scale
            ];

        // Render id
        let drone_id_stat = format!("ID: {}", drone.get_id());
        render_string_at_ndi_cords(texture_manager, drone_id_stat, "Basic".to_string(), scale, draw_cords);
        
        // Render busy
        draw_cords[1] += spacing_scale;
        let drone_busy_stat = format!("Busy: {}", drone.get_busy());
        render_string_at_ndi_cords(texture_manager, drone_busy_stat, "Basic".to_string(), scale, draw_cords);

        // Render fuel
        draw_cords[1] += spacing_scale;
        let drone_busy_stat = format!("Fuel: {}", drone.get_fuel());
        render_string_at_ndi_cords(texture_manager, drone_busy_stat, "Basic".to_string(), scale, draw_cords);

        // Mine power
        draw_cords[1] += spacing_scale;
        let drone_mine_power = format!("Mine Power: {}", drone.get_mine_power());
        render_string_at_ndi_cords(texture_manager, drone_mine_power, "Basic".to_string(), scale, draw_cords);

        // Chop power
        draw_cords[1] += spacing_scale;
        let drone_chop_power = format!("Chop Power: {}", drone.get_chop_power());
        render_string_at_ndi_cords(texture_manager, drone_chop_power, "Basic".to_string(), scale, draw_cords);



    }

    //=====================================
    // Controls
    //=====================================

    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        if self.bar_grabbed {
            let mouse_ndc_cords = screen_data.get_mouse_ndc_cords();
            self.ndc_cords = [
                mouse_ndc_cords[0] + self.window_ndc_bar_grabbed_cords[0],
                mouse_ndc_cords[1] + self.window_ndc_bar_grabbed_cords[1],
            ]
        }
    }

    pub fn mouse_on_x(&self, pixel_mouse_cords_on_window: [f32; 2]) -> bool {
        if pixel_mouse_cords_on_window[0] > 254.0 && pixel_mouse_cords_on_window[1] < 8.0 {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn mouse_on_move_bar(&self, pixel_mouse_cords_on_window: [f32; 2]) -> bool {
        if pixel_mouse_cords_on_window[1] < 8.0 {
            if pixel_mouse_cords_on_window[0] > 193.0 && pixel_mouse_cords_on_window[0] < 254.0 {
                return true;
            }
        }

        return false;
    }
    
    pub fn mouse_on_left_arrow(&self, pixel_mouse_cords_on_window: [f32; 2]) -> bool {
        let in_x_range = pixel_mouse_cords_on_window[0] < 23.0;
        let in_y_range = pixel_mouse_cords_on_window[1] > 174.0;
        
        if in_x_range && in_y_range {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn mouse_on_right_arrow(&self, pixel_mouse_cords_on_window: [f32; 2]) -> bool {
        let in_x_range = pixel_mouse_cords_on_window[0] > 170.0 && pixel_mouse_cords_on_window[0] < 192.0;
        let in_y_range = pixel_mouse_cords_on_window[1] > 174.0;
        
        if in_x_range && in_y_range {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn mouse_button_down_event(&mut self, button: MouseButton, screen_data: &ScreenData) {
        // Gotta calculate if mouse is on window based off scale using src rect y to x racio
        if button == MouseButton::Left {
            let mouse_ndc = screen_data.get_mouse_ndc_cords();

            let x_in_window = mouse_ndc[0] > self.ndc_cords[0] && mouse_ndc[0] < self.end_ndc_cords[0];
            let y_in_window = mouse_ndc[1] > self.ndc_cords[1] && mouse_ndc[1] < self.end_ndc_cords[1];

            if x_in_window && y_in_window {

                // Calculate pixel cords for clicking buttons
                let pixel_mouse_cords_on_window = self.get_mouse_pixel_cords_on_window(mouse_ndc);

                if self.mouse_on_move_bar(pixel_mouse_cords_on_window) {
                    // Set the cords of where on the window it was grabbed
                    self.window_ndc_bar_grabbed_cords = [
                        self.ndc_cords[0] - mouse_ndc[0],
                        self.ndc_cords[1] - mouse_ndc[1],
                    ];

                    self.bar_grabbed = true;
                }
                else if self.mouse_on_x(pixel_mouse_cords_on_window) {
                    self.visible = false;
                }
                else if self.mouse_on_right_arrow(pixel_mouse_cords_on_window) {
                    if self.spectate_zoom > 1 {
                        self.spectate_zoom -= 1;
                    }
                }
                else if self.mouse_on_left_arrow(pixel_mouse_cords_on_window) {
                    if self.spectate_zoom <= 3{
                        self.spectate_zoom += 1;
                    }
                }
            }
        }
    }

    pub fn handle_mouse_button_up(&mut self, mouse_button_up: MouseButton, screen_data: &ScreenData) {
        if mouse_button_up == MouseButton::Left {
            let mouse_ndc = screen_data.get_mouse_ndc_cords();

            let x_in_window = mouse_ndc[0] > self.ndc_cords[0] && mouse_ndc[0] < self.end_ndc_cords[0];
            let y_in_window = mouse_ndc[1] > self.ndc_cords[1] && mouse_ndc[1] < self.end_ndc_cords[1];

            if x_in_window && y_in_window {
                let pixel_mouse_cords_on_window = self.get_mouse_pixel_cords_on_window(mouse_ndc);
                if self.mouse_on_move_bar(pixel_mouse_cords_on_window) {
                    self.bar_grabbed = false;
                }
            }
        }
    }


}