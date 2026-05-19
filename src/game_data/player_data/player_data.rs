#![allow(dead_code)]
use std::sync::{Arc, RwLock};

use crate::game_data::{World, game_event_manager::event_manager::EventManager, log_init, player_data::{cursor::{cursor::Cursor, cursor_event_scheduler::CursorEventScheduler}, drones::{drone_event_scheduler::DroneEventScheduler, drone_manager::{DroneId, DroneManager}}, game_entity::game_entity_manager::{GameEntity, GameEntityManager}, locations::location_manager::LocationManager, progress_manager::progress_manager::ProgressManager, settings::settings_manager::SettingsManager}, screen::widget::world_rendering::view_mode::ViewMode, tik_manager::game_time::GameTime, types::BlockTexture, world_gen::WorldGenManager};


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
    cursor: Cursor,
    drone_manager: DroneManager,
    location_manager: LocationManager,


    pub game_entity_manager: GameEntityManager,
    progress_manager: ProgressManager,

    // Settings
    settings: SettingsManager,
}

impl PlayerData {
    pub fn new(world: Arc<RwLock<World>>) -> PlayerData {
        log_init("Created Location Manager");
        PlayerData {
            // Player location
            view_mode: Some(ViewMode::God()),
            current_world: world,
            world_gen: WorldGenManager::new(),

            // Game object managment
            cursor: Cursor::new(),
            drone_manager: DroneManager::new(),
            location_manager: LocationManager::new(),
            
            game_entity_manager: GameEntityManager::new(),
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
                ViewMode::God() => {
                    self.cursor.set_ghost_block(BlockTexture::DroneBotRight)
                },
                ViewMode::GameObjectSpectate(_id) => {
                    self.cursor.set_ghost_block(BlockTexture::Air)
                },
                ViewMode::Location(_location_id) => {
                    self.cursor.set_ghost_block(BlockTexture::Air)
                },
            }
        }
        else {
            self.cursor.set_ghost_block(BlockTexture::Air);
        }
    }

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

    //=================================================
    // Game Object Scedulers
    //=================================================

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

    //=================================================
    // Game Object Scedulers
    //=================================================

    pub fn get_view_mode(&self) -> Option<ViewMode> {
        return self.view_mode.clone();
    }

    pub fn new_game_entity(&mut self, entity: GameEntity) {
        self.game_entity_manager.new_game_entity(entity);
    }

    pub fn tik_game_entities(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        self.game_entity_manager.tik_game_entities(time, world, event_manager);
        self.cursor.tik(world, event_manager);
    }

}