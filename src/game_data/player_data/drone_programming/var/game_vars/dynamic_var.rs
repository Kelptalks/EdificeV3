use std::{cell::RefCell, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::{drone_programming::var::{game_vars::game_var_type::GameVar, var_properties::{PropKey, PropValue, VarPropModRequest, VarProperty}, var_type::Var}, drones::drone::Drone, locations::location::WorldLocation}, texture_manager::texture::Texture, types::BlockTexture};


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

    //=====================================
    // Prop Managment
    //=====================================

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            DynamicVar::Location(location_option_ref) => {
                if let Some(location) = location_option_ref {
                    let borrow = location.borrow();
                    vec![
                        VarProperty {key: PropKey::Name, value: PropValue::String(borrow.get_name().to_string()), mutible: true},
                        VarProperty {key: PropKey::Id,   value: PropValue::Num(borrow.get_id() as i32),           mutible: false},
                    ]
                }
                else {
                    eprintln!("Cannot Accsess Properties of NULL location var");
                    return Vec::new();
                }
            },
            DynamicVar::Drone(drone_option_ref) => {
                if let Some(drone) = drone_option_ref {
                    let borrow = drone.borrow();

                    vec![
                        VarProperty {key: PropKey::Name,            value: PropValue::String(borrow.get_name().to_string()),                    mutible: true},
                        VarProperty {key: PropKey::Id,              value: PropValue::Num(borrow.get_id() as i32),                              mutible: false},

                        VarProperty {key: PropKey::Cords,           value: PropValue::Cords(borrow.get_cords()), mutible: false},

                        VarProperty {key: PropKey::Health,          value: PropValue::Num(borrow.get_health() as i32),                          mutible: true},
                        VarProperty {key: PropKey::Fuel,            value: PropValue::Num(borrow.get_fuel() as i32),                            mutible: true},
                        VarProperty {key: PropKey::BusyTime,            value: PropValue::Num(borrow.get_busy() as i32),                            mutible: true},
                        VarProperty {key: PropKey::InventorySlots,  value: PropValue::Inventory(borrow.get_inventory().get_slots().clone()),    mutible: false},
                        
                        VarProperty {key: PropKey::MinePower,       value: PropValue::Num(borrow.get_mine_power() as i32),                      mutible: false},
                        VarProperty {key: PropKey::ChopPower,       value: PropValue::Num(borrow.get_chop_power() as i32),                      mutible: false},
                    ]
                    
                }
                else {
                    eprintln!("Cannot Accsess Properties of NULL drone var");
                    return Vec::new();
                }
            },
        }
    }


    pub fn handle_location_prop_request(location: &mut Rc<RefCell<WorldLocation>>, request: VarPropModRequest) {
        match request {
            VarPropModRequest::Set(prop_key, prop_value) => {
                match prop_key {
                    PropKey::Name => {
                        location.borrow_mut().set_name(prop_value.into_string());
                    },
                    _ => {
                        eprintln!("set prop key {} not supported for location", prop_key.to_name());
                    }
                }
        
            },
        }
    }

    pub fn handle_drone_prop_request(drone: &mut Rc<RefCell<Drone>>, request: VarPropModRequest) {
        match request {
            VarPropModRequest::Set(prop_key, prop_value) => {
                match prop_key {
                    PropKey::Name => {
                        drone.borrow_mut().set_name(prop_value.into_string());
                    },
                    PropKey::Fuel => {
                        let num = prop_value.into_num();
                        if num > 0 {
                            drone.borrow_mut().set_fuel(num as u32);
                        }
                    },
                    PropKey::Health => {
                        let num = prop_value.into_num();
                        if num > 0 {
                            drone.borrow_mut().set_health(num as u32);
                        }
                    }
                    PropKey::BusyTime => {
                        let num = prop_value.into_num();
                        if num > 0 {
                            drone.borrow_mut().set_busy(num as u32);
                        }
                    }
                    _ => {
                        eprintln!("set prop key {} not supported for drone", prop_key.to_name());
                    }
                }
        
            },
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            DynamicVar::Location(location_ref_option) => {
                if let Some(location) = location_ref_option {
                    Self::handle_location_prop_request(location, request)
                }
            },
            DynamicVar::Drone(drone_ref_option) => {
                if let Some(drone) = drone_ref_option {
                    Self::handle_drone_prop_request(drone, request);
                }
            },
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

#[derive(PartialEq, Clone, Copy)]
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


