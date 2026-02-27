use crate::game_data::{log_init, player_data::locations::location_manager::LocationManager};


/*
################
## PlayerData ##
################
Manages data relating to the player that is used for gameplay and
rendering.
 
*/
pub struct PlayerData {
    location_manager: LocationManager,
}

impl PlayerData {
    pub fn new() -> PlayerData {
        log_init("Created Location Manager");
        PlayerData {
            location_manager: LocationManager::new(),
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================
    
    pub fn get_mut_location_manager(&mut self) -> &mut LocationManager {
        return &mut self.location_manager;
    }




}