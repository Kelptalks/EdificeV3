use std::f32::consts::PI;

use crate::game_data::{player_data::drone_script::var::{programming_vars::programming_var::ProgrammingVarKind, var::Var, var_type::VarKind}, screen::widget::{panel::panel::Panel, prelude::VarSlot, widget::WidgetType}};


#[derive(Clone)]
pub enum ComparisonOperators {
    Not,
    Equal,
    Or,
    And,
}

impl ComparisonOperators {
    pub fn get_name(&self) -> String {
        match self {
            ComparisonOperators::Not => "!".to_string(),
            ComparisonOperators::Equal => "==".to_string(),
            ComparisonOperators::Or => "||".to_string(),
            ComparisonOperators::And => "&&".to_string(),
        }
    }
}



#[derive(Clone)]
pub struct Comparison {
    var_1: Var,

    comparison_operator: ComparisonOperators,

    var_2: Var,
}

impl Comparison {
    pub fn new() -> Comparison {
        Comparison {
            var_1: Var::new_blank_with_kind(VarKind::Any), 
            comparison_operator: ComparisonOperators::And, 
            var_2: Var::new_blank_with_kind(VarKind::Any), 
        }
    }

    pub fn get_name(&self) -> String {
        format!("({}, {:?}, {}", 
            self.var_1.get_name(), 
            self.comparison_operator.get_name(), 
            self.var_1.get_name()
        )
    }

    pub fn get_widget(&self) -> WidgetType {
        let mut panel = Panel::new_blank();
        
        let var_slot_1 = VarSlot::new_with_var(self.var_1.clone());
        panel.add_widget(var_slot_1.wrap_into_widget());

        let let_condition_var_slot = VarSlot::new_with_kind(ProgrammingVarKind::Condition().wrap_into_kind());
        panel.add_widget(let_condition_var_slot.wrap_into_widget());

        let var_slot_2 = VarSlot::new_with_var(self.var_2.clone());
        panel.add_widget(var_slot_2.wrap_into_widget());

        return panel.wrap_into_widget()

    }
}