use crate::game_data::player_data::nature_manager::animals::puff::puff_manager::{PuffId, PuffManager};




pub enum AnimalObject {
    Puff(PuffId)
}

pub struct AnimalMangager {
    puff_manager: PuffManager,
}

impl AnimalMangager {
    pub fn new() -> AnimalMangager {
        AnimalMangager {
            puff_manager: PuffManager::new(),
        }
    }
}


pub enum NatureObject {
    Animal(AnimalObject)
}

pub struct NatureManager {
    animal_manager: AnimalMangager,
}

impl NatureManager {
    pub fn new() -> NatureManager {
        NatureManager {
            animal_manager: AnimalMangager::new()
        }
    }

    
}