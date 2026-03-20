use std::{cell::RefCell, rc::Rc};

use miniquad::KeyCode;

use crate::game_data::{game_event_manager::input_event_manager::input_event_manager, locations::world_area::WorldArea, player_data::{locations::location::WorldLocation, player_data::PlayerData}, screen::widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget_calculations::TextSize, world_rendering::{play_world_view_config::PlayViewRendingConfig, play_world_view_render::PlayWorldViewRender}}};

use crate::game_data::game_event_manager::prelude::*;


fn construct_building_events(shift_type_ref: &Rc<RefCell<usize>>, location_ref: &Rc<RefCell<WorldLocation>>) -> Vec<Event> {
    let mut building_input_events = Vec::new();


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

//=====================================
// Zooming event
//=====================================

fn construct_zoom_events(zoom_ref: &Rc<RefCell<i32>>) -> Vec<Event> {
    let mut zoom_input_events = Vec::new();

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

//=====================================
// All inputs
//=====================================

fn get_input_events(play_view: &PlayWorldViewRender) -> Vec<Event>{
    let mut input_events = Vec::new();

    let location_ref = play_view.get_rendering_config().get_location_ref().clone();
    let shift_type_ref = play_view.get_rendering_config().get_camera_movment_event_type_ref();

    let camera_control_manager = play_view.get_camera_control_manager();

    // Verticle Key Movement
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

    // Zooming events
    let zoom_ref = play_view.get_rendering_config().get_zoom_ref();
    input_events.append(&mut construct_zoom_events(zoom_ref));

    input_events.append(&mut construct_building_events(&shift_type_ref, &location_ref));

    return input_events;
}


pub fn add_bulding_world_view_panel(panel: &mut Panel, player_data: &mut PlayerData) {
    let play_view_panel = panel.add_sub_panel();
    play_view_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
    play_view_panel.set_color(PanelColor::Dark);

    // Header
    let text_display = play_view_panel.add_text_display("Building View".to_string());
    text_display.set_text_scale(TextSize::Medium);

    // Create the rendering config
    let location_manager = player_data.get_mut_location_manager();
    let location = location_manager.create_location(
        "Building_Location".to_string(), 
        WorldArea::new_blank()
    );    


    let rendering_config = PlayViewRendingConfig::new(player_data.get_world_ref(), location);


    // Add World Rendering
    let play_view = play_view_panel.add_play_world_view_renderer(rendering_config);
    let input_events = get_input_events(&play_view);

    // Wrapp and add input events
    for event in input_events {
        play_view.add_event(event);
    }



    play_view.set_prefered_size(0.8);
}