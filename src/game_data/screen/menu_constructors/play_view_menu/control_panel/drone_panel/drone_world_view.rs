use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::Event, player_data::{locations::location::WorldLocation, player_data::PlayerData}, screen::{menu_constructors::play_view_menu::control_panel::view_panel_input_constructor, widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, text::text_input::TextInput, world_rendering::play_world_view_config::PlayViewRendingConfig}}};

pub fn add_drone_world_view_panel(panel: &mut Panel, player_data: &mut PlayerData, rendering_location: &Rc<RefCell<WorldLocation>>) {
    let play_view_sub_panel = panel.add_sub_panel();
    play_view_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
    play_view_sub_panel.set_color(crate::game_data::screen::widget::panel::panel_color::PanelColor::Dark);

        

    let rendering_config = PlayViewRendingConfig::new(player_data.get_world_ref(), rendering_location.clone());
    let play_view = play_view_sub_panel.add_play_world_view_renderer(rendering_config);



    let mut events:Vec<Event> = Vec::new();
    events.append(&mut view_panel_input_constructor::construct_zoom_events(play_view));
    play_view.add_events(&mut events);



    play_view.set_prefered_size(0.8);
}