use rand::seq::{IndexedRandom, SliceRandom};

pub enum AxisDirection {
    Up,
    Down,
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl AxisDirection {
    pub fn get_cords(&self) -> [i32; 3] {
        match self {
            AxisDirection::Up        => [ 0,  0,  1],
            AxisDirection::Down      => [ 0,  0, -1],
            AxisDirection::North     => [ 0,  1,  0],
            AxisDirection::South     => [ 0, -1,  0],
            AxisDirection::East      => [ 1,  0,  0],
            AxisDirection::West      => [-1,  0,  0],
            AxisDirection::NorthEast => [ 1,  1,  0],
            AxisDirection::NorthWest => [-1,  1,  0],
            AxisDirection::SouthEast => [ 1, -1,  0],
            AxisDirection::SouthWest => [-1, -1,  0],
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            AxisDirection::Up        => "Up",
            AxisDirection::Down      => "Down",
            AxisDirection::North     => "North",
            AxisDirection::South     => "South",
            AxisDirection::East      => "East",
            AxisDirection::West      => "West",
            AxisDirection::NorthEast => "NorthEast",
            AxisDirection::NorthWest => "NorthWest",
            AxisDirection::SouthEast => "SouthEast",
            AxisDirection::SouthWest => "SouthWest",
        }
    }

    /// Returns the closest direction to the given impulse vector.
    /// Checks vertical dominance first; otherwise quantizes to one of 8 horizontal directions.
    pub fn from_impulse(impulse: [f32; 3]) -> AxisDirection {
        let [x, y, z] = impulse;
        let ax = x.abs();
        let ay = y.abs();
        let az = z.abs();


        // Cardinal if one horizontal axis is more than 2x the other, otherwise diagonal
        if ax > ay * 2.0 {
            if x >= 0.0 { AxisDirection::West  } else { AxisDirection::East  }
        } else if ay > ax * 2.0 {
            if y >= 0.0 { AxisDirection::South } else { AxisDirection::North }
        } else {
            match (x >= 0.0, y >= 0.0) {
                (true,  true)  => AxisDirection::NorthEast,
                (false, true)  => AxisDirection::NorthWest,
                (true,  false) => AxisDirection::SouthEast,
                (false, false) => AxisDirection::SouthWest,
            }
        }
    }

    pub fn all() -> [AxisDirection; 10] {
        [
            AxisDirection::Up,
            AxisDirection::Down,
            AxisDirection::North,
            AxisDirection::South,
            AxisDirection::East,
            AxisDirection::West,
            AxisDirection::NorthEast,
            AxisDirection::NorthWest,
            AxisDirection::SouthEast,
            AxisDirection::SouthWest,
        ]
    }

    pub fn random() -> AxisDirection {
        let idx = rand::random::<u32>() % 10;
        match idx {
            0 => AxisDirection::Up,
            1 => AxisDirection::Down,
            2 => AxisDirection::North,
            3 => AxisDirection::South,
            4 => AxisDirection::East,
            5 => AxisDirection::West,
            6 => AxisDirection::NorthEast,
            7 => AxisDirection::NorthWest,
            8 => AxisDirection::SouthEast,
            _ => AxisDirection::SouthWest,
        }
    }
}
