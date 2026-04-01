use crate::game_data::locations::{world_area::WorldArea, world_point::{self, WorldPoint}};

static ALL_WORLD_AREA_SIDES: [WorldAreaSide; 6] = [
    WorldAreaSide::XPlus,
    WorldAreaSide::XMinus,
    WorldAreaSide::YPlus,
    WorldAreaSide::YMinus,
    WorldAreaSide::ZPlus,
    WorldAreaSide::ZMinus,
];

#[derive(Clone, PartialEq)]
pub enum WorldAreaSide {
    XPlus,
    XMinus,
    YPlus,
    YMinus,
    ZPlus,
    ZMinus,
}


impl WorldAreaSide {
    pub fn all() -> &'static [WorldAreaSide; 6] {
        &ALL_WORLD_AREA_SIDES
    }

    // Returns which sides of the area the point lies on.
    // A point can be on multiple sides at once (edges touch 2, corners touch 3).
    // Returns empty if the point is not on any face.
    pub fn get_sides_of_point_in_area(area: &WorldArea, point: &WorldPoint) -> Vec<WorldAreaSide> {
        // Quick rejection: if the point isn't on any border face at all, skip the per-axis work.
        if !area.cords_on_border(point.cords) {
            return Vec::new();
        }

        // Derive the true min/max corners regardless of how the points were stored.
        let p1 = area.get_point_1_cords();
        let p2 = area.get_point_2_cords();
        let mut min = [0i32; 3];
        let mut max = [0i32; 3];
        for i in 0..3 {
            min[i] = p1[i].min(p2[i]);
            max[i] = p1[i].max(p2[i]);
        }

        let mut sides = Vec::new();
        let c = point.cords;

        if c[0] == min[0] { sides.push(WorldAreaSide::XMinus); }
        if c[0] == max[0] { sides.push(WorldAreaSide::XPlus); }
        if c[1] == min[1] { sides.push(WorldAreaSide::YMinus); }
        if c[1] == max[1] { sides.push(WorldAreaSide::YPlus); }
        if c[2] == min[2] { sides.push(WorldAreaSide::ZMinus); }
        if c[2] == max[2] { sides.push(WorldAreaSide::ZPlus); }

        return sides;
    }

    // Get modifying values to expand the side of an area
    pub fn get_area_shift_mod(&self) -> [i32; 3] {
        match self  {
            WorldAreaSide::XPlus => [1, 0, 0],
            WorldAreaSide::XMinus => [-1, 0, 0],
            WorldAreaSide::YPlus => [0, 1, 0],
            WorldAreaSide::YMinus => [0, -1, 0],
            WorldAreaSide::ZPlus => [0, 0, 1],
            WorldAreaSide::ZMinus => [0, 0, -1],

        }
    }

    pub fn area_shift_mod_to_side(shift_mod: [i32; 3]) -> Option<WorldAreaSide> {
        match shift_mod {
            [1, 0, 0]  => Some(WorldAreaSide::XPlus),
            [-1, 0, 0] => Some(WorldAreaSide::XMinus),
            [0, 1, 0]  => Some(WorldAreaSide::YPlus),
            [0, -1, 0] => Some(WorldAreaSide::YMinus),
            [0, 0, 1]  => Some(WorldAreaSide::ZPlus),
            [0, 0, -1] => Some(WorldAreaSide::ZMinus),
            _ => None,
        }
    }

    // Get modifying values to expand the side of an area
    pub fn get_area_point_expander_mods(&self) -> [WorldPoint; 2] {
        match self  {
            WorldAreaSide::XPlus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],

            WorldAreaSide::XMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
            
            WorldAreaSide::YPlus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
            
            WorldAreaSide::YMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
            
            WorldAreaSide::ZPlus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],

            WorldAreaSide::ZMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
        }
    }

    // Get modifying values to shrink the side of an area
    pub fn get_area_point_shrinking_mods(&self) -> [WorldPoint; 2] {
        match self  {
            WorldAreaSide::XPlus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],

            WorldAreaSide::XMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
            
            WorldAreaSide::YPlus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
            
            WorldAreaSide::YMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
            
            WorldAreaSide::ZPlus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],

            WorldAreaSide::ZMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
        }
    }
}
