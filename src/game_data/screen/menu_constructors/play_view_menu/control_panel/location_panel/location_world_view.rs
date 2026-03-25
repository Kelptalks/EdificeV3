use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::Event, locations::world_area::WorldArea, player_data::{locations::{location::WorldLocation, location_config::WorldLocationConfig}, player_data::PlayerData}, screen::{menu_constructors::play_view_menu::control_panel::view_panel_input_constructor, widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, text::text_input::TextInput, widget_calculations::TextSize, world_rendering::play_world_view_config::PlayViewRendingConfig}}, types::UITextures};

pub fn add_location_world_view_panel(panel: &mut Panel, player_data: &mut PlayerData, location_config: Rc<RefCell<WorldLocationConfig>>) {
    let play_view_sub_panel = panel.add_sub_panel();
    play_view_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
    play_view_sub_panel.set_color(crate::game_data::screen::widget::panel::panel_color::PanelColor::Dark);

    // Header
    let location_name_text_input = TextInput::new_text_input(&location_config.borrow().get_location_name_ref());
    play_view_sub_panel.add_widget(location_name_text_input.wrap_into_widget());
        

    let rendering_config = PlayViewRendingConfig::new(player_data.get_world_ref(), location_config.borrow().get_source_location_ref());
    let play_view = play_view_sub_panel.add_play_world_view_renderer(rendering_config);


    let mut events:Vec<Event> = Vec::new();
    events.append(&mut view_panel_input_constructor::construct_area_selection_events(play_view));
    events.append(&mut view_panel_input_constructor::construct_camera_keyboard_movements(play_view));
    events.append(&mut view_panel_input_constructor::construct_zoom_events(play_view));
    play_view.add_events(&mut events);



    play_view.set_prefered_size(0.8);
}