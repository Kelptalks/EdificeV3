use std::{cell::RefCell, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::locations::location::WorldLocation};

pub struct WorldLocationConfig {

    name: Rc<RefCell<String>>,

    source_location: Rc<RefCell<WorldLocation>>,
}

impl WorldLocationConfig {
    pub fn new(location_ref: Rc<RefCell<WorldLocation>>) -> WorldLocationConfig {
        WorldLocationConfig {
            name: Rc::new(RefCell::new("".to_string())),
            source_location: location_ref,
        }
    }

    pub fn get_source_location_ref(&self) -> Rc<RefCell<WorldLocation>> {
        return self.source_location.clone();
    }


    pub fn get_location_name_ref(&self) -> Rc<RefCell<String>> {
        return self.name.clone();
    }

    pub fn get_location_name(&self) -> String {
        return self.name.borrow_mut().clone();
    }
}