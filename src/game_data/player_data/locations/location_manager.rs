use std::{cell::RefCell, collections::HashMap, panic::Location, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::locations::location::WorldLocation};


/*
#####################
## LocationManager ##
#####################
Managers the retrival and creation of new locations

*/
pub struct LocationManager {
    player_cursor_location_cords: Rc<RefCell<[i32; 3]>>,
    
    location_map: HashMap<u32, Rc<RefCell<WorldLocation>>>,
    location_name_map: HashMap<String, u32>,
    next_id: u32,
}

impl LocationManager {

    //=====================================
    // Init
    //=====================================

    pub fn new() -> LocationManager {
        LocationManager {
            player_cursor_location_cords: Rc::new(RefCell::new([0; 3])),

            location_map: HashMap::new(),
            location_name_map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn create_location(&mut self, name: String, area: WorldArea) -> Rc<RefCell<WorldLocation>>{
        // Add location to maps
        let new_location = Rc::new(RefCell::new(WorldLocation::new(name.clone(), area, self.next_id)));
        self.location_map.insert(self.next_id, new_location.clone());
        self.location_name_map.insert(name, self.next_id);

        self.next_id += 1;

        return new_location;
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_player_cursor_location_cords_ref(&self) -> Rc<RefCell<[i32; 3]>> {
        return self.player_cursor_location_cords.clone();
    }

    pub fn name_to_id(&self, name: &str) -> Option<u32> {
        return self.location_name_map.get(name).cloned();
    }

    // Location
    pub fn get_location_with_name(&self, name: &str) -> Option<Rc<RefCell<WorldLocation>>> {
        if let Some(id) = self.name_to_id(name) {
            return self.location_map.get(&id).cloned();
        }
        else {
            return None;
        }
    }
    pub fn get_mut_location_with_name(&mut self, name: &str) -> Option<Rc<RefCell<WorldLocation>>> {
        if let Some(id) = self.name_to_id(name) {
            return self.location_map.get_mut(&id).cloned();
        }
        else {
            return None;
        }
    }

    

}