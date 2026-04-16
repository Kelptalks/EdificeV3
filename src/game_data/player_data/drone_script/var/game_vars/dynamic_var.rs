use std::{cell::RefCell, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::{drone_script::var::{self, game_vars::game_var_type::GameVarType, programming_vars::programming_var::ProgrammingVar, var::Var, var_properties::{PropKey, PropValue, VarPropModRequest, VarProperty}, var_type::VarType}, drones::drone::Drone, locations::location::WorldLocation}, texture_manager::texture::Texture, types::BlockTexture};


#[derive(Clone)]
pub enum DynamicVarType {
    Location(Option<Rc<RefCell<WorldLocation>>>),
    Drone(Option<Rc<RefCell<Drone>>>),
}

impl PartialEq for DynamicVarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DynamicVarType::Location(Some(a)), DynamicVarType::Location(Some(b))) => Rc::ptr_eq(a, b),
            (DynamicVarType::Location(None), DynamicVarType::Location(None)) => true,
            _ => false,
        }
    }
}

impl DynamicVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        VarType::Game(super::game_var_type::GameVarType::Dynamic(self))
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            DynamicVarType::Location(Some(location)) => {
                return location.borrow().get_texture().clone();
            },
            DynamicVarType::Location(None) => {
                return Texture::BlockTexture(BlockTexture::Air);
            },
            DynamicVarType::Drone(Some(drone_ref)) => {
                return drone_ref.borrow().get_texture();
            },
            DynamicVarType::Drone(None) => {
                return Texture::BlockTexture(BlockTexture::Air);
            },
        }
    }

    pub fn to_kind(&self) -> DynamicVarTypeKind {
        match self {
            DynamicVarType::Location(_) => DynamicVarTypeKind::Location,
            DynamicVarType::Drone(_) => DynamicVarTypeKind::Drone,
        }
    }

    //=====================================
    // Prop Managment
    //=====================================

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            DynamicVarType::Location(location_option_ref) => {
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
            DynamicVarType::Drone(drone_option_ref) => {
                if let Some(drone) = drone_option_ref {
                    let borrow = drone.borrow();

                    vec![
                        VarProperty {key: PropKey::Name,            value: PropValue::String(borrow.get_name().to_string()),                    mutible: true},
                        VarProperty {key: PropKey::Id,              value: PropValue::Num(borrow.get_id() as i32),                              mutible: false},

                        VarProperty {key: PropKey::Cords,           value: PropValue::Cords(borrow.get_cords()), mutible: false},

                        VarProperty {key: PropKey::Health,          value: PropValue::Num(borrow.get_health() as i32),                          mutible: true},
                        VarProperty {key: PropKey::Fuel,            value: PropValue::Num(borrow.get_fuel() as i32),                            mutible: true},
                        VarProperty {key: PropKey::BusyTime,            value: PropValue::Num(borrow.get_busy() as i32),                        mutible: true},
                        VarProperty {key: PropKey::InventorySlots,  value: PropValue::Inventory(borrow.get_inventory().get_slots().clone()),    mutible: true},
                        
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
            VarPropModRequest::Add(prop_key, prop_value) => {
                
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
            VarPropModRequest::Add(prop_key, prop_value) => {
                match prop_key {
                    PropKey::InventorySlots => {
                        if let PropValue::Inventory(slots) = prop_value {
                            drone.borrow_mut().get_mut_inventory().add_inventory_slots(slots);
                        }
                    },
                    _ => {

                    }
                }
            },
        }
    }

    pub fn request_prop(&mut self, request: VarPropModRequest) {
        match self {
            DynamicVarType::Location(location_ref_option) => {
                if let Some(location) = location_ref_option {
                    Self::handle_location_prop_request(location, request)
                }
            },
            DynamicVarType::Drone(drone_ref_option) => {
                if let Some(drone) = drone_ref_option {
                    Self::handle_drone_prop_request(drone, request);
                }
            },
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            DynamicVarType::Location(location_option_ref) => {
                if let Some(location) = location_option_ref {
                    return location.borrow().get_name().to_string();
                }
                return "UNKOWN LOCATION".to_string();
            },
            DynamicVarType::Drone(drone_option_ref) => {
                if let Some(drone) = drone_option_ref {
                    return drone.borrow().get_name();
                }
                return "UNKOWN DRONE".to_string();
            },
        }
    }

    pub fn rename(&mut self, name: String) {
        match self {
            DynamicVarType::Location(ref_option) => {
                if let Some(location_ref) = ref_option {
                    location_ref.borrow_mut().set_name(name);
                }
                else {
                    eprint!("Tried To Rename Unkown Location");
                }
            },
            DynamicVarType::Drone(ref_option) => {
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
            DynamicVarType::Location(option_location_ref) => {
                if let Some(location_rc) = option_location_ref {
                    return Some(*location_rc.borrow().get_area());
                }
                else {
                    return None;
                }
            },
            DynamicVarType::Drone(option_drone_ref) => {
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
            DynamicVarType::Location(option_location_ref) => {
                *option_location_ref = None;
            },
            DynamicVarType::Drone(option_drone_ref) => {
                *option_drone_ref = None;
            },
        }
    }

    //=====================================
    // Constructor
    //=====================================

    pub fn construct_location_var(location: &Option<Rc<RefCell<WorldLocation>>>) -> Var {
        if let Some(location_ref) = location {
            let var = DynamicVarType::Location(Some(location_ref.clone())).wrap_into_var_type();
            Var::new_with_var_type(var)
        }
        else {
            let var = DynamicVarType::Location(None).wrap_into_var_type();
            Var::new_with_var_type(var)
        }
    }

    pub fn construct_drone_var(drone: &Rc<RefCell<Drone>>) -> Var {
        let var = DynamicVarType::Drone(Some(drone.clone())).wrap_into_var_type();
        Var::new_with_var_type(var)
    }

    //=====================================
    // Getters
    //=====================================

    pub fn into_location_ref(var: &Rc<RefCell<VarType>>) -> Option<Rc<RefCell<WorldLocation>>> {
        let borrow = var.borrow();
        if let VarType::Game(GameVarType::Dynamic(DynamicVarType::Location(location_option_ref))) = &*borrow {
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
                return Texture::UITexture(crate::game_data::types::UITextures::LocationVarIcon)
            },
            DynamicVarTypeKind::Drone => {
                return Texture::BlockTexture(BlockTexture::DroneUpRight)
            },
        }
    }
}


