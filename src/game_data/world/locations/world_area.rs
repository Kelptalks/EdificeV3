use crate::game_data::{
    game_event_manager::{
        game_event_manager::GameEventManager,
        world_event_manager::world_event_manager::WorldEvent,
    },
    types::BlockTexture,
};

/*
###############
## WorldArea ##
###############
Represents an area in the world

Why :
includes functions for checking bounds


*/
#[derive(Clone, Copy)]
pub struct WorldArea {
    points: [[i32; 3]; 2],
}

impl WorldArea {
    pub fn new_blank() -> WorldArea {
        WorldArea {
            points: [[0, 0, 0], [0, 0, 0]]
        }
    }

    //=====================================
    // points
    //=====================================

    pub fn set_point_1(&mut self, world_cords: [i32; 3]) {
        self.points[0] = world_cords;
    }

    pub fn set_point_2(&mut self, world_cords: [i32; 3]) {
        self.points[1] = world_cords;
    }

    pub fn get_point_1(&self) -> [i32; 3] {
        return self.points[0];
    }

    pub fn get_point_2(&self) -> [i32; 3] {
        return self.points[1];
    }

    //=====================================
    // modification
    //=====================================

    pub fn shift_cords(&mut self, shift_cords: [i32; 3]) {
        for point in &mut self.points {
            for (i, axis) in point.iter_mut().enumerate() {
                *axis += shift_cords[i];
            }
        }
    }

    //=====================================
    // Getters
    //=====================================

    pub fn get_dimensions(&self) -> [i32; 3] {
        let [start, end] = self.points;
        [
            (end[0] - start[0]).abs(),
            (end[1] - start[1]).abs(),
            (end[2] - start[2]).abs(),
        ]
    }

    pub fn get_half_dimensions(&self) -> [i32; 3] {
        return self.get_dimensions().map(|d| d / 2);
    }

    pub fn get_center_world_cords(&self) -> [i32; 3] {
        let [start, _] = self.points;
        let half = self.get_half_dimensions();
        [
            start[0] + half[0],
            start[1] + half[1],
            start[2] + half[2],
        ]
    }

    //=====================================
    // Checks
    //=====================================

    pub fn cords_in_area(&self, cords: [i32; 3]) -> bool {
        let mut min = [i32::MAX; 3];
        let mut max = [i32::MIN; 3];

        // Get min and max
        for point in &self.points {
            for i in 0..cords.len() {
                if point[i] < min[i] { min[i] = point[i]; }
                if point[i] > max[i] { max[i] = point[i]; }
            }
        }

        let in_range = true;

        for i in 0..3 {
            if cords[i] < min[i] || cords[i] > max[i] {
                return false;
            }
        }

        return in_range;
    }

    pub fn cords_on_border(&self, cords: [i32; 3]) -> bool {
        let mut min = [i32::MAX; 3];
        let mut max = [i32::MIN; 3];

        for point in &self.points {
            for i in 0..3 {
                if point[i] < min[i] { min[i] = point[i]; }
                if point[i] > max[i] { max[i] = point[i]; }
            }
        }

        // Must be inside the area first
        for i in 0..3 {
            if cords[i] < min[i] || cords[i] > max[i] {
                return false;
            }
        }

        // True if touching at least one edge on any axis
        for i in 0..3 {
            if cords[i] == min[i] || cords[i] == max[i] {
                return true;
            }
        }

        false
    }

    pub fn cords_on_corner(&self, cords: [i32; 3]) -> bool {
        let mut min = [i32::MAX; 3];
        let mut max = [i32::MIN; 3];

        for point in &self.points {
            for i in 0..3 {
                if point[i] < min[i] { min[i] = point[i]; }
                if point[i] > max[i] { max[i] = point[i]; }
            }
        }

        // Must be at min or max on ALL axes simultaneously
        for i in 0..3 {
            if cords[i] != min[i] && cords[i] != max[i] {
                return false;
            }
        }

        true
    }

    pub fn cords_on_edge(&self, cords: [i32; 3]) -> bool {
        let mut min = [i32::MAX; 3];
        let mut max = [i32::MIN; 3];

        for point in &self.points {
            for i in 0..3 {
                if point[i] < min[i] { min[i] = point[i]; }
                if point[i] > max[i] { max[i] = point[i]; }
            }
        }

        let mut on_edge_count = 0;

        for i in 0..3 {
            // Out of bounds entirely
            if cords[i] < min[i] || cords[i] > max[i] {
                return false;
            }
            if cords[i] == min[i] || cords[i] == max[i] {
                on_edge_count += 1;
            }
        }

        // Exactly 2 axes on an edge = edge, not corner (3) or face (1)
        on_edge_count == 2
    }

    //=====================================
    // Modification
    //=====================================

    pub fn fill_area(&self, game_event_manager: &mut GameEventManager, block_texture: BlockTexture) {
        let mut min = [i32::MAX; 3];
        let mut max = [i32::MIN; 3];

        for point in &self.points {
            for i in 0..3 {
                if point[i] < min[i] { min[i] = point[i]; }
                if point[i] > max[i] { max[i] = point[i]; }
            }
        }

        for x in min[0]..=max[0] {
            for y in min[1]..=max[1] {
                for z in min[2]..=max[2] {
                    game_event_manager.add_world_event(WorldEvent::ModBlock([x, y, z], block_texture));
                }
            }
        }
    }

}