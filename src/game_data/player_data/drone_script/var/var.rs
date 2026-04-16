use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::var::var_type::{VarType, VarTypeKind}, texture_manager::texture::Texture};



/*
############
## Header ##
############
Comments
*/
#[derive(Clone, PartialEq)]
pub struct Var {
    var_kind: VarTypeKind,
    var_type_ref: Rc<RefCell<VarType>>,
}

impl Var {
    //=====================================
    // Constructors
    //=====================================
    pub fn new_with_var_type(var_type: VarType) -> Var {
        Var {
            var_kind: var_type.to_kind(), 
            var_type_ref: Rc::new(RefCell::new(var_type)),
        }
    }
    
    pub fn new_blank() -> Var {
        Var {
            var_kind: VarTypeKind::Any,
            var_type_ref: Rc::new(RefCell::new(VarType::Null())),
        }
    }
    
    //=====================================
    // getters
    //=====================================

    pub fn get_name(&self) -> String {
        self.var_type_ref.borrow().get_name()
    }

    pub fn get_kind(&self) -> VarTypeKind {
        return self.var_kind;
    }

    pub fn get_texture(&self) -> Texture {
        self.var_type_ref.borrow().get_texture()
    }

    pub fn get_kind_texture(&self) -> Texture {
        self.var_kind.get_texture()
    }

    pub fn get_var_type_ref(&self) -> &Rc<RefCell<VarType>> {
        &self.var_type_ref
    }

    //=====================================
    // Setters
    //=====================================

    pub fn set_value(&mut self, var_type: VarType) {
        if var_type.to_kind() == self.get_kind() {
            *self.var_type_ref.borrow_mut() = var_type
        }
    } 

    pub fn clear(&mut self) {
        *self.var_type_ref.borrow_mut() = VarType::Null();
    }

    //=====================================
    // Converters
    //=====================================

    pub fn into_var_ref(&self) -> VarRef {
        VarRef::new_with_var(self)
    }
    
    pub fn into_ref(&self) -> Rc<RefCell<Var>> {
        return Rc::new(RefCell::new(self.clone()))
    }

}



/*
############
## Header ##
############
Comments
*/
#[derive(Clone, PartialEq)]
pub struct VarRef {
    var_kind: VarTypeKind,
    var_ref: Rc<RefCell<Var>>,
}

impl VarRef {
    pub fn new_with_var(var: &Var) -> VarRef {
        VarRef {
            var_ref: var.into_ref(),
            var_kind: var.var_kind
        }
    }

    pub fn new_blank_with_kind(var_kind: VarTypeKind) -> VarRef {
        VarRef {
            var_ref: Rc::new(RefCell::new(Var::new_blank())),
            var_kind: var_kind,
        }
    }

    pub fn set_with_var(&mut self, var: &Var) {
        self.var_ref = var.into_ref();
    }
    
    pub fn get_name(&self) -> String {
        self.var_ref.borrow().get_name()
    }
}
