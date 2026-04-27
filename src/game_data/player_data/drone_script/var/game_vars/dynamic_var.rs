use core::num;
use std::{cell::RefCell, clone, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::{drone_script::var::{self, game_vars::{game_var_type::{GameVarKind, GameVarType}, primitive_game_var::PrimitiveGameVarType}, prim_vars::prim_var_type::{PrimitiveVarKind, PrimitiveVarType}, programming_vars::programming_var::ProgrammingVar, var::Var, var_properties::{PropKey, VarPropModRequest, VarProperty}, var_type::{VarKind, VarType}}, drones::drone::Drone, locations::location::WorldLocation}, screen::{ui_elements::panel, widget::{self, drone_programming::{function_slot::FunctionSlot, scripting_elements::scripting_panel::{self, ScriptingPanel}}, panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType}}, texture_manager::texture::Texture, types::BlockTexture};


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

    pub fn into_widget(&self) -> Option<WidgetType> {
        match self {
            DynamicVarType::Location(ref_cell) => {
                None
            },
            DynamicVarType::Drone(ref_cell) => {
                if let Some(drone) = ref_cell {
                    // Create header
                    let mut panel = Panel::new_blank();
                    
                    let scripting_panel = FunctionSlot::new_with_function(drone.borrow().get_function_ref());

                    panel.add_widget(scripting_panel.wrap_into_widget());
                    panel.size();
                    return Some(panel.wrap_into_widget())
                }
                else {
                    None
                }
            },
        }
    }
    
    //=====================================
    // Identity
    //=====================================

    pub fn to_kind(&self) -> DynamicVarTypeKind {
        match self {
            DynamicVarType::Location(_) => DynamicVarTypeKind::Location,
            DynamicVarType::Drone(_) => DynamicVarTypeKind::Drone,
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

    //=====================================
    // Properties
    //=====================================

    

    pub fn get_properties(&self) -> Vec<VarProperty> {
        match self {
            DynamicVarType::Location(location_option_ref) => {
                if let Some(location) = location_option_ref {
                    let borrow = location.borrow();
                    vec![
                        VarProperty {
                            key: PropKey::Name, 
                            value: PrimitiveVarType::String(borrow.get_name().to_string()).create_var(), 
                            mutible: true
                        },
                        VarProperty {
                            key: PropKey::Id,
                            value: PrimitiveVarType::Num(borrow.get_id() as i32).create_var(), mutible: false
                        },
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
                        VarProperty {key: PropKey::Name,           value: PrimitiveVarType::String(borrow.get_name().to_string()).create_var(),   mutible: true},
                        VarProperty {key: PropKey::Id,             value: PrimitiveVarType::Num(borrow.get_id().as_usize() as i32).create_var(),             mutible: false},

                        VarProperty {key: PropKey::Cords,          value: PrimitiveGameVarType::construct_cords_var(borrow.get_cords()),          mutible: false},

                        VarProperty {key: PropKey::Health,         value: PrimitiveVarType::Num(borrow.get_health() as i32).create_var(),         mutible: true},
                        VarProperty {key: PropKey::Fuel,           value: PrimitiveVarType::Num(borrow.get_fuel() as i32).create_var(),           mutible: true},
                        VarProperty {key: PropKey::BusyTime,       value: PrimitiveVarType::Num(borrow.get_busy() as i32).create_var(),                              mutible: true},
                        VarProperty {key: PropKey::InventorySlots, value: PrimitiveGameVarType::construct_inventory_var(borrow.get_inventory().get_slots().clone()), mutible: true},

                        VarProperty {key: PropKey::MinePower,      value: PrimitiveVarType::Num(borrow.get_mine_power() as i32).create_var(),     mutible: false},
                        VarProperty {key: PropKey::ChopPower,      value: PrimitiveVarType::Num(borrow.get_chop_power() as i32).create_var(),     mutible: false},
                        
                        VarProperty {key: PropKey::Script,         value: ProgrammingVar::Script(borrow.get_function_ref().clone()).create_var(),   mutible: true},
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
            VarPropModRequest::Set(prop_key, var) => {
                match prop_key {
                    PropKey::Name => {
                        location.borrow_mut().set_name(var.as_string().unwrap_or_default());
                    },
                    _ => {
                        eprintln!("set prop key {} not supported for location", prop_key.to_name());
                    }
                }
            },
            VarPropModRequest::Add(prop_key, var) => {},
        }
    }

    pub fn handle_drone_prop_request(drone: &mut Rc<RefCell<Drone>>, request: VarPropModRequest) {
        match request {
            VarPropModRequest::Set(prop_key, var) => {
                match prop_key {
                    PropKey::Name => {
                        drone.borrow_mut().set_name(var.as_string().unwrap_or_default());
                    },
                    PropKey::Fuel => {
                        if let Some(num) = var.as_i32() {
                            if num > 0 { drone.borrow_mut().set_fuel(num as u32); }
                        }
                    },
                    PropKey::Health => {
                        if let Some(num) = var.as_i32() {
                            if num > 0 { drone.borrow_mut().set_health(num as u32); }
                        }
                    },
                    PropKey::BusyTime => {
                        if let Some(num) = var.as_i32() {
                            if num > 0 { drone.borrow_mut().set_busy(num as u32); }
                        }
                    },
                    _ => {
                        eprintln!("set prop key {} not supported for drone", prop_key.to_name());
                    }
                }
            },
            VarPropModRequest::Add(prop_key, var) => {
                match prop_key {
                    // PropKey::InventorySlots => no Var type for inventory
                    _ => {}
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

    //=====================================
    // Managment
    //=====================================

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

    pub fn into_location_ref(var: &Var) -> Option<Rc<RefCell<WorldLocation>>> {
        let var_type_ref = var.get_var_type_ref();
        let borrow = var_type_ref.borrow();
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

    pub fn wrap_into_var_kind(self) -> VarKind {
        VarKind::Game(GameVarKind::Dynamic(self))
    }
}


