
use crate::game_data::{player_data::drone_script::var::{game_vars::action_var::ActionVarType, var_properties::{VarPropModRequest, VarProperty}, var_type::VarType}, screen::widget::widget::WidgetType, texture_manager::texture::Texture, types::UITextures};

pub use super::{
    dynamic_var::{DynamicVarType, DynamicVarTypeKind},
    primitive_var::{PrimitiveGameVarType, PrimitiveGameVarTypeKind},
};


// ─── GameVar ──────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub enum GameVarType {
    Primitive(PrimitiveGameVarType),
    Dynamic(DynamicVarType),
    Action(ActionVarType)
}

impl PartialEq for GameVarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GameVarType::Primitive(a), GameVarType::Primitive(b)) => a == b,
            (GameVarType::Dynamic(a), GameVarType::Dynamic(b)) => a == b,
            _ => false,
        }
    }
}

impl GameVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        VarType::Game(self)
    }

    //=====================================
    // Visual
    //=====================================

    pub fn get_texture(&self) -> Texture {
        match self {
            GameVarType::Primitive(p) => p.get_texture(),
            GameVarType::Dynamic(d) => d.get_texture(),
            GameVarType::Action(d) => d.get_texture(),
        }
    }

    pub fn into_widget(&self) -> Option<WidgetType> {
        match self {
            GameVarType::Primitive(primitive_game_var_type) => {
                None
            },
            GameVarType::Dynamic(dynamic_var_type) => {
                dynamic_var_type.into_widget()
            },
            GameVarType::Action(action_var_type) => {
                None
            },
        }
    }

    //=====================================
    // Identity
    //=====================================

    pub fn to_kind(&self) -> GameVarKind {
        match self {
            GameVarType::Primitive(p) => GameVarKind::Primitive(p.to_kind()),
            GameVarType::Dynamic(d) => GameVarKind::Dynamic(d.to_kind()),
            GameVarType::Action(d) => GameVarKind::Action
        }
    }
    
    pub fn get_name(&self) -> String {
        match self {
            GameVarType::Primitive(p) => p.get_name(),
            GameVarType::Dynamic(d) => d.get_name(),
            GameVarType::Action(d) => d.to_string(),
        }
    }

    //=====================================
    // Properties
    //=====================================

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            GameVarType::Primitive(primitive_var) => primitive_var.get_properties(),
            GameVarType::Dynamic(dynamic_var) => dynamic_var.get_properties(),
            GameVarType::Action(d) => Vec::new(),
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            GameVarType::Primitive(primitive_var_type_kind) => {
                eprintln!("NO IMPLEMENTATION IMPLEMENTED FOR MINIPULATING PRIM VARS");
            },
            GameVarType::Dynamic(dynamic_var_type_kind) => {
                dynamic_var_type_kind.request_prop(request);
            },
            GameVarType::Action(d) => {
                eprintln!("NO IMPLEMENTATION IMPLEMENTED FOR MINIPULATING PRIM VARS");
            },
        }
    } 
    
    //=====================================
    // Managment
    //=====================================

    pub fn clear(&mut self) {
        match self {
            GameVarType::Primitive(primitive_var) => primitive_var.clear(),
            GameVarType::Dynamic(dynamic_var) => dynamic_var.clear(),
            GameVarType::Action(d) => todo!(),
        }
    }
}

// ─── GameVarTypeKind ──────────────────────────────────────────────────────────

#[derive(PartialEq, Clone, Copy)]
pub enum GameVarKind {
    Primitive(PrimitiveGameVarTypeKind),
    Dynamic(DynamicVarTypeKind),
    Action
}

impl GameVarKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            GameVarKind::Primitive(p) => p.get_texture(),
            GameVarKind::Dynamic(d) => d.get_texture(),
            GameVarKind::Action => UITextures::FuelIcon.wrap_into_texture()
        }
    }

    
}
