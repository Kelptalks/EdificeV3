use crate::game_data::screen::widget::world_rendering::area_rendering_manager::ray_caster::casted_triangle::CastedTriangle;

/// Per-frame, world-derived context handed to a spectated game entity's
/// `play_view`. Holds world-specific information only — no screen/input state.
pub struct WorldViewData {
    pub mouse_triangle: Option<CastedTriangle>,
}

impl WorldViewData {
    pub fn new(mouse_triangle: Option<CastedTriangle>) -> WorldViewData {
        WorldViewData { mouse_triangle }
    }
}
