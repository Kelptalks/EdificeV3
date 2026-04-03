use std::{cell::{Ref, RefCell}, rc::Rc};

use crate::game_data::{game_event_manager::player_data_event_manager::var_event_manager::var_events::{DroneVarEvent, VarEvents}, player_data::drone_programming::var::{game_vars::{dynamic_var::DynamicVar, game_var_type::GameVar}, var_type::Var}, screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{panel::panel::{PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot}, widget::WidgetType, widget_calculations::TextSize}}};

pub fn get_drone_managment_panel(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        let panel_title = panel.add_text_display("Drone Panel".to_string());
        panel_title.set_text_scale(TextSize::Large);
        

        let drone_pathing_location_var = Rc::new(RefCell::new(Var::Game(GameVar::Dynamic(DynamicVar::Location(None)))));
        let drone_pathing_locatoin_var_slot = VarSlot::new(&drone_pathing_location_var);

        panel.add_widget(drone_pathing_locatoin_var_slot.wrap_into_widget());
        
        
        let path_to_location_button = panel.add_button();
        path_to_location_button.add_left_click_event(VarEvents::DroneVarEvent(ref_manager.selected_var.clone(), DroneVarEvent::PathToLocationVar(drone_pathing_location_var)).wrap_into_event());

        panel.size();
    }


    return panel;
}

