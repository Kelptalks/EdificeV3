use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::var::{self, game_vars::{game_var_type::GameVarType, primitive_var::PrimitiveGameVarType}, var_type::VarType}, screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{drone_programming::var_slot::var_slot::VarSlot, panel::{panel::{PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget::WidgetType, widget_calculations::TextSize}}};





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
        source_var_panel.set_color(PanelColor::DarkBlue);
        

        let slot_sub_panel = panel.add_sub_panel();
        slot_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        slot_sub_panel.set_color(PanelColor::DarkBlue);

        for _i in 0..11 {
            
            // Init Widget
            let mut var_slot = VarSlot::new_ref_with_kind(var::var_type::VarKind::Any);
            var_slot.set_dragging_properties(true, true, true);

            // Add to panel
            slot_sub_panel.add_widget(var_slot.wrap_into_widget());
        }



        
        panel.size();
    }
    return panel;
}