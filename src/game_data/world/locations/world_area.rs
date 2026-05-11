use crate::game_data::{
    game_event_manager::{
        game_event_manager::EventManager,
        world_event_manager::world_event_manager::WorldEvent,
    }, locations::{world_area_side::WorldAreaSide, world_point::WorldPoint}, types::BlockTexture
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

    pub fn new_with_cords(cords: [[i32; 3]; 2]) -> WorldArea {
        WorldArea {points: [WorldPoint::new_with_cords(cords[0]), WorldPoint::new_with_cords(cords[1])] }
    }

    //=====================================
    // points
    //=====================================
    pub fn set_point(&mut self, index: usize, new_point: WorldPoint) {
        self.points[index] = new_point;
    }

    pub fn set_point_1_cords(&mut self, cords: [i32; 3]) {
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

    pub fn get_max_point(&self) -> WorldPoint {
        let p1 = self.get_point_1_cords();
        let p2 = self.get_point_2_cords();
        let mut max = [0i32; 3];
        for i in 0..3 {
            max[i] = p1[i].max(p2[i]);
        }

        return WorldPoint { cords: max }
    }

    pub fn get_min_point(&self) -> WorldPoint {
        let p1 = self.get_point_1_cords();
        let p2 = self.get_point_2_cords();
        let mut min = [0i32; 3];
        for i in 0..3 {
            min[i] = p1[i].min(p2[i]);
        }

        return WorldPoint { cords: min }
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

    pub fn expand(&mut self, side: &WorldAreaSide) {
        self.normalize_points();
        let mod_points = side.get_area_point_sizing_mods();
        self.points[0].add_to_point(mod_points[0]);
        self.points[1].add_to_point(mod_points[1]);
    }

    pub fn expand_to_fit_point(&mut self, cords: [i32; 3]) {
        self.normalize_points();
        for i in 0..3 {
            if cords[i] < self.points[0].cords[i] {
                self.points[0].cords[i] = cords[i];
            } else if cords[i] > self.points[1].cords[i] {
                self.points[1].cords[i] = cords[i];
            }
        }
    }

    pub fn shrink(&mut self, side: &WorldAreaSide) {
        self.normalize_points();
        let mod_points = side.get_area_point_sizing_mods();
        self.points[0].sub_from_point(mod_points[0]);
        self.points[1].sub_from_point(mod_points[1]);
    }

    pub fn shrink_to_avoid_point(&mut self, cords: [i32; 3]) {
        self.normalize_points();
        if !self.cords_in_area(cords) {
            return;
        }
        // Find the axis where shrinking costs the least to exclude the point
        let shrink_from_min = [
            cords[0] - self.points[0].cords[0],
            cords[1] - self.points[0].cords[1],
            cords[2] - self.points[0].cords[2],
        ];
        let shrink_from_max = [
            self.points[1].cords[0] - cords[0],
            self.points[1].cords[1] - cords[1],
            self.points[1].cords[2] - cords[2],
        ];

        let mut best_axis = 0;
        let mut best_cost = i32::MAX;
        let mut from_min = true;

        for i in 0..3 {
            if shrink_from_min[i] < best_cost {
                best_cost = shrink_from_min[i];
                best_axis = i;
                from_min = true;
            }
            if shrink_from_max[i] < best_cost {
                best_cost = shrink_from_max[i];
                best_axis = i;
                from_min = false;
            }
        }

        if from_min {
            self.points[0].cords[best_axis] = cords[best_axis] + 1;
        } else {
            self.points[1].cords[best_axis] = cords[best_axis] - 1;
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
            start.cords[0] + ((end.cords[0] - start.cords[0]) / 2),
            start.cords[1] + ((end.cords[1] - start.cords[1]) / 2),
            start.cords[2] + ((end.cords[2] - start.cords[2]) / 2),
        ]
    }

    //=====================================
    // Checks
    //=====================================

    pub fn cords_in_area(&self, cords: [i32; 3]) -> bool {
        let min = self.get_min_point().cords;
        let max = self.get_max_point().cords;

        (0..3).all(|i| cords[i] >= min[i] && cords[i] <= max[i])
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
                    game_event_manager.add_world_event(WorldEvent::ModBlock([x, y, z], block_texture));
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
                    world_block_mod_events.push(WorldEvent::ModBlock([x, y, z], block_texture));
                }
            }
        }

        return world_block_mod_events;
    }

    

}
