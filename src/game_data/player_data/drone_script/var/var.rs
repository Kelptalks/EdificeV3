use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::var::{game_vars::game_var_type::GameVarKind, var_type::{VarKind, VarType}}, texture_manager::texture::Texture};



/*
############
## Header ##
############
Comments
*/
#[derive(PartialEq, Clone)]
pub struct Var {
    name: String,
    
    var_kind: VarKind,
    var_type_ref: Rc<RefCell<VarType>>,
}

impl Var {

    //=====================================
    // Constructors
    //=====================================
    pub fn new_with_var_type(var_type: VarType) -> Var {
        Var {
            name: var_type.get_name(),

            var_kind: var_type.to_kind(), 
            var_type_ref: Rc::new(RefCell::new(var_type)),
        }
    }
    
    pub fn new_blank() -> Var {
        Var {
            name: "BlankVar".to_string(),

            var_kind: VarKind::Any,
            var_type_ref: Rc::new(RefCell::new(VarType::Null())),
        }
    }

    pub fn new_blank_with_kind(var_kind: VarKind) -> Var {
        Var {
            name: "Null".to_string(),

            var_kind,
            var_type_ref: Rc::new(RefCell::new(VarType::Null())),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            var_kind: self.var_kind.clone(),
            var_type_ref: Rc::new(RefCell::new(self.var_type_ref.borrow().clone())),
        }
    }
    
    //=====================================
    // getters
    //=====================================

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_kind(&self) -> VarKind {
        return self.var_kind;
    }

    pub fn get_texture(&self) -> Texture {
        self.var_type_ref.borrow().get_texture()
    }

    pub fn get_kind_texture(&self) -> Texture {
        self.var_kind.get_texture()
    }

    pub fn get_var_type_ref(&self) -> Rc<RefCell<VarType>> {
        self.var_type_ref.clone()
    }

    pub fn is_null(&self) -> bool {
        self.var_type_ref.borrow().is_null()
    }

    //=====================================
    // Setters
    //=====================================

    pub fn set_value(&mut self, var_type: VarType) {
        if var_type.to_kind() == self.get_kind() {
            *self.var_type_ref.borrow_mut() = var_type
        }
    }

    pub fn set_with_var(&mut self, new_var: &Var) {
        self.name = new_var.name.clone();
        
        *self.var_type_ref.borrow_mut() = new_var.get_var_type_ref().borrow().clone();
    }

    pub fn clear(&mut self) {
        *self.var_type_ref.borrow_mut() = VarType::Null();
    }

    

    //=====================================
    // Converters
    //=====================================

    pub fn into_ref(&self) -> Rc<RefCell<Var>> {
        return Rc::new(RefCell::new(self.clone()))
    }

}
