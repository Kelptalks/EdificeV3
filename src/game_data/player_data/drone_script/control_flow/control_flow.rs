use crate::game_data::player_data::drone_script::{control_flow::condition::Condition, element_body::ScriptElementBody, script_element::ScriptElement, var::{prim_vars::prim_var_type::PrimitiveVarKind, var::Var, var_type::VarKind}};



#[derive(Clone)]
pub struct ControlFlow {
    condition: Condition,
    body: ScriptElementBody,

}


impl ControlFlow {
    pub fn new_blank() -> ControlFlow {
        println!("test");
        ControlFlow {
            condition: Condition::If(Var::new_blank_with_kind(VarKind::Prim(PrimitiveVarKind::Bool))), 
            body: ScriptElementBody::new(),
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

    //=====================================
    // Execution
    //=====================================

    //=====================================
    // Values
    //=====================================

    pub fn get_mut_body(&mut self) -> &mut ScriptElementBody {
        return &mut self.body
    }

    pub fn get_body(&self) -> &ScriptElementBody {
        &self.body
    }
}