pub struct LocomotionComponent {
    pos: [f32; 3],
    velocity: [f32; 3],
    acceleration: [f32; 3],
}


impl LocomotionComponent {
    pub fn new(pos: [f32; 3]) -> LocomotionComponent {
        LocomotionComponent {
            pos,
            velocity: [0.0; 3],
            acceleration: [0.0; 3],
        }
    }
}