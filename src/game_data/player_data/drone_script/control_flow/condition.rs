


use crate::game_data::{player_data::drone_script::{control_flow::comparison::{Comparison, ComparisonOperators}, var::var::Var}, screen::widget::{panel::panel::Panel, prelude::VarSlot, widget::WidgetType}, texture_manager::texture::Texture, types::UITextures};





#[derive(Clone)]
pub enum Condition {
    If(Var),
    IfCondition(Comparison),
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Condition {
    pub fn get_name(&self) -> String {
        match self {
            Condition::If(var_ref) => var_ref.get_name(),
            Condition::IfCondition(comparison) => comparison.get_name(),
        }
    }


    pub fn get_widget(&self) -> WidgetType {
        match self {
            Condition::If(var) => {
                let mut panel = Panel::new_blank();

                panel.add_text_display("If".to_string());

                let mut var_slot = VarSlot::new_with_var(var.clone());
                var_slot.set_dragging_properties(false, true, true);
                panel.add_widget(var_slot.wrap_into_widget());


                return panel.wrap_into_widget()   
            },
            Condition::IfCondition(comparison) => {
                let mut panel = Panel::new_blank();

                panel.add_text_display("If".to_string());

                panel.add_widget(comparison.get_widget());

                return panel.wrap_into_widget()
                
            },
        }
    }


    pub fn get_texture(&self) -> Texture {
        match self {
            Condition::If(_) => UITextures::IfIcon.wrap_into_texture(),
            Condition::IfCondition(_) => UITextures::IfIcon.wrap_into_texture(),
        }
    }
}