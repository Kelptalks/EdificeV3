use crate::game_data::{
    game_event_manager::{
        game_event_manager::EventManager,
        world_event_manager::world_event_manager::WorldEvent,
    }, locations::world_point::WorldPoint, types::BlockTexture
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
    points: [WorldPoint; 2],
}

impl WorldArea {
    pub fn new_blank() -> WorldArea {
        WorldArea {
            points: [WorldPoint::new(), WorldPoint::new()]
        }
    }

    //=====================================
    // points
    //=====================================
    pub fn set_point(&mut self, index: usize, new_point: WorldPoint) {
        self.points[index] = new_point;
    }

    pub fn set_point_1(&mut self, cords: [i32; 3]) {
        self.points[0] = WorldPoint::new_with_cords(cords);
    }

    pub fn set_point_2_cords(&mut self, cords: [i32; 3]) {
        self.points[1] = WorldPoint::new_with_cords(cords);
    }

    pub fn get_point_1_cords(&self) -> [i32; 3] {
        return self.points[0].cords;
    }

    pub fn get_point_2_cords(&self) -> [i32; 3] {
        return self.points[1].cords;
    }

    pub fn get_world_point(&self, index: usize) -> WorldPoint{
        return self.points[index];
    }

    pub fn shift_point(&mut self, point_index: usize, shift_cords: [i32; 3]) {
        for (i, axis) in self.points[point_index].cords.iter_mut().enumerate() {
            *axis += shift_cords[i];
        }
    }

    // Ensures point 1 always holds the lesser coordinate on every axis and
    // point 2 holds the greater, by selecting the real corners of the box.
    pub fn normalize_points(&mut self) {
        for i in 0..3 {
            if self.points[0].cords[i] > self.points[1].cords[i] {
                let tmp = self.points[0].cords[i];
                self.points[0].cords[i] = self.points[1].cords[i];
                self.points[1].cords[i] = tmp;
            }
        }
    }

    //=====================================
    // Entire Area
    //=====================================

    pub fn shift_cords(&mut self, shift_cords: [i32; 3]) {
        for point in &mut self.points {
            for (i, axis) in point.cords.iter_mut().enumerate() {
                *axis += shift_cords[i];
            }
        }
    }

    pub fn resize(&mut self, mod_scale: [i32; 3], expand: bool) {
        for i in 0..3 {
            if self.points[0].cords[i] < self.points[1].cords[i] {
                self.points[0].cords[i] += if expand { -mod_scale[i] } else { mod_scale[i] };
                self.points[1].cords[i] += if expand { mod_scale[i] } else { -mod_scale[i] };
            } else {
                self.points[1].cords[i] += if expand { -mod_scale[i] } else { mod_scale[i] };
                self.points[0].cords[i] += if expand { mod_scale[i] } else { -mod_scale[i] };
            }
        }
    }

    //=====================================
    // Getters
    //=====================================


    pub fn get_dimensions(&self) -> [i32; 3] {
        let [start, end] = self.points;
        [
            (end.cords[0] - start.cords[0]).abs(),
            (end.cords[1] - start.cords[1]).abs(),
            (end.cords[2] - start.cords[2]).abs(),
        ]
    }

    pub fn get_half_dimensions(&self) -> [i32; 3] {
        return self.get_dimensions().map(|d| d / 2);
    }

    pub fn get_largest_dimension_scale(&self) -> i32 {
        self.get_dimensions().iter().max().copied().unwrap()
    }

    pub fn get_center_world_cords(&self) -> [i32; 3] {
        let [start, end] = self.points;
        [
            start.cords[0] + (end.cords[0] - start.cords[0]) / 2,
            start.cords[1] + (end.cords[1] - start.cords[1]) / 2,
            start.cords[2] + (end.cords[2] - start.cords[2]) / 2,
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
                if point.cords[i] < min[i] { min[i] = point.cords[i]; }
                if point.cords[i] > max[i] { max[i] = point.cords[i]; }
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
                if point.cords[i] < min[i] { min[i] = point.cords[i]; }
                if point.cords[i] > max[i] { max[i] = point.cords[i]; }
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
                if point.cords[i] < min[i] { min[i] = point.cords[i]; }
                if point.cords[i] > max[i] { max[i] = point.cords[i]; }
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
                if point.cords[i] < min[i] { min[i] = point.cords[i]; }
                if point.cords[i] > max[i] { max[i] = point.cords[i]; }
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

    pub fn fill_area(&self, game_event_manager: &mut EventManager, block_texture: BlockTexture) {
        let mut min = [i32::MAX; 3];
        let mut max = [i32::MIN; 3];

        for point in &self.points {
            for i in 0..3 {
                if point.cords[i] < min[i] { min[i] = point.cords[i]; }
                if point.cords[i] > max[i] { max[i] = point.cords[i]; }
            }
        }

        for x in min[0]..=max[0] {
            for y in min[1]..=max[1] {
                for z in min[2]..=max[2] {
                    game_event_manager.add_world_event(WorldEvent::PlaceBlock([x, y, z], block_texture));
                }
            }
        }
    }

    pub fn get_fill_area_events(&self, block_texture: BlockTexture) -> Vec<WorldEvent> {
        let mut min = [i32::MAX; 3];
        let mut max = [i32::MIN; 3];

        for point in &self.points {
            for i in 0..3 {
                if point.cords[i] < min[i] { min[i] = point.cords[i]; }
                if point.cords[i] > max[i] { max[i] = point.cords[i]; }
            }
        }

        let mut world_block_mod_events = Vec::new();

        for x in min[0]..=max[0] {
            for y in min[1]..=max[1] {
                for z in min[2]..=max[2] {
                    world_block_mod_events.push(WorldEvent::PlaceBlock([x, y, z], block_texture));
                }
            }
        }

        return world_block_mod_events;
    }

}
