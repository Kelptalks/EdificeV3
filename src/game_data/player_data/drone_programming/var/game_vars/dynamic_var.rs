use std::{cell::RefCell, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::{drones::drone::Drone, locations::location::WorldLocation}, texture_manager::texture::Texture, types::BlockTexture};


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
    pub fn get_texture(&self) -> Texture {
        match self {
            DynamicVar::Location(Some(location)) => {
                return location.borrow().get_texture();
            },
            DynamicVar::Location(None) => {
                return Texture::UITexture(crate::game_data::types::UITextures::LocationIcon);
            },
            DynamicVar::Drone(Some(drone_ref)) => {
                return drone_ref.borrow().get_texture();
            },
            DynamicVar::Drone(None) => {
                return Texture::BlockTexture(BlockTexture::DroneBotLeft);
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
}

pub enum DynamicVarRef {
    Location(Rc<RefCell<Option<Rc<RefCell<WorldLocation>>>>>),
    Drone(Rc<RefCell<Option<Rc<RefCell<Drone>>>>>),
}

impl DynamicVarRef {
    pub fn to_kind(&self) -> DynamicVarTypeKind {
        match self {
            DynamicVarRef::Location(_) => DynamicVarTypeKind::Location,
            DynamicVarRef::Drone(_) => DynamicVarTypeKind::Drone,
        }
    }

    pub fn set_var_ref(&self, var: DynamicVar) {
        match (self, var) {
            (DynamicVarRef::Location(ref_cell), DynamicVar::Location(location)) => {
                *ref_cell.borrow_mut() = location;
            },
            (DynamicVarRef::Drone(ref_cell), DynamicVar::Drone(drone)) => {
                *ref_cell.borrow_mut() = drone;
            },
            _ => {
                println!("Cannot Set VarRef of different type");
            }
        }
    }

    pub fn to_var(&self) -> DynamicVar {
        match self {
            DynamicVarRef::Location(ref_cell) => DynamicVar::Location(ref_cell.borrow().clone()),
            DynamicVarRef::Drone(ref_cell) => DynamicVar::Drone(ref_cell.borrow().clone()),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            DynamicVarRef::Location(_) => "Location".to_string(),
            DynamicVarRef::Drone(_) => "Drone".to_string(),
        }
    }

    pub fn get_area(&self) -> Option<WorldArea> {
        match self {
            DynamicVarRef::Location(rc) => {
                if let Some(location_rc) = rc.borrow().as_ref() {
                    return Some(*location_rc.borrow().get_area());
                }
                else {
                    return None;
                }
            },
            DynamicVarRef::Drone(rc) => {
                if let Some(drone_rc) = rc.borrow().as_ref() {
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

    pub fn create_ref_var(&self) -> DynamicVarRef {
        match self {
            DynamicVarTypeKind::Location => DynamicVarRef::Location(Rc::new(RefCell::new(None))),
            DynamicVarTypeKind::Drone => DynamicVarRef::Drone(Rc::new(RefCell::new(None))),
        }
    }
}


