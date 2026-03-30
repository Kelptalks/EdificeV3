use std::{cell::RefCell, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::{drone_programming::var::var_type::Var, drones::drone::Drone, locations::location::WorldLocation}, texture_manager::texture::Texture, types::BlockTexture};


#[derive(Clone)]
pub enum DynamicVar {
    Location(Option<Rc<RefCell<WorldLocation>>>),
    Drone(Option<Rc<RefCell<Drone>>>),
}

impl PartialEq for DynamicVar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DynamicVar::Location(Some(a)), DynamicVar::Location(Some(b))) => Rc::ptr_eq(a, b),
            (DynamicVar::Location(None), DynamicVar::Location(None)) => true,
            _ => false,
        }
    }
}

impl DynamicVar {
    pub fn wrap_into_var(self) -> Var {
        Var::Game(super::game_var_type::GameVar::Dynamic(self))
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            DynamicVar::Location(Some(location)) => {
                return location.borrow().get_texture().clone();
            },
            DynamicVar::Location(None) => {
                return Texture::BlockTexture(BlockTexture::Air);
            },
            DynamicVar::Drone(Some(drone_ref)) => {
                return drone_ref.borrow().get_texture();
            },
            DynamicVar::Drone(None) => {
                return Texture::BlockTexture(BlockTexture::Air);
            },
        }
    }

    pub fn to_kind(&self) -> DynamicVarTypeKind {
        match self {
            DynamicVar::Location(_) => DynamicVarTypeKind::Location,
            DynamicVar::Drone(_) => DynamicVarTypeKind::Drone,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            DynamicVar::Location(location_option_ref) => {
                if let Some(location) = location_option_ref {
                    return location.borrow().get_name().to_string();
                }
                return "UNKOWN LOCATION".to_string();
            },
            DynamicVar::Drone(drone_option_ref) => {
                if let Some(drone) = drone_option_ref {
                    return drone.borrow().get_name();
                }
                return "UNKOWN DRONE".to_string();
            },
        }
    }

    pub fn get_area(&self) -> Option<WorldArea> {
        match self {
            DynamicVar::Location(option_location_ref) => {
                if let Some(location_rc) = option_location_ref {
                    return Some(*location_rc.borrow().get_area());
                }
                else {
                    return None;
                }
            },
            DynamicVar::Drone(option_drone_ref) => {
                if let Some(drone_rc) = option_drone_ref {
                    let drone_cords = drone_rc.borrow().get_cords();

                    let mut area = WorldArea::new_blank();
                    area.set_point_1(drone_cords);
                    area.set_point_2(drone_cords);

                    return Some(area);
                }
                else {
                    return None;
                }
            },
        }
    }

    pub fn clear(&mut self) {
        match self {
            DynamicVar::Location(option_location_ref) => {
                *option_location_ref = None;
            },
            DynamicVar::Drone(option_drone_ref) => {
                *option_drone_ref = None;
            },
        }
    }

}

#[derive(PartialEq)]
pub enum DynamicVarTypeKind {
    Location,
    Drone,
}

impl DynamicVarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            DynamicVarTypeKind::Location => {
                return Texture::UITexture(crate::game_data::types::UITextures::LocationIcon)
            },
            DynamicVarTypeKind::Drone => {
                return Texture::BlockTexture(BlockTexture::DroneUpRight)
            },
        }
    }
}


