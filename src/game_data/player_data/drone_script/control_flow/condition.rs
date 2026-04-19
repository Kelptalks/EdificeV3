

use miniquad::Comparison;

use crate::game_data::player_data::drone_script::var::var::VarRef;





#[derive(Clone)]
pub enum Condition {
    ConditionBool(VarRef),
    ConditionComparison(Comparison),

}

impl Condition {
    
}