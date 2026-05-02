use crate::game_data::{World, game_event_manager::prelude::Event, player_data::{nature_manager::nature_manager::NatureObject, player_data::PlayerData}};

pub enum GameObjectType {
    Nature(NatureObject),
}


trait GameObject {
    const TIK_INTERVAL: u64;
    

    //=====================================
    // Tik Execution
    //=====================================

    fn tik(&mut self, time: u64, world: &World, player_data: &PlayerData) -> Option<Vec<Event>> {
        if time % Self::TIK_INTERVAL == 0 {
            Some(self.execute_tik(world, player_data))
        }
        else {
            None
        }
    }

    fn execute_tik(&mut self, world: &World, player_data: &PlayerData) -> Vec<Event>;


    //=====================================
    // Location
    //=====================================


}