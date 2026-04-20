use crate::game_data::player_data::drone_script::{control_flow::condition::Condition, script_element::ScriptElement, var::{var::VarRef, var_type::VarKind}};



#[derive(Clone)]
pub struct ControlFlow {
    condition: Condition,
    body: Vec<ScriptElement>,

}


impl ControlFlow {
    pub fn new_blank() -> ControlFlow {
        ControlFlow {
            condition: Condition::If(VarRef::new_blank_with_kind(VarKind::Any)), 
            body: Vec::new(),
        }
    }

    pub fn get_condition(&self) -> Condition{
        self.condition.clone()
    }

    pub fn wrap_into_script_element(self) -> ScriptElement {
        ScriptElement::ControlFlow(self)
    }

    pub fn get_name(&self) -> String{
        return "Control_Flow".to_string()
    }
}