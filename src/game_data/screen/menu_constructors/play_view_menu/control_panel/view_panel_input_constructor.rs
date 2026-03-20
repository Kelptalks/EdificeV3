use std::cell::RefCell;
use std::rc::Rc;

use miniquad::KeyCode;

use crate::game_data::game_event_manager::input_event_manager::input_event_manager;
use crate::game_data::{screen::widget::world_rendering::play_world_view_render::PlayWorldViewRender};
use crate::game_data::game_event_manager::prelude::*;

pub fn construct_area_selection_events(play_view: &PlayWorldViewRender) -> Vec<Event> {
    let mut building_input_events = Vec::new();
    let shift_type_ref = play_view.get_rendering_config().get_camera_movement_event_type_ref();


    let left_mouse_down_input_event = 
        InputEvent::RightMouseButtonDown(
            WidgetEvent::SetUsizeEvent(shift_type_ref.clone(), 1).wrap_into_event_vec()
        ).wrap_into_event();
    building_input_events.push(left_mouse_down_input_event);

    let left_mouse_up_input_event = 
        InputEvent::RightMouseButtonUp(
            WidgetEvent::SetUsizeEvent(shift_type_ref.clone(), 0).wrap_into_event_vec()
        ).wrap_into_event();
    building_input_events.push(left_mouse_up_input_event);

    return building_input_events;
}

pub fn construct_zoom_events(play_view: &PlayWorldViewRender) -> Vec<Event> {
    let mut zoom_input_events = Vec::new();
    let zoom_ref = play_view.get_rendering_config().get_zoom_ref();

    let zoom_out_event = WidgetEvent::Modi32Event(zoom_ref.clone(), -1).wrap_into_event_vec();
    let scroll_up_input_event = 
        InputEvent::ScrollUp(
            zoom_out_event
        ).wrap_into_event(); 
    zoom_input_events.push(scroll_up_input_event);

    let zoom_in_event = WidgetEvent::Modi32Event(zoom_ref.clone(), 1).wrap_into_event_vec();
    let scroll_down_input_event = 
        InputEvent::ScrollDown(
            zoom_in_event
        ).wrap_into_event(); 
    zoom_input_events.push(scroll_down_input_event);

    return zoom_input_events;
}

pub fn construct_camera_keyboard_movements(play_view: &PlayWorldViewRender) -> Vec<Event>{
    let mut input_events = Vec::new();
    let camera_control_manager = play_view.get_camera_control_manager();

    let movment_event = camera_control_manager.get_camera_shift_event([0, 0, -1]);
    input_events.push(input_event_manager::construct_key_down_event(KeyCode::LeftShift, movment_event));

    let movment_event = camera_control_manager.get_camera_shift_event([0, 0, 1]);
    input_events.push(input_event_manager::construct_key_down_event(KeyCode::Space, movment_event));

    // Horizontal Key Movment
    let movment_event = camera_control_manager.get_camera_shift_event([0, -1, 0]);
    input_events.push(input_event_manager::construct_key_down_event(KeyCode::W, movment_event));

    let movment_event = camera_control_manager.get_camera_shift_event([0, 1, 0]);
    input_events.push(input_event_manager::construct_key_down_event(KeyCode::S, movment_event));

    let movment_event = camera_control_manager.get_camera_shift_event([-1, 0, 0]);
    input_events.push(input_event_manager::construct_key_down_event(KeyCode::A, movment_event));

    let movment_event = camera_control_manager.get_camera_shift_event([1, 0, 0]);
    input_events.push(input_event_manager::construct_key_down_event(KeyCode::D, movment_event));
    return input_events;

}