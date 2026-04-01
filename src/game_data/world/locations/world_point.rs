#[derive(Clone, Copy)]
pub struct WorldPoint {
    pub cords: [i32; 3],
}

impl WorldPoint {
    pub fn new() -> WorldPoint {
        WorldPoint {
            cords: [0; 3],
        }
    }

    pub fn new_with_cords(cords: [i32; 3]) -> WorldPoint {
        WorldPoint {
            cords: cords,
        }
    }
}