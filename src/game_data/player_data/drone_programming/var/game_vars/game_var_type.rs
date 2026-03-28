use std::{cell::{Ref, RefCell}, rc::{self, Rc}};

use crate::game_data::{player_data::{drones::drone::{self, Drone}, locations::{location::{self, WorldLocation}, location_manager::LocationManager}}, texture_manager::texture::Texture, types::{BlockTexture, DroneItemTexture, drone_item::DroneItem}};


#[derive(PartialEq)]
pub enum GameVarTypeKind {
    DroneItem,
    Block,
    Location,
    Drone,
}

impl GameVarTypeKind {
    pub fn get_texture(&self) -> Texture {
        match self {
            GameVarTypeKind::DroneItem => {
                return Texture::DroneItemTexture(crate::game_data::types::DroneItemTexture::Ash)
            },
            GameVarTypeKind::Block => {
                return Texture::BlockTexture(BlockTexture::Selector)
            },
            GameVarTypeKind::Location => {
                return Texture::UITexture(crate::game_data::types::UITextures::LocationIcon)
            },
            GameVarTypeKind::Drone => {
                return Texture::BlockTexture(BlockTexture::DroneUpRight)
            },
        }
    }

    pub fn create_ref_var(&self) -> GameVarRef {
        match self {
            GameVarTypeKind::DroneItem => return GameVarRef::DroneItem(Rc::new(RefCell::new(DroneItem::Ash))),
            GameVarTypeKind::Block => return GameVarRef::Block(Rc::new(RefCell::new(BlockTexture::Air))),
            GameVarTypeKind::Location => return GameVarRef::Location(Rc::new(RefCell::new(None))),
            GameVarTypeKind::Drone => return GameVarRef::Drone(Rc::new(RefCell::new(None))),
        }
    }
}

#[derive(Clone)]
pub enum GameVar {
    DroneItem(DroneItem),
    Block(BlockTexture),
    Location(Option<Rc<RefCell<WorldLocation>>>),
    Drone(Option<Rc<RefCell<Drone>>>)
    
}

impl PartialEq for GameVar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GameVar::DroneItem(a), GameVar::DroneItem(b)) => a == b,
            (GameVar::Block(a), GameVar::Block(b)) => a == b,
            (GameVar::Location(Some(a)), GameVar::Location(Some(b))) => Rc::ptr_eq(a, b),
            (GameVar::Location(None), GameVar::Location(None)) => true,
            _ => false,
        }
    }
}

impl GameVar {
    pub fn get_texture(&self) -> Texture {
        match self {
            GameVar::DroneItem(drone_item) => {
                return Texture::DroneItemTexture(drone_item.to_texture_enum());
            },
            GameVar::Block(block_texture) => {
                return Texture::BlockTexture(*block_texture);
            },
            
            GameVar::Location(Some(location)) => {
                return location.borrow().get_texture();
            },
            GameVar::Location(None) => {
                return Texture::UITexture(crate::game_data::types::UITextures::LocationIcon);
            },

            GameVar::Drone(Some(drone_ref)) => {
                return drone_ref.borrow().get_texture();
            }
            GameVar::Drone(None) => {
                return Texture::BlockTexture(BlockTexture::DroneBotLeft);
            },

        }
    }

    pub fn to_kind(&self) -> GameVarTypeKind {
        match self {
            GameVar::DroneItem(_) => return GameVarTypeKind::DroneItem,
            GameVar::Block(_) => return GameVarTypeKind::Block,
            GameVar::Location(_) => return GameVarTypeKind::Location,
            GameVar::Drone(_) => return GameVarTypeKind::Drone,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            GameVar::DroneItem(_) => "DroneItem".to_string(),
            GameVar::Block(block) => block.to_string().to_string(),
            GameVar::Location(location_option_ref) => {
                if let Some(location) = location_option_ref {
                    return location.borrow().get_name().to_string();
                }

                return "UNKOWN LOCATION".to_string();
            }
            GameVar::Drone(drone_option_ref) => {
                if let Some(drone) = drone_option_ref {
                    return drone.borrow().get_name();
                } 

                return "UNKOWN DRONE".to_string();
            }
        }
    }

}

pub enum GameVarRef {
    DroneItem(Rc<RefCell<DroneItem>>),
    Block(Rc<RefCell<BlockTexture>>),
    Location(Rc<RefCell<Option<Rc<RefCell<WorldLocation>>>>>),
    Drone(Rc<RefCell<Option<Rc<RefCell<Drone>>>>>),
}

impl GameVarRef {
    pub fn to_kind(&self) -> GameVarTypeKind {
        match self {
            GameVarRef::DroneItem(_) => return GameVarTypeKind::DroneItem,
            GameVarRef::Block(_) => return GameVarTypeKind::Block,
            GameVarRef::Location(_) => return GameVarTypeKind::Location,
            GameVarRef::Drone(_) => return GameVarTypeKind::Drone,
        }
    }

    pub fn set_var_ref(&self, var: GameVar) {
        match (self, var) {
            (GameVarRef::DroneItem(ref_cell), GameVar::DroneItem(drone_item)) => {
                *ref_cell.borrow_mut() = drone_item;
            },
            (GameVarRef::Block(ref_cell), GameVar::Block(block_texture)) => {
                *ref_cell.borrow_mut() = block_texture;
            },
            (GameVarRef::Location(ref_cell), GameVar::Location(location)) => {
                *ref_cell.borrow_mut() = location;
            },
            (GameVarRef::Drone(ref_cell), GameVar::Drone(drone)) => {
                *ref_cell.borrow_mut() = drone;
            },

            _ => {
                println!("Cannot Set VarRef of different type");
            }
        }
    }

    pub fn to_var(&self) -> GameVar {
        match self {
            GameVarRef::DroneItem(ref_cell) => {
                return GameVar::DroneItem(*ref_cell.borrow());
            },
            GameVarRef::Block(ref_cell) => {
                return GameVar::Block(*ref_cell.borrow());
            },
            GameVarRef::Location(ref_cell) => {
                return GameVar::Location(ref_cell.borrow().clone());
            },
            GameVarRef::Drone(ref_cell) => {
                return GameVar::Drone(ref_cell.borrow().clone());
            }
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            GameVarRef::DroneItem(_) => "DroneItem".to_string(),
            GameVarRef::Block(_) => "Block".to_string(),
            GameVarRef::Location(_) => "Location".to_string(),
            GameVarRef::Drone(_) => "Drone".to_string(),
        }
    }
}