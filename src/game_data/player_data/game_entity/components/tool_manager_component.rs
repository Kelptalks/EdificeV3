use crate::game_data::{
    player_data::{
        drones::drone_actions::drone_actions::DroneActionError,
        game_entity::components::{action_outcome::ActionOutcome, entity_components::EntityComponent},
    },
    types::drone_item::DroneItem,
};

const TOOL_SLOTS: usize = 3;

/// Holds a drone's equipped tools and the mine/chop power they grant.
#[derive(Clone)]
pub struct ToolManagerComponent {
    tools: [Option<DroneItem>; TOOL_SLOTS],
    mine_power: u32,
    chop_power: u32,
}

impl ToolManagerComponent {
    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::ToolManager(self)
    }

    pub fn new() -> ToolManagerComponent {
        ToolManagerComponent {
            tools: [None; TOOL_SLOTS],
            mine_power: 1,
            chop_power: 1,
        }
    }

    pub fn get_tools(&self) -> &[Option<DroneItem>; TOOL_SLOTS] {
        &self.tools
    }

    pub fn get_mine_power(&self) -> u32 {
        self.mine_power
    }

    pub fn get_chop_power(&self) -> u32 {
        self.chop_power
    }

    /// Busy ticks to mine a block of the given hardness, scaled by mine power.
    pub fn get_mine_time(&self, hardness: u32) -> u32 {
        hardness / self.mine_power
    }

    /// Busy ticks to chop a block of the given hardness, scaled by chop power.
    pub fn get_chop_time(&self, hardness: u32) -> u32 {
        hardness / self.chop_power
    }

    /// Places `tool` in the first free slot and recalculates power. The caller
    /// (drone) removes the tool from inventory only when this returns Ok.
    pub fn equip(&mut self, tool: DroneItem) -> ActionOutcome {
        for slot in self.tools.iter_mut() {
            if slot.is_none() {
                *slot = Some(tool);
                self.recalculate_power();
                return ActionOutcome::ok(0);
            }
        }
        ActionOutcome::failed(DroneActionError::MissingSlot)
    }

    /// Sums tool power across equipped tools; base power is 1.
    fn recalculate_power(&mut self) {
        let mut mine_power = 1;
        let mut chop_power = 1;
        for tool in self.tools.iter().flatten() {
            mine_power += tool.mine_power();
            chop_power += tool.chop_power();
        }
        self.mine_power = mine_power;
        self.chop_power = chop_power;
    }
}
