use miniquad::{KeyCode, KeyMods, MouseButton};

use crate::game_data::{game_event_manager::{game_event_manager::EventManager, render_event_manager::render_event_manager::RenderEvent}, screen::{iso_cord_tool, screen_data::CurrentMenu, screen_mananager::ScreenManager}};
/*
####################
## CameraControls ##
####################




*/


//=====================================
// Key input
//=====================================

pub fn mouse_motion_event(screen_manager: &mut ScreenManager, x_cor: f32, y_cor: f32) {
    // Save needed pre movement data
    let starting_mouse_ndc_cords = screen_manager.get_screen_data().get_renderer_mouse_ndc_cords();
    let mut camera_data_clone = screen_manager.get_camera_data().clone();

    // Update mouse cords for new position
    screen_manager.update_mouse_cords(x_cor, y_cor);
    screen_manager.get_mut_screen_data().re_calculate_mouse_cords(&camera_data_clone);
    
    // Save ending mouse data
    let ending_mouse_ndc_cords = screen_manager.get_screen_data().get_renderer_mouse_ndc_cords();

    // Handle camera movment due to mouse
    let is_middle_mouse_held = screen_manager.get_screen_data().is_middle_mouse_held();
    
    if is_middle_mouse_held {
        let x_ndc_change = ending_mouse_ndc_cords[0] - starting_mouse_ndc_cords[0];
        let y_ndc_change = ending_mouse_ndc_cords[1] - starting_mouse_ndc_cords[1];

        let camera_data = screen_manager.get_mut_camera_data();

        camera_data.mod_x_cam_cor(x_ndc_change);
        camera_data.mod_y_cam_cor(y_ndc_change);

        // Also modify the clone to update mouse_cords
        camera_data_clone.mod_x_cam_cor(x_ndc_change);
        camera_data_clone.mod_y_cam_cor(y_ndc_change);

        // Update mouse cords again after movement
        screen_manager.update_mouse_cords(x_cor, y_cor);
        screen_manager.get_mut_screen_data().re_calculate_mouse_cords(&camera_data_clone);
    }
}

pub fn mouse_button_down_event(screen_manager: &mut ScreenManager, button: MouseButton) {
    let screen_data = screen_manager.get_mut_screen_data();
    
    if button == MouseButton::Middle {
        screen_data.set_middle_mouse_held(true);
    }


}

pub fn mouse_button_up_event(screen_manager: &mut ScreenManager, button: MouseButton) {
    let screen_data = screen_manager.get_mut_screen_data();
    if button == MouseButton::Middle {
        screen_data.set_middle_mouse_held(false);
    }
}


pub fn key_down_event(event_manager: &mut EventManager, screen_manager: &mut ScreenManager, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
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

            KeyCode::M => {
                event_manager.add_render_event(RenderEvent::ChangeMenu(CurrentMenu::PlayView));
            }
            _ => {}
    }

}

//=====================================
// Mouse inputs
//=====================================


pub fn mouse_wheel_event(screen_mananager: &mut ScreenManager, x_scroll_distance: f32, y_scroll_distance: f32) {
    let zoom_speed = 0.06;
    let starting_iso_cam_center = screen_mananager.get_mut_camera_data().get_iso_cam_center();


    // Use cloned camera data to prevent race conditions due to camera rendering updates
    let mut cloned_camera_data = screen_mananager.get_camera_data().clone();
    if y_scroll_distance > 0.0 {
        cloned_camera_data.mod_scale(1.0 + zoom_speed);
    }
    else if y_scroll_distance < 0.0 {
        cloned_camera_data.mod_scale(1.0 - zoom_speed);
    }

    // Update camera values after zoom to get new iso center
    cloned_camera_data.update_camera_values();

    let new_iso_cam_center = cloned_camera_data.get_iso_cam_center();
    let x_iso_change = new_iso_cam_center[0] - starting_iso_cam_center[0];
    let y_iso_change = new_iso_cam_center[1] - starting_iso_cam_center[1];

    let screen_cord_shift = iso_cord_tool::float_iso_to_ndc_cords(cloned_camera_data.get_tile_ndc_scale(), [x_iso_change, y_iso_change]);
    
    // Update the real camera data
    if y_scroll_distance > 0.0 {
        screen_mananager.get_mut_camera_data().mod_scale(1.0 + zoom_speed);
    }
    else if y_scroll_distance < 0.0 {
        screen_mananager.get_mut_camera_data().mod_scale(1.0 - zoom_speed);
    }
    screen_mananager.get_mut_camera_data().mod_x_cam_cor(screen_cord_shift[0]);
    screen_mananager.get_mut_camera_data().mod_y_cam_cor(screen_cord_shift[1]);


}
