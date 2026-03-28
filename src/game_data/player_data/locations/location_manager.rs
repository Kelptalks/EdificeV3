use std::{cell::RefCell, collections::HashMap, panic::Location, rc::Rc};

use crate::game_data::{game_event_manager::{event_manager, prelude::Event}, locations::world_area::WorldArea, player_data::{drone_programming::var::{game_vars::game_var_type::GameVar, var_type::Var}, locations::location::WorldLocation}, screen::widget::{drone_programming::vars::var_source::VarSource, selection_panel::{self, selection_panel::SelectionPanel, selection_panel_config::WidgetUpdateManager}, widget::WidgetType}};


/*
#####################
## LocationManager ##
#####################
Managers the retrival and creation of new locations

*/
pub struct LocationManager {
    player_location_map: HashMap<u32, Rc<RefCell<WorldLocation>>>,
    player_location_name_map: HashMap<String, u32>,
    next_id: u32,



    selection_panel_update_manager: Rc<RefCell<WidgetUpdateManager>>,
}

impl LocationManager {

    //=====================================
    // Init
    //=====================================

    pub fn new() -> LocationManager {
        LocationManager {
            player_location_map: HashMap::new(),
            player_location_name_map: HashMap::new(),
            next_id: 0,

            selection_panel_update_manager: WidgetUpdateManager::new(), // Used for updating UI with new drones created
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

        let widget = VarSource::new(Var::Game(GameVar::Location(Some(new_location.clone()))));
        self.selection_panel_update_manager.borrow_mut().add_widget(WidgetType::VarSource(widget));

        self.next_id += 1;


        return new_location;
    }

    //=====================================
    // UI Updating
    //=====================================

    pub fn get_panel_update_manager(&self) -> &Rc<RefCell<WidgetUpdateManager>> {
        return &self.selection_panel_update_manager;
    }

    //=====================================
    // Location Getters
    //=====================================

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