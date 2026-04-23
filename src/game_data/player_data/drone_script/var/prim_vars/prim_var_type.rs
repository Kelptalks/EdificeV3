use crate::game_data::{
    player_data::drone_script::var::{var::Var, var_properties::{PropKey, PropValue, VarPropModRequest, VarProperty}, var_type::{VarKind, VarType}}, screen::widget::{text::header::TextDisplay, widget::WidgetType}, texture_manager::texture::Texture, types::UITextures
};

#[derive(Clone, PartialEq)]
pub enum PrimitiveVarType {
    Bool(bool),
    Num(i32),
    String(String),

}

impl PrimitiveVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        VarType::Prim(self)
    }

    pub fn create_var(self) -> Var {
        self.wrap_into_var_type().create_var()
    }

    //=====================================
    // Visual
    //=====================================

    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVarType::Bool(_) => Texture::UITexture(UITextures::BoolVarIcon),
            PrimitiveVarType::Num(_) => Texture::UITexture(UITextures::NumVarIcon),
            PrimitiveVarType::String(_) => Texture::UITexture(UITextures::ScallingIconMidCenter)
        }
    }

    pub fn into_widget(&self) -> Option<WidgetType> {
        match self {
            PrimitiveVarType::Bool(value) => {
                let string = format!("Bool: {}", value);
                return Some(TextDisplay::new(string).wrap_into_widget());
            },
            PrimitiveVarType::Num(_) => {
                None
            },
            PrimitiveVarType::String(_) => {
                None
            }
        }
    }

    //=====================================
    // Identity
    //=====================================

    pub fn to_kind(&self) -> PrimitiveVarKind {
        match self {
            PrimitiveVarType::Bool(_) => PrimitiveVarKind::Bool,
            PrimitiveVarType::Num(_) => PrimitiveVarKind::Num,
            PrimitiveVarType::String(_) => PrimitiveVarKind::String,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            PrimitiveVarType::Bool(b) => b.to_string(),
            PrimitiveVarType::Num(n) => n.to_string(),
            PrimitiveVarType::String(n) => n.to_string()
        }
    }

    //=====================================
    // Properties
    //=====================================

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            PrimitiveVarType::Bool(b) => {
                vec![
                    VarProperty {
                        key: PropKey::Name, 
                        value: PrimitiveVarType::Bool(*b).wrap_into_var_type().create_var(), 
                        mutible: false
                    },
                ]
            }
            PrimitiveVarType::Num(n) => {
                vec![
                    VarProperty {
                        key: PropKey::Name, 
                        value: PrimitiveVarType::Num(*n).wrap_into_var_type().create_var(), 
                        mutible: false
                    },
                ]
            }
            PrimitiveVarType::String(s) => {
                vec![
                    VarProperty {
                        key: PropKey::Name, 
                        value: PrimitiveVarType::String(s.clone()).wrap_into_var_type().create_var(), 
                        mutible: false
                    },
                ]
            }
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            PrimitiveVarType::Bool(b) => {
                if let VarPropModRequest::Set(PropKey::Name, var) = request {
                    if let Some(var) = var.as_bool() {
                        *b = var;
                    }
                }
            },

            PrimitiveVarType::Num(n) => {
                if let VarPropModRequest::Set(PropKey::Name, var) = request {
                    if let Some(var) = var.as_i32() {
                        *n = var;
                    }
                }
            },

            PrimitiveVarType::String(s) => {
                todo!()
            }
        }
    }

    //=====================================
    // Managment
    //=====================================


    pub fn clear(&mut self) {
        match self {
            PrimitiveVarType::Bool(b) => *b = false,
            PrimitiveVarType::Num(n) => *n = 0,
            PrimitiveVarType::String(s) => s.clear(),
        }

    }

}

#[derive(Clone, Copy, PartialEq)]
pub enum PrimitiveVarKind {
    Bool,
    Num,
    String,
}

impl PrimitiveVarKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVarKind::Bool => Texture::UITexture(UITextures::BoolVarIcon),
            PrimitiveVarKind::Num => Texture::UITexture(UITextures::NumVarIcon),
            PrimitiveVarKind::String => Texture::UITexture(UITextures::ScallingIconMidCenter),
        }
    }


    pub fn wrap_into_var_kind(self) -> VarKind {
        VarKind::Prim(self)
    }
}
