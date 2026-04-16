use std::{cell::RefCell, rc::Rc};

use crate::game_data::player_data::drone_script::{function::function::Function, var::var::VarRef};


#[derive(Clone, PartialEq)]
pub struct FunctionCall {
    params: Vec<VarRef>, 
    function_ref: Rc<RefCell<Function>>,
}

impl FunctionCall {
    pub fn get_name(&self) -> String {
        self.function_ref.borrow().get_name()
    }
}