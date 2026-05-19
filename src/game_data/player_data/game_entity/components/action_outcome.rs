use crate::game_data::player_data::drones::drone_actions::drone_actions::DroneActionError;

/// Result of a component action: the busy-tick cost plus success/failure.
/// Components compute the cost but never touch the drone's `busy_time` — the
/// drone reads `busy_cost` off the outcome and applies it itself.
pub struct ActionOutcome {
    pub busy_cost: u32,
    pub error: DroneActionError,
}

impl ActionOutcome {
    /// A successful action that costs `busy_cost` ticks of busy time.
    pub fn ok(busy_cost: u32) -> ActionOutcome {
        ActionOutcome {
            busy_cost,
            error: DroneActionError::Ok,
        }
    }

    /// A failed action. Failed actions never cost busy time.
    pub fn failed(error: DroneActionError) -> ActionOutcome {
        ActionOutcome {
            busy_cost: 0,
            error,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.error == DroneActionError::Ok
    }
}
