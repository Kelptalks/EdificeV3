use std::{cell::RefCell, rc::Rc};

use crate::game_data::{World, game_event_manager::game_event_manager::EventData};

#[derive(Clone)]
pub enum WorldConfigEvent {
    // Links
    ModWorldSize(i32),
    LinkToggleTreeGen(Rc<RefCell<bool>>),
}


impl WorldConfigEvent {
    //=====================================
    // Execution
    //=====================================
    pub fn execute_world_event(&self, world: &mut World, event_data: &mut EventData) {
        let world_config = event_data.get_mut_world_gen_manager().get_mut_world_config();
        match self {
            WorldConfigEvent::ModWorldSize(scale) => {
                world_config.mod_scale(*scale);
            },
            WorldConfigEvent::LinkToggleTreeGen(ref_cell) => {
                world_config.set_flat_world_toggle_link(ref_cell.clone());
            },
        }
    }
}