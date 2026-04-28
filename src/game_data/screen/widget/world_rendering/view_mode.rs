use crate::game_data::player_data::{drones::drone_manager::DroneId, locations::location_manager::LocationId};


#[derive(Clone, Copy)]
pub enum ViewMode {
    Start(),

    Drone(DroneId),
    Location(LocationId),
}




