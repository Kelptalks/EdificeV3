use crate::game_data::{drone_programming::var::{game_vars::game_var_type::GameVarTypeKind, var_type::VarTypeKind}, player_data::player_data::PlayerData, screen::{menu_constructors::play_view_menu::selection_panel::building_panel::building_world_view, widget::{panel::panel::{PanelAlignment, PanelOrientation}, widget::WidgetType}}};

pub fn get_building_panel(player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);

        building_world_view::add_bulding_world_view_panel(panel, player_data);


        let block_selection_panel = panel.add_sub_panel();
        block_selection_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        for i in 0..9 {
            block_selection_panel.add_var_slot(VarTypeKind::Game(GameVarTypeKind::Block));
        }


        panel.size();
    }

    return panel;
}