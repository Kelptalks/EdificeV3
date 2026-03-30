use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::var::{self, game_vars::{game_var_type::GameVar, primitive_var::PrimitiveVar}, var_type::Var}, screen::widget::{drone_programming::vars::var_slot::{VarSlot}, panel::{panel::{PanelAlignment, PanelOrientation}, panel_color::PanelColor}, widget::WidgetType}};





//=====================================
// Var Ref HotBar
//=====================================
// A set of var slots that can hold any value type simply for the pourpose of
// setting other var slots

pub fn get_widget() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
        panel.set_color(PanelColor::Dark);

        for _i in 0..9 {
            // Init Ref
            let blank_var = Var::Game(GameVar::Primitive(PrimitiveVar::Block(crate::game_data::types::BlockTexture::Air)));
            let var_instance = Rc::new(RefCell::new(blank_var));
            
            // Init Widget
            let mut var_slot = VarSlot::new(&var_instance);
            var_slot.set_dragging_properties(true, true, true);
            var_slot.set_allowed_type(var::var_type::VarTypeKind::Any);

            // Add to panel
            panel.add_widget(var_slot.wrap_into_widget());
        }


        
        panel.size();
    }
    return panel;
}