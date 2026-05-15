use crate::game_data::{World, player_data::game_entity::components::{entity_components::{EntityComponent, EntityComponentEvent}, pos_component::PosComponent}, tik_manager::game_time::GameTime};

#[derive(Clone)]
pub struct LocomotionComponent {
    pub velocity: [f32; 3],
    pub gravity: f32,
    pub friction: f32,
}

impl LocomotionComponent {
    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::Locomotion(self)
    }

    pub fn new(gravity: f32, friction: f32) -> LocomotionComponent {
        LocomotionComponent {
            velocity: [0.0; 3],
            gravity,
            friction,
        }
    }

    /// Advances physics and applies the resulting delta to `pos`.
    /// Cancels velocity if the destination is solid.
    pub fn tik(&mut self, _time: &GameTime, pos: &mut PosComponent, world: &World) {
        self.velocity[0] *= 1.0 - self.friction;
        self.velocity[1] *= 1.0 - self.friction;
        self.velocity[2] -= self.gravity;
        self.velocity[2] *= 1.0 - self.friction;
        if !pos.apply_delta(self.velocity, world) {
            self.velocity = [0.0; 3];
        }
    }

    pub fn apply_impulse(&mut self, impulse: [f32; 3]) {
        self.velocity[0] += impulse[0];
        self.velocity[1] += impulse[1];
        self.velocity[2] += impulse[2];
    }
}



#[derive(Clone)]
pub enum LocomotionComponentEvent {
    Accelerate([f32; 3]),
    SetVelocity([f32; 3]),
}

impl LocomotionComponentEvent {
    pub fn wrap_into_component_event(self) -> EntityComponentEvent {
        EntityComponentEvent::Locomotion(self)
    }

    pub fn execute(self, locomotion: &mut LocomotionComponent) {
        match self {
            LocomotionComponentEvent::Accelerate(impulse) => {
                locomotion.apply_impulse(impulse);
            },
            LocomotionComponentEvent::SetVelocity(vel) => {
                locomotion.velocity = vel;
            },
        }
    }
}
