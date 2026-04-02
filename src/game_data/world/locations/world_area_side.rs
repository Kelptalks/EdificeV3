use crate::game_data::locations::{world_area::WorldArea, world_point::WorldPoint};

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

    pub fn to_string(&self) -> String {
        match self {
            WorldAreaSide::XPlus => "Xplus".to_string(),
            WorldAreaSide::XMinus => "XMinus".to_string(),
            WorldAreaSide::YPlus => "YPlus".to_string(),
            WorldAreaSide::YMinus => "YMinus".to_string(),
            WorldAreaSide::ZPlus => "ZPlus".to_string(),
            WorldAreaSide::ZMinus => "ZMinus".to_string(),
        }
    }

    // Returns which sides of the area the point lies on.
    // A point can be on multiple sides at once (edges touch 2, corners touch 3).
    // Returns empty if the point is not on any face.
    pub fn get_sides_of_point_in_area(area: &WorldArea, point: &WorldPoint) -> Vec<WorldAreaSide> {
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

        if c[0] <= min[0] { sides.push(WorldAreaSide::XMinus); }
        if c[0] >= max[0] { sides.push(WorldAreaSide::XPlus); }
        if c[1] <= min[1] { sides.push(WorldAreaSide::YMinus); }
        if c[1] >= max[1] { sides.push(WorldAreaSide::YPlus); }
        if c[2] <= min[2] { sides.push(WorldAreaSide::ZMinus); }
        if c[2] >= max[2] { sides.push(WorldAreaSide::ZPlus); }

        return sides;
    }

    // Returns the distance from a given side of the area to the provided coordinates.
    // Uses cords_in_area to clamp the coords, then measures along the relevant axis.
    pub fn dist_from_side(&self, area: &WorldArea, cords: [i32; 3]) -> i32 {
        let p1 = area.get_point_1_cords();
        let p2 = area.get_point_2_cords();
        
        let mut min = [0i32; 3];
        let mut max = [0i32; 3];
        for i in 0..3 {
            min[i] = p1[i].min(p2[i]);
            max[i] = p1[i].max(p2[i]);
        }

        match self {
            WorldAreaSide::XPlus  => max[0] - cords[0],
            WorldAreaSide::XMinus => cords[0] - min[0],
            WorldAreaSide::YPlus  => max[1] - cords[1],
            WorldAreaSide::YMinus => cords[1] - min[1],
            WorldAreaSide::ZPlus  => max[2] - cords[2],
            WorldAreaSide::ZMinus => cords[2] - min[2],
        }
    }


    // Get modifying values to expand the side of an area
    pub fn get_area_shift_mod(&self, amount: i32) -> [i32; 3] {
        match self  {
            WorldAreaSide::XPlus => [amount, 0, 0],
            WorldAreaSide::XMinus => [-amount, 0, 0],
            WorldAreaSide::YPlus => [0, amount, 0],
            WorldAreaSide::YMinus => [0, -amount, 0],
            WorldAreaSide::ZPlus => [0, 0, amount],
            WorldAreaSide::ZMinus => [0, 0, -amount],

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
    pub fn get_area_point_sizing_mods(&self) -> [WorldPoint; 2] {
        match self  {
            WorldAreaSide::XPlus => return [
                WorldPoint::new_with_cords([1, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],

            WorldAreaSide::XMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([-1, 0, 0])],
            
            WorldAreaSide::YPlus => return [
                WorldPoint::new_with_cords([0, 1, 0]), 
                WorldPoint::new_with_cords([0, 0, 0])],
            
            WorldAreaSide::YMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, -1, 0])],
            
            WorldAreaSide::ZPlus => return [
                WorldPoint::new_with_cords([0, 0, 1]), 
                WorldPoint::new_with_cords([0, 0, 0])],

            WorldAreaSide::ZMinus => return [
                WorldPoint::new_with_cords([0, 0, 0]), 
                WorldPoint::new_with_cords([0, 0, -1])],
        }
    }

    //=====================================
    // RayCasting
    //=====================================


    // Get the faces of a side based off it's scale
    pub fn get_face_origins(&self, area: &WorldArea) -> Vec<[i32; 3]> {
        let max = area.get_max_point().cords;
        let min = area.get_min_point().cords;

        match self {
            // Fixed X, iterate Z and Y
            WorldAreaSide::XPlus => (min[1]..=max[1]).flat_map(|z|
                (min[2]..=max[2]).map(move |y| [max[0], z, y])
            ).collect(),

            WorldAreaSide::XMinus => (min[1]..=max[1]).flat_map(|z|
                (min[2]..=max[2]).map(move |y| [min[0], z, y])
            ).collect(),

            // Fixed Z, iterate X and Y
            WorldAreaSide::ZPlus => (min[0]..=max[0]).flat_map(|x|
                (min[2]..=max[2]).map(move |y| [x, max[1], y])
            ).collect(),

            WorldAreaSide::ZMinus => (min[0]..=max[0]).flat_map(|x|
                (min[2]..=max[2]).map(move |y| [x, min[1], y])
            ).collect(),

            // Fixed Y, iterate X and Z
            WorldAreaSide::YPlus => (min[0]..=max[0]).flat_map(|x|
                (min[1]..=max[1]).map(move |z| [x, z, max[2]])
            ).collect(),

            WorldAreaSide::YMinus => (min[0]..=max[0]).flat_map(|x|
                (min[1]..=max[1]).map(move |z| [x, z, min[2]])
            ).collect(),
        }
    }

    pub fn get_face_origins_local(&self, area: &WorldArea) -> Vec<[i32; 3]> {
        let max = area.get_max_point().cords;
        let min = area.get_min_point().cords;
        let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];

        match self {
            WorldAreaSide::XPlus => (0..=size[1]).flat_map(|z|
                (0..=size[2]).map(move |y| [size[0], z, y])
            ).collect(),

            WorldAreaSide::XMinus => (0..=size[1]).flat_map(|z|
                (0..=size[2]).map(move |y| [0, z, y])
            ).collect(),

            WorldAreaSide::ZPlus => (0..=size[0]).flat_map(|x|
                (0..=size[2]).map(move |y| [x, size[1], y])
            ).collect(),

            WorldAreaSide::ZMinus => (0..=size[0]).flat_map(|x|
                (0..=size[2]).map(move |y| [x, 0, y])
            ).collect(),

            WorldAreaSide::YPlus => (0..=size[0]).flat_map(|x|
                (0..=size[1]).map(move |z| [x, z, size[2]])
            ).collect(),

            WorldAreaSide::YMinus => (0..=size[0]).flat_map(|x|
                (0..=size[1]).map(move |z| [x, z, 0])
            ).collect(),
        }
    }

    pub fn get_face_origins_paired(&self, area: &WorldArea) -> Vec<([i32; 3], [i32; 3])> {
        let max = area.get_max_point().cords;
        let min = area.get_min_point().cords;
        let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];

        match self {
            WorldAreaSide::XPlus => (min[1]..=max[1]).flat_map(|z|
                (min[2]..=max[2]).map(move |y| ([max[0], z, y], [size[0], z - min[1], y - min[2]]))
            ).collect(),

            WorldAreaSide::XMinus => (min[1]..=max[1]).flat_map(|z|
                (min[2]..=max[2]).map(move |y| ([min[0], z, y], [0, z - min[1], y - min[2]]))
            ).collect(),

            WorldAreaSide::ZPlus => (min[0]..=max[0]).flat_map(|x|
                (min[2]..=max[2]).map(move |y| ([x, max[1], y], [x - min[0], size[1], y - min[2]]))
            ).collect(),

            WorldAreaSide::ZMinus => (min[0]..=max[0]).flat_map(|x|
                (min[2]..=max[2]).map(move |y| ([x, min[1], y], [x - min[0], 0, y - min[2]]))
            ).collect(),

            WorldAreaSide::YPlus => (min[0]..=max[0]).flat_map(|x|
                (min[1]..=max[1]).map(move |z| ([x, z, max[2]], [x - min[0], z - min[1], size[2]]))
            ).collect(),

            WorldAreaSide::YMinus => (min[0]..=max[0]).flat_map(|x|
                (min[1]..=max[1]).map(move |z| ([x, z, min[2]], [x - min[0], z - min[1], 0]))
            ).collect(),
        }
    }

}
