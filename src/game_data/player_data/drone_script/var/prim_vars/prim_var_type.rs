use crate::game_data::{
    player_data::drone_script::var::{var_properties::{PropKey, PropValue, VarPropModRequest, VarProperty}, var_type::{VarType, VarKind}},
    texture_manager::texture::Texture,
    types::UITextures,
};

#[derive(Clone, PartialEq)]
pub enum PrimitiveVarType {
    Bool(bool),
    Num(i32),
}

impl PrimitiveVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        VarType::Prim(self)
    }
    
    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVarType::Bool(_) => Texture::UITexture(UITextures::BoolVarIcon),
            PrimitiveVarType::Num(_) => Texture::UITexture(UITextures::NumVarIcon),
        }
    }

    pub fn to_kind(&self) -> PrimitiveVarKind {
        match self {
            PrimitiveVarType::Bool(_) => PrimitiveVarKind::Bool,
            PrimitiveVarType::Num(_) => PrimitiveVarKind::Num,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            PrimitiveVarType::Bool(b) => b.to_string(),
            PrimitiveVarType::Num(n) => n.to_string(),
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            PrimitiveVarType::Bool(b) => {
                vec![
                    VarProperty { key: PropKey::Name, value: PropValue::Bool(*b), mutible: false },
                ]
            }
            PrimitiveVarType::Num(n) => {
                vec![
                    VarProperty { key: PropKey::Name, value: PropValue::Num(*n), mutible: false },
                ]
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            PrimitiveVarType::Bool(b) => *b = false,
            PrimitiveVarType::Num(n) => *n = 0,
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            PrimitiveVarType::Bool(b) => {
                if let VarPropModRequest::Set(PropKey::Name, PropValue::Bool(val)) = request {
                    *b = val;
                }
            },

            PrimitiveVarType::Num(n) => {
                if let VarPropModRequest::Set(PropKey::Name, PropValue::Num(val)) = request {
                    *n = val;
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PrimitiveVarKind {
    Bool,
    Num,
}

impl PrimitiveVarKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVarKind::Bool => Texture::UITexture(UITextures::BoolVarIcon),
            PrimitiveVarKind::Num => Texture::UITexture(UITextures::NumVarIcon),
        }
    }

    pub fn wrap_into_var_kind(self) -> VarKind {
        VarKind::Prim(self)
    }
}
