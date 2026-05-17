#![allow(dead_code)]

use crate::game_data::{player_data::drone_script::var::{self}, screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{drone_programming::var_slot::var_slot::VarSlot, panel::{panel::{PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget::WidgetType}}};





//=====================================
// Var Ref HotBar
//=====================================
// A set of var slots that can hold any value type simply for the pourpose of
// setting other var slots

pub fn get_widget(ref_manager: &RefManager) -> WidgetType {

    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
        panel.set_color(PanelColor::Clear);

        let source_var_panel = panel.add_sub_panel();
        source_var_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        source_var_panel.set_color(PanelColor::DarkUI);

        let selected_var_panel = panel.add_sub_panel();
        let selected_var_slot = ref_manager.selected_var.clone();
        let mut var_slot = VarSlot::new_with_var(selected_var_slot);
        var_slot.set_dragging_properties(true, false, false);
        selected_var_panel.add_widget(var_slot.wrap_into_widget());


        let slot_sub_panel = panel.add_sub_panel();
        slot_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        slot_sub_panel.set_color(PanelColor::DarkUI);

        for _i in 0..11 {
            
            // Init Widget
            let mut var_slot = VarSlot::new_with_kind(var::var_type::VarKind::Any);
            var_slot.set_dragging_properties(true, true, true);

            // Add to panel
            slot_sub_panel.add_widget(var_slot.wrap_into_widget());
        }



        
        panel.size();
    }
    return panel;
}