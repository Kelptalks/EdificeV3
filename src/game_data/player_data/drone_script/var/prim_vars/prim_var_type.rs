use crate::game_data::{
    player_data::drone_script::var::{var_properties::{PropKey, PropValue, VarPropModRequest, VarProperty}, var_type::VarType},
    texture_manager::texture::Texture,
    types::UITextures,
};

#[derive(Clone, PartialEq)]
pub enum PrimitiveVarType {
    Bool(bool),
}

impl PrimitiveVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        VarType::Prim(self)
    }
    
    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVarType::Bool(_) => Texture::UITexture(UITextures::BoolVarIcon),
        }
    }

    pub fn to_kind(&self) -> PrimitiveVarTypeKind {
        match self {
            PrimitiveVarType::Bool(_) => PrimitiveVarTypeKind::Bool,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            PrimitiveVarType::Bool(b) => b.to_string(),
        }
    }

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            PrimitiveVarType::Bool(b) => {
                vec![
                    VarProperty { key: PropKey::Name, value: PropValue::Bool(*b), mutible: false },
                ]
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            PrimitiveVarType::Bool(b) => *b = false,
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            PrimitiveVarType::Bool(b) => {
                if let VarPropModRequest::Set(PropKey::Name, PropValue::Bool(val)) = request {
                    *b = val;
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PrimitiveVarTypeKind {
    Bool,
}

impl PrimitiveVarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            PrimitiveVarTypeKind::Bool => Texture::UITexture(UITextures::AnyVarIcon),
        }
    }
}
