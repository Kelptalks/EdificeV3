
use crate::game_data::{locations::world_area::WorldArea, screen::{renderer::camera_data::Direction, widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block_manager::LairBlockManager}};

pub struct RayCastingConfig {
    pub lair_manager: LairBlockManager,
    pub world_area: WorldArea,

    pub direction: [i32; 3],
    pub view_distance: u32,
    pub shadow_draw_distance: u32,
    pub camera_direction: Direction,
}

impl RayCastingConfig {
    pub fn new(lair_manager: LairBlockManager, world_area: &WorldArea, direction: [i32; 3], view_distance: u32) -> RayCastingConfig {
        RayCastingConfig {
            lair_manager,
            world_area: world_area.clone(),
            direction,
            view_distance,
            shadow_draw_distance: 50,
            camera_direction: Direction::North,
        }
    }
}