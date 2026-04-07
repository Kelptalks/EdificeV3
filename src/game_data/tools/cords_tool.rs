
static STRAIT_AXIS: [[i32; 3]; 6] = [
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
    [-1, 0, 0],
    [0, -1, 0],
    [0, 0, -1],
];

pub fn get_strait_axis() -> [[i32; 3]; 6] {
    STRAIT_AXIS
}