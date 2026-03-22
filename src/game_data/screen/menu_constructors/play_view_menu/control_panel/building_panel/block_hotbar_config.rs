use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, WidgetEvent}, types::BlockTexture};

pub struct BlockHotbarConfig {
    block_slot_refs: Vec<Rc<RefCell<BlockTexture>>>,
}


impl BlockHotbarConfig {
    pub fn new() -> BlockHotbarConfig {
        BlockHotbarConfig {
            block_slot_refs: Vec::new(),
        }
    }

    pub fn get_var_slot_events(&self) -> Vec<Event> {
        let events: Vec<Event> = Vec::new(); 
        for block_ref in &self.block_slot_refs {
            
        }

        return events;
    }
}