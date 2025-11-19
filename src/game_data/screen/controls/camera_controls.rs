use miniquad::{KeyCode, KeyMods};

use crate::game_data::screen::{Camera, CameraData, camera_controls, iso_cord_tool, renderer::camera, screen_mananager::{ControlManager, ScreenManager}};
/*
####################
## CameraControls ##
####################




*/


//=====================================
// Key input
//=====================================

pub fn mouse_motion_event(screen_manager: &mut ScreenManager, mouse_cords: [f32; 2]) {

}

pub fn mouse_button_down_event(screen_manager: &mut ScreenManager) {
    let control_manager = screen_manager.get_control_manager();
    
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let pixel_cords = control_manager.get_mouse_pixel_cords();
    println!("Pixel Cords: ({}, {})", pixel_cords[0], pixel_cords[1]);

    let ndc_cords = control_manager.get_mouse_ndc_cords();
    println!("NDC Cords: ({}, {})", ndc_cords[0], ndc_cords[1]);

    let pixel_world_cords = control_manager.get_renderer_mouse_pixel_cords();
    println!("Renderer Pixel Cords: ({}, {})", pixel_world_cords[0], pixel_world_cords[1]);

    let ndc_world_cords = control_manager.get_renderer_mouse_ndc_cords();
    println!("Renderer NDC Cords: ({}, {})", ndc_world_cords[0], ndc_world_cords[1]);


}

pub fn key_down_event(screen_manager: &mut ScreenManager, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
    let camera_data = screen_manager.get_mut_camera_data();
    match keycode {
        KeyCode::W | KeyCode::Up => {
                camera_data.mod_y_cam_cor(0.01);
            }
            KeyCode::A | KeyCode::Right => {
                camera_data.mod_x_cam_cor(0.01);
            }   
            KeyCode::S | KeyCode::Down => {
                camera_data.mod_y_cam_cor(-0.01);
            }
            KeyCode::D | KeyCode::Left => {
                camera_data.mod_x_cam_cor(-0.01);
            }
            _ => {}
    }
}

pub fn mouse_wheel_event(screen_mananager: &mut ScreenManager, x_scroll_distance: f32, y_scroll_distance: f32) {
    let zoom_speed = 0.06;
    let camera_data = screen_mananager.get_mut_camera_data();
    if (y_scroll_distance > 0.0) {
        camera_data.mod_scale(1.0 + zoom_speed);
    }
    else if (y_scroll_distance < 0.0) {
        camera_data.mod_scale(1.0 - zoom_speed);
    }
}
