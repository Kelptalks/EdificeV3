

use crate::game_data::{player_data::drone_script::var::{game_vars::game_var_type::{GameVarType, GameVarKind}, prim_vars::prim_var_type::{PrimitiveVarKind, PrimitiveVarType}, programming_vars::programming_var::{ProgrammingVar, ProgrammingVarKind}, var::Var, var_properties::{VarPropModRequest, VarProperty}}, screen::widget::widget::WidgetType, texture_manager::texture::Texture, types::BlockTexture};



#[derive(Clone, Copy)]
pub enum VarKind {
    Any,
    Game(GameVarKind),
    ProgrammingVar(ProgrammingVarKind),
    Prim(PrimitiveVarKind),
}

impl PartialEq for VarKind {
    fn eq(&self, other: &Self) -> bool {
        if matches!(self, Self::Any) || matches!(other, Self::Any) {
            return true;
        }
        match (self, other) {
            (Self::Game(a), Self::Game(b)) => a == b,
            (Self::Prim(a), Self::Prim(b)) => a == b,
            _ => false,
        }
    }
}

impl VarKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            VarKind::Any => {
                return Texture::UITexture(crate::game_data::types::UITextures::AnyVarIcon);
            },
            VarKind::Game(game_var_type_kind) => {
                return game_var_type_kind.get_texture();
            },
            VarKind::ProgrammingVar(programming_var_kind) => {
                return programming_var_kind.get_texture();
            },
            VarKind::Prim(prim_var_type_kind) => {
                return prim_var_type_kind.get_texture();
            },
        }
    }
}


#[derive(Clone, PartialEq)]
pub enum VarType {
    Null(),
    Game(GameVarType),
    ProgrammingVar(ProgrammingVar),
    Prim(PrimitiveVarType),
}

impl VarType {
    pub fn create_var(self) -> Var {
        Var::new_with_var_type(self)
    }

    pub fn is_null(&self) -> bool {
        match self {
            VarType::Null() => {
                true
            }
            _ => {
                false
            }
        }  
    }

    //=====================================
    // Visual
    //=====================================

    pub fn get_texture(&self) -> Texture {
        match self {
            VarType::Null() => {
                Texture::BlockTexture(BlockTexture::Air)
            },
            VarType::Game(game_var_type) => game_var_type.get_texture(),
            VarType::ProgrammingVar(programming_var) => programming_var.get_texture(),
            VarType::Prim(prim) => prim.get_texture(),
        }
    }

    pub fn into_widget(&self) -> Option<WidgetType> {
        match self {
            VarType::Null() => None,
            VarType::Game(game_var_type) => {
                game_var_type.into_widget()
            },
            VarType::ProgrammingVar(programming_var) => {
                programming_var.into_widget()
            },
            VarType::Prim(primitive_var_type) => {
                primitive_var_type.into_widget()
            },
        }
    }

    //=====================================
    // Identity
    //=====================================

    pub fn to_kind(&self) -> VarKind {
        match self {
            VarType::Null() => {
                VarKind::Any
            },
            VarType::Game(game_var_type) => {
                VarKind::Game(game_var_type.to_kind())

            },
            VarType::ProgrammingVar(programming_var) => {
                VarKind::ProgrammingVar(programming_var.to_kind())
            },
            VarType::Prim(prim) => {
                VarKind::Prim(prim.to_kind())
            },
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            VarType::Null() => {
                return "NULL".to_string()
            },
            VarType::Game(game_var) => game_var.get_name(),
            VarType::ProgrammingVar(programming_var) => programming_var.get_name(),
            VarType::Prim(prim) => prim.get_name(),
        }
    }
    

    //=====================================
    // Properties
    //=====================================

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            VarType::Null() => {
                return Vec::new()
            }
            VarType::Game(game_var) => {
                game_var.get_properties()
            }
            VarType::ProgrammingVar(programming_var) => {
                programming_var.get_properties()
            }
            VarType::Prim(prim) => {
                prim.get_properties()
            }
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            VarType::Null() => {

            },
            VarType::Game(game_var) => {
                game_var.request_prop(request)
            },
            VarType::ProgrammingVar(programming_var) => {
                programming_var.request_prop(request);
            },
            VarType::Prim(prim) => {
                prim.request_prop(request);
            },
        }
    }

    //=====================================
    // Managment
    //=====================================

    pub fn clear(&mut self) {
        match self {
            VarType::Null() => {

            }
            VarType::Game(game_var) => {game_var.clear()},
            VarType::ProgrammingVar(programming_var) => {programming_var.clear();},
            VarType::Prim(prim) => {prim.clear();},
        }
    }


}
