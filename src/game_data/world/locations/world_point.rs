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

    pub fn add_to_point(&mut self, mod_point: WorldPoint) {
        self.cords[0] += mod_point.cords[0];
        self.cords[1] += mod_point.cords[1];
        self.cords[2] += mod_point.cords[2];
    }

    pub fn sub_from_point(&mut self,  mod_point: WorldPoint) {
        self.cords[0] -= mod_point.cords[0];
        self.cords[1] -= mod_point.cords[1];
        self.cords[2] -= mod_point.cords[2];
    }
}