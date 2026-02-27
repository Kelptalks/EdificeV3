use std::collections::HashMap;

use crate::game_data::{locations::world_area::WorldArea, player_data::locations::location::Location};


/*
#####################
## LocationManager ##
#####################
Managers the retrival and creation of new locations

*/
pub struct LocationManager {
    location_map: HashMap<String, Location>
}

impl LocationManager {

    //=====================================
    // Init
    //=====================================

    pub fn new() -> LocationManager {
        LocationManager {
            location_map: HashMap::new(),
        }
    }

    pub fn create_location(&mut self, name: String, area: WorldArea) {
        let new_location = Location::new(name.clone(), area);
        self.location_map.insert(name, new_location);
    }

    //=====================================
    // Getters / Setters
    //=====================================

    // Location
    pub fn get_location(&self, name: String) -> Option<&Location> {
        return self.location_map.get(&name);
    }
    pub fn get_mut_location(&mut self, name: String) -> Option<&mut Location> {
        return self.location_map.get_mut(&name);
    }

    pub fn get_mut_location_map(&mut self) -> &mut HashMap<String, Location> {
        return &mut self.location_map;
    }

    
}