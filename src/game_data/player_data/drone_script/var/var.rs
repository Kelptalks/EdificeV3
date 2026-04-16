use std::{cell::RefCell, rc::Rc};

use crate::game_data::player_data::drone_script::var::var_type::{VarType, VarTypeKind};



#[derive(Clone, PartialEq)]
pub struct Var {
    var_kind: VarTypeKind,
    var: Rc<RefCell<VarType>>,
}

impl Var {
    pub fn new_with_var_type(var_type: VarType) -> Var {
        Var {
            var_kind: var_type.to_kind(), 
            var: Rc::new(RefCell::new(var_type)),
        }
    }
    
    pub fn new_blank() -> Var {
        Var {
            var_kind: VarTypeKind::Any,
            var: Rc::new(RefCell::new(VarType::Null())),
        }
    }
    
    pub fn get_name(&self) -> String {
        self.var.borrow().get_name()
    }

    pub fn into_var_ref(&self) -> VarRef {
        VarRef { 
            var_ref: Rc::new(RefCell::new(self.clone()))
        }
    }

}


#[derive(Clone, PartialEq)]
pub struct VarRef {
    var_ref: Rc<RefCell<Var>>,
}

impl VarRef {
    pub fn get_name(&self) -> String {
        self.var_ref.borrow().get_name()
    }
}
