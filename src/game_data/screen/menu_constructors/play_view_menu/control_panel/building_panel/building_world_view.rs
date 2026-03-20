use std::{cell::RefCell, rc::Rc};

use miniquad::KeyCode;

use crate::game_data::{game_event_manager::input_event_manager::input_event_manager, locations::world_area::WorldArea, player_data::player_data::PlayerData, screen::{menu_constructors::play_view_menu::control_panel::view_panel_input_constructor, widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget_calculations::TextSize, world_rendering::{play_world_view_config::PlayViewRendingConfig, play_world_view_render::PlayWorldViewRender}}}, types::BlockTexture};

use crate::game_data::game_event_manager::prelude::*;

fn get_building_events(play_view: &PlayWorldViewRender) -> Vec<Event>{
    let mut input_events = Vec::new();

    let location_ref = play_view.get_rendering_config().get_location_ref();

    let event_to_enact = WorldEvent::FillLocation(location_ref.clone(), BlockTexture::Air).wrap_into_event_vec();
    
    let input_event = InputEvent::LeftMouseButtonDown(event_to_enact).wrap_into_event();

    input_events.push(input_event);

    return input_events;
}

fn get_input_events(play_view: &PlayWorldViewRender) -> Vec<Event>{
    let mut input_events = Vec::new();

    // Basic controls
    input_events.append(&mut view_panel_input_constructor::construct_area_selection_events(play_view));
    input_events.append(&mut view_panel_input_constructor::construct_camera_keyboard_movements(play_view));
    input_events.append(&mut view_panel_input_constructor::construct_zoom_events(play_view));

    // Building specific
    input_events.append(&mut self::get_building_events(play_view));

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