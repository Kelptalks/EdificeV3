use std::{cell::RefCell, rc::Rc};

use crate::game_data::player_data::locations::{location::WorldLocation, location_manager::LocationManager};

pub struct WorldLocationConfig {

    name: Rc<RefCell<String>>,

    source_location: Rc<RefCell<WorldLocation>>,
}

impl WorldLocationConfig {
    pub fn new(location_ref: Rc<RefCell<WorldLocation>>) -> Rc<RefCell<WorldLocationConfig>> {
        let config = WorldLocationConfig {
            name: Rc::new(RefCell::new("NewLocation                ".to_string())),
            source_location: location_ref,
        };

        return Rc::new(RefCell::new(config));
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

    pub fn create_location_in_manager(&self, location_manager: &mut LocationManager) {
        if location_manager.get_location_with_name(&*self.name.borrow()).is_none(){
            location_manager.create_location(self.name.borrow().clone(), self.source_location.borrow().get_area().clone());
        }
        else {
            *self.name.borrow_mut() = "LOCATION ALREADY EXISTS".to_string();
        }
    }
}