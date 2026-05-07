use std::sync::{Arc, RwLock};

use crate::game_data::{World, log_init, player_data::{cursor::{cursor::Cursor, cursor_event_scheduler::CursorEventScheduler}, drones::{drone_event_scheduler::DroneEventScheduler, drone_manager::{DroneId, DroneManager}}, locations::location_manager::{LocationId, LocationManager}, nature_manager::nature_manager::NatureManager, progress_manager::progress_manager::ProgressManager, settings::settings_manager::SettingsManager}, screen::{menu_constructors::play_view_menu::new_play_view::PlayViewMode, widget::world_rendering::view_mode::ViewMode}, types::BlockTexture, world_gen::WorldGenManager};


/*
################
## PlayerData ##
################
Manages data relating to the player that is used for gameplay and
rendering.
 
*/
pub struct PlayerData {
    // View
    view_mode: Option<ViewMode>,

    // World
    pub current_world: Arc<RwLock<World>>,
    pub world_gen: WorldGenManager,

    // Game Object Managment
    cursor_manager: Cursor,
    drone_manager: DroneManager,
    nature_manager: NatureManager,
    location_manager: LocationManager,



    progress_manager: ProgressManager,

    // Settings
    settings: SettingsManager,
}

impl PlayerData {
    pub fn new(world: Arc<RwLock<World>>) -> PlayerData {
        log_init("Created Location Manager");
        PlayerData {
            // Player location
            view_mode: Some(ViewMode::Start()),
            current_world: world,
            world_gen: WorldGenManager::new(),

            // Game object managment
            cursor_manager: Cursor::new(),
            drone_manager: DroneManager::new(),
            nature_manager: NatureManager::new(),
            location_manager: LocationManager::new(),
            
            progress_manager: ProgressManager::new(),

            settings: SettingsManager::new(),
        }
    }


    //=====================================
    // Mutible Getters for execution
    //=====================================

    pub fn set_view_mode(&mut self, mode: &Option<ViewMode>){
        self.view_mode = mode.clone();
        if let Some(mode) = self.view_mode {
            match mode {
                ViewMode::Start() => {
                    self.cursor_manager.set_ghost_block(BlockTexture::DroneBotRight)
                },
                ViewMode::Drone(drone_id) => {
                    self.cursor_manager.set_ghost_block(BlockTexture::Air)
                },
                ViewMode::Location(location_id) => {
                    self.cursor_manager.set_ghost_block(BlockTexture::Air)
                },
            }
        }
        else {
            self.cursor_manager.set_ghost_block(BlockTexture::Air);
        }
    }

    pub fn get_mut_cursor(&mut self) -> &mut Cursor {
        &mut self.cursor_manager
    }

    pub fn get_mut_drone_manager(&mut self) -> &mut DroneManager {
        return &mut self.drone_manager;
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_cursor(&self) -> &Cursor {
        &self.cursor_manager
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

    //=================================================
    // Game Object Scedulers
    //=================================================

    pub fn get_cursor_event_scheduler(&self) -> CursorEventScheduler {
        return CursorEventScheduler::new(self.cursor_manager.clone());
    }

    pub fn get_drone_event_scheduler(&self, drone_id: DroneId) -> Option<DroneEventScheduler> {
        if let Some(drone_clone) = self.get_drone_manager().clone_drone_with_id(drone_id) {
            Some(DroneEventScheduler::new(drone_clone))
        }
        else {
            None
        }
    }

    //=================================================
    // Game Object Scedulers
    //=================================================

    pub fn get_view_mode(&self) -> Option<ViewMode> {
        return self.view_mode.clone();
    }

}