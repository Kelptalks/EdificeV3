
static STRAIT_AXIS: [[i32; 3]; 6] = [
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
    [-1, 0, 0],
    [0, -1, 0],
    [0, 0, -1],
];

pub fn get_strait_directions() -> [[i32; 3]; 6] {
    STRAIT_AXIS
}

pub fn diff_cords(cords_1: [i32; 3], cords_2: [i32; 3]) -> [i32; 3] {
    [
        cords_1[0] - cords_2[0],
        cords_1[1] - cords_2[1],
        cords_1[2] - cords_2[2],
    ]
}

pub fn add_cords(cords_1: [i32; 3], cords_2: [i32; 3]) -> [i32; 3] {
    [
        cords_1[0] + cords_2[0],
        cords_1[1] + cords_2[1],
        cords_1[2] + cords_2[2],
    ]
}


pub fn mult_cords(cords_1: [i32; 3], cords_2: [i32; 3]) -> [i32; 3] {
    [
        cords_1[0] * cords_2[0],
        cords_1[1] * cords_2[1],
        cords_1[2] * cords_2[2],
    ]
}