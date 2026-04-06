use std::{cell::RefCell, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::{drone_programming::var::{game_vars::game_var_type::GameVar, var_type::Var}, drones::drone::Drone, locations::location::WorldLocation}, texture_manager::texture::Texture, types::BlockTexture};


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

    pub fn rename(&mut self, name: String) {
        match self {
            DynamicVar::Location(ref_option) => {
                if let Some(location_ref) = ref_option {
                    location_ref.borrow_mut().set_name(name);
                }
                else {
                    eprint!("Tried To Rename Unkown Location");
                }
            },
            DynamicVar::Drone(ref_option) => {
                if let Some(drone_ref) = ref_option {
                    drone_ref.borrow_mut().set_name(name);
                }
                else {
                    eprint!("Tried To Rename Unkown Drone");
                }
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
                    area.set_point_1_cords(drone_cords);
                    area.set_point_2_cords(drone_cords);

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

    //=====================================
    // Constructor
    //=====================================

    pub fn construct_location_var_ref(location: &Option<Rc<RefCell<WorldLocation>>>) -> Rc<RefCell<Var>>{
        if let Some(location_ref) = location {
            let var = DynamicVar::Location(Some(location_ref.clone())).wrap_into_var();
            Rc::new(RefCell::new(var))
        }
        else {
            let var = DynamicVar::Location(None).wrap_into_var();
            Rc::new(RefCell::new(var))
        }
    }

    pub fn construct_drone_var_ref(drone: &Rc<RefCell<Drone>>) -> Rc<RefCell<Var>> {
        let var = DynamicVar::Drone(Some(drone.clone())).wrap_into_var();
        Rc::new(RefCell::new(var))
    }

    //=====================================
    // Getters
    //=====================================

    pub fn into_location_ref(var: &Rc<RefCell<Var>>) -> Option<Rc<RefCell<WorldLocation>>> {
        let borrow = var.borrow();
        if let Var::Game(GameVar::Dynamic(DynamicVar::Location(location_option_ref))) = &*borrow {
            return location_option_ref.clone()
        }
        else {
            eprintln!("Failed to convert var {} to location", borrow.get_name());
            return None
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


