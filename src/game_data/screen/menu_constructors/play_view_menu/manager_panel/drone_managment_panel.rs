use std::{cell::{Ref, RefCell}, rc::Rc};

use crate::game_data::{game_event_manager::player_data_event_manager::var_event_manager::var_events::{DroneVarEvent, VarEvents}, player_data::{drone_script::var::{game_vars::{dynamic_var::DynamicVarType, game_var_type::GameVarType}, var_type::VarType}, drones::drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, prim_actions::{drone_invintory_actions::DroneInventoryAction, drone_world_actions::DroneWorldAction}}}, screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{drone_programming::function_slot::FunctionSlot, panel::panel::{PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot}, widget::WidgetType, widget_calculations::TextSize}}, types::drone_item::DroneItem};

pub fn get_drone_managment_panel(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        let panel_title = panel.add_text_display("Drone Panel".to_string());
        panel_title.set_text_scale(TextSize::Large);
        


        let execute_button = panel.add_button();


        panel.size();
    }


    return panel;
}

