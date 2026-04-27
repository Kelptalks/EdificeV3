use std::sync::{Arc, RwLock};

use crate::game_data::{World, log_init, player_data::{cursor::{cursor::Cursor, cursor_event_scheduler::CursorEventScheduler}, drones::{drone_event_scheduler::DroneEventScheduler, drone_manager::{DroneId, DroneManager}}, locations::location_manager::{LocationId, LocationManager}, settings::settings_manager::SettingsManager}};


/*
################
## PlayerData ##
################
Manages data relating to the player that is used for gameplay and
rendering.
 
*/
pub struct PlayerData {
    // Activity
    cursor: Cursor,
    focused_object: Option<GameObject>,

    // Location
    current_world: Arc<RwLock<World>>,

    // Game Object Managment
    drone_manager: DroneManager,
    location_manager: LocationManager,

    // Settings
    settings: SettingsManager,
}

impl PlayerData {
    pub fn new(world: Arc<RwLock<World>>) -> PlayerData {
        log_init("Created Location Manager");
        PlayerData {
            // Player location
            focused_object: None,
            cursor: Cursor::new(),
            current_world: world,

            // Game object managment
            drone_manager: DroneManager::new(),
            location_manager: LocationManager::new(),
            settings: SettingsManager::new(),
        }
    }


    //=====================================
    // Mutible Getters for execution
    //=====================================

    pub fn get_mut_cursor(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn get_mut_drone_manager(&mut self) -> &mut DroneManager {
        return &mut self.drone_manager;
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn get_world_ref(&self) -> Arc<RwLock<World>> {
        return self.current_world.clone();
    }

    pub fn get_mut_location_manager(&mut self) -> &mut LocationManager {
        return &mut self.location_manager;
    }

    pub fn get_drone_manager(&self) -> &DroneManager {
        return &self.drone_manager
    }



    pub fn get_cursor_event_scheduler(&self) -> CursorEventScheduler {
        return CursorEventScheduler::new(self.cursor.clone());
    }

    pub fn get_drone_event_scheduler(&self, drone_id: DroneId) -> Option<DroneEventScheduler> {
        if let Some(drone_clone) = self.get_drone_manager().clone_drone_with_id(drone_id) {
            Some(DroneEventScheduler::new(drone_clone))
        }
        else {
            None
        }
    }


    

    pub fn get_focused_game_object(&self) -> Option<GameObject> {
        return self.focused_object.clone();
    }

}


#[derive(Clone, Copy)]
pub enum GameObject {
    Drone(DroneId),
    Location(LocationId)
}

impl GameObject {

}