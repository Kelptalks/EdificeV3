use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::locations::location::WorldLocation};


#[derive(Clone, Copy)]
pub struct LocationId {
    id: usize
}

impl LocationId {
    pub fn as_usize(&self) -> usize {
        self.id
    }
}

/*
#####################
## LocationManager ##
#####################
Managers the retrival and creation of new locations

*/
pub struct LocationManager {
    player_location_map: HashMap<u32, Rc<RefCell<WorldLocation>>>,
    all_play_locations_vec_ref: Rc<RefCell<Vec<Rc<RefCell<WorldLocation>>>>>,
    
    player_location_name_map: HashMap<String, u32>,
    next_id: u32,


}

impl LocationManager {

    //=====================================
    // Init
    //=====================================

    pub fn new() -> LocationManager {
        LocationManager {
            player_location_map: HashMap::new(),
            player_location_name_map: HashMap::new(),
            all_play_locations_vec_ref: Rc::new(RefCell::new(Vec::new())),

            next_id: 0,
        }
    }

    //=====================================
    // Location Creation
    //=====================================

    pub fn create_location(&mut self, name: String, area: WorldArea) -> Rc<RefCell<WorldLocation>>{
        // Add location to maps
        let new_location = Rc::new(RefCell::new(WorldLocation::new(name.clone(), area, self.next_id)));
        self.player_location_map.insert(self.next_id, new_location.clone());
        self.player_location_name_map.insert(name, self.next_id);
        self.all_play_locations_vec_ref.borrow_mut().push(new_location.clone());
        // let widget = VarSource::new(Var::Game(GameVar::Dynamic(DynamicVar::Location(Some(new_location.clone())))));
        // self.selection_panel_update_manager.borrow_mut().add_widget(WidgetType::VarSource(widget));

        self.next_id += 1;


        return new_location;
    }

    pub fn create_location_blank(&mut self) -> Rc<RefCell<WorldLocation>> {
        let name = self.next_id.to_string();

        // Add location to maps
        let new_location = Rc::new(RefCell::new(WorldLocation::new(name, WorldArea::new_blank(), self.next_id)));
        self.player_location_map.insert(self.next_id, new_location.clone());
        self.player_location_name_map.insert("blank".to_string(), self.next_id);
        self.all_play_locations_vec_ref.borrow_mut().push(new_location.clone());
        
        // let widget = VarSource::new(Var::Game(GameVar::Dynamic(DynamicVar::Loscation(Some(new_location.clone())))));
        // self.selection_panel_update_manager.borrow_mut().add_widget(WidgetType::VarSource(widget));

        self.next_id += 1;


        return new_location;
    }

    //=====================================
    // UI Updating
    //=====================================

    pub fn add_all_location_to_selection_manager(&mut self) {
        for (_key, _location) in self.player_location_map.iter() {
            // let widget = VarSource::new(Var::Game(GameVar::Dynamic(DynamicVar::Location(Some(location.clone())))));
            // self.selection_panel_update_manager.borrow_mut().add_widget(WidgetType::VarSource(widget));
        }
    }

    //=====================================
    // Location Getters
    //=====================================

    pub fn get_locations_ref_vec(&self) -> &Rc<RefCell<Vec<Rc<RefCell<WorldLocation>>>>> {
        return &self.all_play_locations_vec_ref;
    }

    pub fn name_to_id(&self, name: &str) -> Option<u32> {
        return self.player_location_name_map.get(name).cloned();
    }

    // Location
    pub fn get_all_locations(&self) -> Vec<Rc<RefCell<WorldLocation>>> {
        self.player_location_map.values().cloned().collect()
    }

    pub fn get_all_location_ids(&self) -> Vec<u32> {
        self.player_location_map.keys().copied().collect()
    }

    pub fn get_location_with_name(&self, name: &str) -> Option<Rc<RefCell<WorldLocation>>> {
        if let Some(id) = self.name_to_id(name) {
            return self.player_location_map.get(&id).cloned();
        }
        else {
            return None;
        }
    }
    pub fn get_mut_location_with_name(&mut self, name: &str) -> Option<Rc<RefCell<WorldLocation>>> {
        if let Some(id) = self.name_to_id(name) {
            return self.player_location_map.get_mut(&id).cloned();
        }
        else {
            return None;
        }
    }

    

}