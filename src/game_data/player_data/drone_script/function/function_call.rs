use std::{cell::RefCell, rc::Rc};

use crate::game_data::player_data::drone_script::{function::function::Function, var::var::{Var}};


#[derive(Clone, PartialEq)]
pub struct FunctionCall {
    params: Vec<Var>,
    
    function_ref: Rc<RefCell<Function>>,

    return_value: Var,
}

impl FunctionCall {
    pub fn get_name(&self) -> String {
        self.function_ref.borrow().get_name()
    }
}