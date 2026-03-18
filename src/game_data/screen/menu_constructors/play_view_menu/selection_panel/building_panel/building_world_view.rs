use crate::game_data::{game_event_manager::{game_event_manager::Event, input_event_manager::input_event_manager::InputEvent, player_data_event_manager::{location_event::LocationEvent, player_event_manager::PlayerDataEvent}}, locations::world_area::WorldArea, player_data::player_data::PlayerData, screen::widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget_calculations::TextSize, world_rendering::play_world_view_config::PlayViewRendingConfig}};



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
    let location_ref = rendering_config.get_location_ref().clone();

    // Add World Rendering
    let play_view = play_view_panel.add_play_world_view_renderer(rendering_config);

    let cords_ref = player_data.get_mut_location_manager().get_player_cursor_location_cords_ref();
    play_view.link_camera_world_cords_ref(cords_ref);


    let shift_event = 
        PlayerDataEvent::LocationEvent(
            location_ref, 
            LocationEvent::ShiftLocation([0, 0, -1])
        );

    let key_down_input_event = 
        InputEvent::KeyDown(
            miniquad::KeyCode::LeftShift, 
            Event::PlayerDataEvent(shift_event)
        ); 

    play_view.add_input_event(key_down_input_event);



    play_view.set_prefered_size(0.8);
}