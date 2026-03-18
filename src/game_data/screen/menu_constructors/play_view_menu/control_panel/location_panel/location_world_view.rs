use crate::game_data::{player_data::player_data::PlayerData, screen::widget::panel::panel::{Panel, PanelAlignment, PanelOrientation}};

pub fn add_location_world_view_panel(panel: &mut Panel, player_data: &mut PlayerData) {
    let play_view_sub_panel = panel.add_sub_panel();
    play_view_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

}