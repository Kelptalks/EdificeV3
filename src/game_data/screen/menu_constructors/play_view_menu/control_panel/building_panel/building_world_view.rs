use std::{cell::RefCell, rc::Rc};

use miniquad::KeyCode;

use crate::game_data::{game_event_manager::{game_event_manager::Event, input_event_manager::input_event_manager::InputEvent, player_data_event_manager::{location_event::LocationEvent, player_event_manager::PlayerDataEvent}, widget_event_manager::widget_event_manager::WidgetEvent, world_event_manager::world_event_manager::WorldEvent}, locations::world_area::WorldArea, player_data::{locations::location::WorldLocation, player_data::PlayerData}, screen::widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget_calculations::TextSize, world_rendering::play_world_view_config::PlayViewRendingConfig}};

fn construct_building_events(location_ref: &Rc<RefCell<WorldLocation>>) -> Vec<InputEvent> {
    let mut building_input_events = Vec::new();


    let fill_location_event = Event::WorldEvent(WorldEvent::FillLocation(location_ref.clone(), crate::game_data::types::BlockTexture::Stone));
    let place_block_event = 
        InputEvent::RightMouseButtonClicked(
            fill_location_event
        ); 
    
    building_input_events.push(place_block_event);

    return building_input_events;
}

fn construct_zoom_events(zoom_ref: &Rc<RefCell<i32>>) -> Vec<InputEvent> {
    let mut zoom_input_events = Vec::new();

    let zoom_out_event = Event::WidgetEvent(WidgetEvent::Modi32Event(zoom_ref.clone(), -1));
    let scroll_up_input_event = 
        InputEvent::ScrollUp(
            zoom_out_event
        ); 
    zoom_input_events.push(scroll_up_input_event);

    let zoom_in_event = Event::WidgetEvent(WidgetEvent::Modi32Event(zoom_ref.clone(), 1));
    let scroll_down_input_event = 
        InputEvent::ScrollDown(
            zoom_in_event
        ); 
    zoom_input_events.push(scroll_down_input_event);

    return zoom_input_events;
}

fn construct_shift_input_event(location_ref: &Rc<RefCell<WorldLocation>>, keycode: KeyCode, shift: [i32; 3]) -> InputEvent {
    // Move camera down event
    let shift_event = 
        PlayerDataEvent::LocationEvent(
            location_ref.clone(), 
            LocationEvent::ShiftLocation(shift)
        );

    let key_down_input_event = 
        InputEvent::KeyDown(
            keycode, 
            Event::PlayerDataEvent(shift_event)
        ); 

    return key_down_input_event;
}

fn get_input_events(rendering_config: &PlayViewRendingConfig) -> Vec<InputEvent>{
    let mut input_events = Vec::new();

    let location_ref = rendering_config.get_location_ref().clone();

    // Verticle Key Movement
    input_events.push(self::construct_shift_input_event(&location_ref, KeyCode::LeftShift, [0, 0, -1]));
    input_events.push(self::construct_shift_input_event(&location_ref, KeyCode::Space, [0, 0, 1]));
    
    // Horizontal Key Movment
    input_events.push(self::construct_shift_input_event(&location_ref, KeyCode::W, [0, -1, 0]));
    input_events.push(self::construct_shift_input_event(&location_ref, KeyCode::S, [0, 1, 0]));

    input_events.push(self::construct_shift_input_event(&location_ref, KeyCode::A, [-1, 0, 0]));
    input_events.push(self::construct_shift_input_event(&location_ref, KeyCode::D, [1, 0, 0]));

    // Zooming events
    let zoom_ref = rendering_config.get_zoom_ref();
    input_events.append(&mut construct_zoom_events(zoom_ref));

    input_events.append(&mut construct_building_events(&location_ref));

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


    for event in input_events {
        play_view.add_input_event(event);
    }



    play_view.set_prefered_size(0.8);
}