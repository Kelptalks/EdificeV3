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

    pub fn new_blank_with_kind(var_type: VarTypeKind) -> Var {
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
    var_type_ref: Option<Rc<RefCell<VarType>>>,
}

impl VarRef {

    //=====================================
    // Constructors
    //=====================================

    pub fn new_with_var(var: &Var) -> VarRef {
        VarRef {
            var_type_ref: Some(var.get_var_type_ref().clone()),
            var_kind: var.var_kind
        }
    }

    pub fn new_blank_with_kind(var_kind: VarTypeKind) -> VarRef {
        VarRef {
            var_type_ref: None,
            var_kind: var_kind,
        }
    }
    
    //=====================================
    // getters
    //=====================================

    pub fn get_name(&self) -> String {
        if let Some(var_type_ref) = &self.var_type_ref {
            var_type_ref.borrow().get_name()
        }
        else {
            "No Ref".to_string()
        }
    }

    pub fn get_var_type_ref(&self) -> Rc<RefCell<VarType>> {
        if let Some(var_type_ref) = &self.var_type_ref {
            var_type_ref.clone()
        }
        else {
            Rc::new(RefCell::new(VarType::Null()))
        }
    }

    pub fn get_kind_texture(&self) -> Texture {
        self.var_kind.get_texture()
    }

    pub fn get_texture(&self) -> Texture {
        if let Some(var_type_ref) = &self.var_type_ref {
            var_type_ref.borrow().get_texture()
        }
        else {
            Texture::BlockTexture(crate::game_data::types::BlockTexture::Air)
        }
    }

    pub fn is_null(&self) -> bool {
        if let Some(var_type_ref) = &self.var_type_ref {
            var_type_ref.borrow().is_null()
        }
        else {
            true
        }
    }

    //=====================================
    // Setters
    //=====================================

    pub fn set_with_var(&mut self, var: &Var) {
        if var.get_kind() == self.var_kind {
            self.var_type_ref = Some(var.get_var_type_ref().clone());
        }
    }

    pub fn set_with_var_type_ref(&mut self, var_type_ref: &Rc<RefCell<VarType>>) {
        if var_type_ref.borrow().to_kind() == self.var_kind {
            self.var_type_ref = Some(var_type_ref.clone());
        }
    }

    pub fn clear(&mut self) {
        self.var_type_ref = None
    }

}
