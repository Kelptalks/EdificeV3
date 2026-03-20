use std::{cell::RefCell, rc::Rc};

use miniquad::KeyCode;

use crate::game_data::{locations::world_area::WorldArea, player_data::{locations::location::WorldLocation, player_data::PlayerData}, screen::widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget_calculations::TextSize, world_rendering::play_world_view_config::PlayViewRendingConfig}};

use crate::game_data::game_event_manager::prelude::*;


fn construct_building_events(shift_type_ref: &Rc<RefCell<usize>>, location_ref: &Rc<RefCell<WorldLocation>>) -> Vec<InputEvent> {
    let mut building_input_events = Vec::new();


    let left_mouse_down_input_event = 
        InputEvent::LeftMouseButtonDown(WidgetEvent::SetUsizeEvent(shift_type_ref.clone(), 1).wrap_into_event());
    building_input_events.push(left_mouse_down_input_event);

    let left_mouse_up_input_event = 
        InputEvent::LeftMouseButtonReleased(WidgetEvent::SetUsizeEvent(shift_type_ref.clone(), 0).wrap_into_event());
    building_input_events.push(left_mouse_up_input_event);

    return building_input_events;
}

//=====================================
// Zooming event
//=====================================

fn construct_zoom_events(zoom_ref: &Rc<RefCell<i32>>) -> Vec<InputEvent> {
    let mut zoom_input_events = Vec::new();

    let zoom_out_event = WidgetEvent::Modi32Event(zoom_ref.clone(), -1).wrap_into_event();
    let scroll_up_input_event = 
        InputEvent::ScrollUp(
            zoom_out_event
        ); 
    zoom_input_events.push(scroll_up_input_event);

    let zoom_in_event = WidgetEvent::Modi32Event(zoom_ref.clone(), 1).wrap_into_event();
    let scroll_down_input_event = 
        InputEvent::ScrollDown(
            zoom_in_event
        ); 
    zoom_input_events.push(scroll_down_input_event);

    return zoom_input_events;
}

//=====================================
// Location Shifting Input Event
//=====================================

fn construct_shift_input_event(
    shift_type_ref: &Rc<RefCell<usize>>, 
    location_ref: &Rc<RefCell<WorldLocation>>, 
    keycode: KeyCode, 
    shift: [i32; 3]
) -> InputEvent {

    // Create shift events 
    let default_shift_event = 
        PlayerDataEvent::LocationEvent(
            location_ref.clone(), 
            LocationEvent::ShiftLocation(shift)
        ).wrap_into_event();


    let mouse_held_shift_event = 
        PlayerDataEvent::LocationEvent(
            location_ref.clone(),
            LocationEvent::ShiftPoint(0, shift)
        ).wrap_into_event();

    
    // Wrap with event type
    let event_shift_list = vec![
        default_shift_event, 
        mouse_held_shift_event
        ];

    let dispatch_control_flow_event = 
        DispatchEvent::IndexedEvent(
            shift_type_ref.clone(), 
            event_shift_list
        ).wrap_dispatch_event();

    let key_down_input_event = 
        InputEvent::KeyDown(
            keycode, 
            vec![dispatch_control_flow_event]
        ); 

    return key_down_input_event;
}

fn get_input_events(rendering_config: &PlayViewRendingConfig) -> Vec<InputEvent>{
    let mut input_events = Vec::new();

    let location_ref = rendering_config.get_location_ref().clone();

    let shift_type_ref = Rc::new(RefCell::new(0));

    // Verticle Key Movement
    input_events.push(self::construct_shift_input_event(&shift_type_ref, &location_ref, KeyCode::LeftShift, [0, 0, -1]));
    input_events.push(self::construct_shift_input_event(&shift_type_ref, &location_ref, KeyCode::Space, [0, 0, 1]));
    
    // Horizontal Key Movment
    input_events.push(self::construct_shift_input_event(&shift_type_ref, &location_ref, KeyCode::W, [0, -1, 0]));
    input_events.push(self::construct_shift_input_event(&shift_type_ref, &location_ref, KeyCode::S, [0, 1, 0]));

    input_events.push(self::construct_shift_input_event(&shift_type_ref, &location_ref, KeyCode::A, [-1, 0, 0]));
    input_events.push(self::construct_shift_input_event(&shift_type_ref, &location_ref, KeyCode::D, [1, 0, 0]));

    // Zooming events
    let zoom_ref = rendering_config.get_zoom_ref();
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
    let input_events = get_input_events(&rendering_config);


    // Add World Rendering
    let play_view = play_view_panel.add_play_world_view_renderer(rendering_config);

    // Wrapp and add input events
    for event in input_events {
        let wrapped_input_event = Event::InputEvent(event);
        play_view.add_event(wrapped_input_event);
    }



    play_view.set_prefered_size(0.8);
}