use crate::game_data::screen::{menu_constructors::play_view_menu::{new_play_view::RefManager, world_hot_bar}, widget::{panel::panel::{PanelAlignment, PanelOrientation}, prelude::{PanelColor, PlayWorldViewRender}, widget::WidgetType}};

use miniquad::KeyCode;

use crate::game_data::game_event_manager::input_event_manager::input_event_manager;
use crate::game_data::game_event_manager::prelude::*;

//=====================================
// UI Constructors
//=====================================

pub fn get_widget(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Clear);

        let mut inputs = Vec::new();
        let play_view = PlayWorldViewRender::new(ref_manager.play_view_rendering_config.clone());
        
        // Add Controls
        inputs.append(&mut construct_zoom_events(ref_manager));
        inputs.append(&mut construct_area_selection_events(ref_manager));
        inputs.append(&mut construct_camera_keyboard_movements(&play_view));
        panel.add_events(&mut inputs);

        
        panel.add_widget(play_view.wrap_into_widget());

        panel.add_widget(world_hot_bar::get_widget(ref_manager));        
    }

    return panel;
    
}

//=====================================
// Control Constructors
//=====================================

fn construct_area_selection_events(ref_manager: &mut RefManager) -> Vec<Event> {
    let mut building_input_events = Vec::new();
    let config = ref_manager.play_view_rendering_config.borrow();

    let shift_type_ref = config.get_camera_movement_event_type_ref();

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

fn construct_zoom_events(ref_manager: &mut RefManager) -> Vec<Event> {
    let mut zoom_input_events = Vec::new();
    let config = ref_manager.play_view_rendering_config.borrow();

    let zoom_ref = config.get_zoom_ref();

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

fn construct_camera_keyboard_movements(play_view: &PlayWorldViewRender) -> Vec<Event>{
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