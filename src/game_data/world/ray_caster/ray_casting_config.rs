use std::sync::{Arc, RwLock};

use crate::game_data::{World, locations::world_area::WorldArea, screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block_manager::LairBlockManager};

pub struct RayCastingConfig {
    pub lair_manager: LairBlockManager,
    pub world_area: WorldArea,
}

impl RayCastingConfig {
    pub fn new(lair_manager: LairBlockManager, world_area:& WorldArea) -> RayCastingConfig {
        RayCastingConfig {
            lair_manager: lair_manager, 
            world_area: world_area.clone()
        }
    }
}