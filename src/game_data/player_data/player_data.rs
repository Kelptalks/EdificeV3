use std::sync::{Arc, RwLock};

use crate::game_data::{World, log_init, player_data::{locations::location_manager::LocationManager, settings::settings_manager::SettingsManager}};


/*
################
## PlayerData ##
################
Manages data relating to the player that is used for gameplay and
rendering.
 
*/
pub struct PlayerData {
    // Location
    current_world: Arc<RwLock<World>>,


    // Drone Control data
    location_manager: LocationManager,
    settings: SettingsManager,
}

impl PlayerData {
    pub fn new(world: Arc<RwLock<World>>) -> PlayerData {
        log_init("Created Location Manager");
        PlayerData {
            current_world: world,

            location_manager: LocationManager::new(),

            settings: SettingsManager::new(),
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================
    pub fn get_world_ref(&self) -> Arc<RwLock<World>> {
        return self.current_world.clone();
    }

    pub fn get_mut_location_manager(&mut self) -> &mut LocationManager {
        return &mut self.location_manager;
    }




}