use std::collections::HashMap;

use crate::game_data::player_data::nature_manager::animals::puff::puff::Puff;

pub struct PuffId {
    id: u32
}

pub struct PuffManager {
    current_id: u32,
    puff_map: HashMap<u32, Puff>
}

impl PuffManager {
    pub fn new() -> PuffManager {
        PuffManager {
            current_id: 0,
            puff_map: HashMap::new(),
        }
    }
}