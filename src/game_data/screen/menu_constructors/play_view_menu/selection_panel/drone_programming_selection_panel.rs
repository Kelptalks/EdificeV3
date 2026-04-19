use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{drone_script::{action::{self, action::Action, action_type::ActionType}, control_flow::control_flow::ControlFlow, function::function::Function, var::{programming_vars::programming_var::ProgrammingVar, var::Var}}, drones::drone_actions::drone_actions::DroneAction}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, TabPanel, VarSlot}, widget::WidgetType, widget_calculations::TextSize}, types::UITextures};



fn get_action_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        // Header
        let text_display = panel.add_text_display("Drone Actions".to_string());
        text_display.set_text_scale(TextSize::Medium);

        let scroll_panel = panel.add_scroll_panel();
        

        let vars_per_row = 5;
        let mut current_collumn = 0;
        
        let mut row_sub_panel  = Panel::new_blank();

        let all_actions = DroneAction::get_all_actions();
        for drone_action in all_actions {
            if current_collumn == vars_per_row {
                scroll_panel.add_widget(row_sub_panel.wrap_into_widget());
                row_sub_panel = Panel::new_blank();
                current_collumn = 0;
            }
            current_collumn += 1;
            

            
            let action = Action::new(ActionType::DroneAction(drone_action));
            let var_type = action.wrap_into_script_element().wrap_into_var_type();
            let var = Var::new_with_var_type(var_type);

            let mut var_slot = VarSlot::new_with_var(var);
            var_slot.set_dragging_properties(true, false, false);
            row_sub_panel.add_widget(var_slot.wrap_into_widget());
        }

        scroll_panel.add_widget(row_sub_panel.wrap_into_widget());

        scroll_panel.set_prefered_scale(0.5);
        panel.size();
    }

    return panel;
}

fn get_control_flow_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        // Header
        let text_display = panel.add_text_display("Control Flow".to_string());
        text_display.set_text_scale(TextSize::Medium);

        let scroll_panel = panel.add_scroll_panel();
        


        let control_flow = ControlFlow::new_blank();
        let var_type = control_flow.wrap_into_script_element().wrap_into_var_type();
        let var = Var::new_with_var_type(var_type);

        let mut var_slot = VarSlot::new_with_var(var);
        var_slot.set_dragging_properties(true, false, false);
        

        scroll_panel.add_widget(var_slot.wrap_into_widget());

        scroll_panel.set_prefered_scale(0.5);
        panel.size();
    }


    panel
}

pub fn get_drone_programming_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        let current_panel_index = Rc::new(RefCell::new(0));
        let mut selection_tab_panel = TabPanel::new(&current_panel_index);

        // Add Action selection
        let button = selection_tab_panel.add_panel(get_action_selection_panel());
        button.set_text("Actions".to_string());
        button.set_icon(UITextures::ScallingIconMidCenter);

        // Add Contol Flow selection
        let button = selection_tab_panel.add_panel(get_control_flow_selection_panel());
        button.set_text("Actions".to_string());
        button.set_icon(UITextures::ScallingIconMidCenter);


        panel.add_widget(selection_tab_panel.wrap_into_widget());

        panel.size();
    }

    panel
}