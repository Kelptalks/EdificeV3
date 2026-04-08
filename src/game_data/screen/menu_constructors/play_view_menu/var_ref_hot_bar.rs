use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::var::{self, game_vars::{game_var_type::GameVar, primitive_var::PrimitiveVar}, var_type::Var}, screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{drone_programming::vars::var_slot::VarSlot, panel::{panel::{PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget::WidgetType, widget_calculations::TextSize}}};





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
        source_var_panel.set_color(PanelColor::Dark);

        // source_var_panel.add_text_display("Selected".to_string()).set_text_scale(TextSize::ExtraSmall);

        let mut var_slot = VarSlot::new(&ref_manager.selected_var.clone());
        var_slot.set_dragging_properties(true, true, true);
        var_slot.set_allowed_type(var::var_type::VarTypeKind::Any);
        source_var_panel.add_widget(var_slot.wrap_into_widget());
        

        let slot_sub_panel = panel.add_sub_panel();
        slot_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        slot_sub_panel.set_color(PanelColor::Dark);

        for _i in 0..11 {
            // Init Ref
            let blank_var = Var::Game(GameVar::Primitive(PrimitiveVar::Block(crate::game_data::types::BlockTexture::Air)));
            let var_instance = Rc::new(RefCell::new(blank_var));
            
            // Init Widget
            let mut var_slot = VarSlot::new(&var_instance);
            var_slot.set_dragging_properties(true, true, true);
            var_slot.set_allowed_type(var::var_type::VarTypeKind::Any);

            // Add to panel
            slot_sub_panel.add_widget(var_slot.wrap_into_widget());
        }


        
        panel.size();
    }
    return panel;
}