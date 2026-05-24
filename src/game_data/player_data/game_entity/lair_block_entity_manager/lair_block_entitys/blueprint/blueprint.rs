
use std::collections::HashMap;

use crate::game_data::{game_event_manager::{event_manager::{self, EventManager}, game_event_manager::GameEventManager}, player_data::game_entity::lair_block_entity_manager::lair_block_entity_manager::{LairBlockEntity, LairBlockEntityEvent, LairBlockEntityId}, screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block::LairBlockMod, tik_manager::game_time::GameTime, tools::id_gen::IdGen, types::BlockTexture, world::world::WorldEvent};

static ID_GEN: IdGen = IdGen::new();

/// A blueprint: a set of ghost-block positions that visualise a planned
/// structure in the lair layer without placing real blocks.
#[derive(Clone)]
pub struct LairEntityBlueprint {
    pub id: u64,
    /// Each entry is (world_cords, block to show as a ghost overlay).
    blocks: HashMap<[i32; 3], BlockTexture>,

}

impl LairEntityBlueprint {
    pub fn new(blocks: HashMap<[i32; 3], BlockTexture>, event_manager: &mut EventManager) -> Self {
        let id = ID_GEN.new_id();

        for (cords, block) in &blocks {
            let event = WorldEvent::AddGameEntity(
                *cords, 
                LairBlockEntityId::BluePrint(id).wrap_into_game_entity_id()
            );
            event_manager.add_world_event(event);
        }

        LairEntityBlueprint {
            id, 
            blocks: blocks, 
        }        
    }

    pub fn wrap_into_lair_block_entity(self) -> LairBlockEntity {
        LairBlockEntity::BluePrint(self)
    }


    pub fn tik(&mut self, _time: &GameTime, _event_manager: &mut EventManager) {}


    pub fn get_mod(&self, cords: [i32; 3]) -> Option<LairBlockMod> {
        if let Some(block) = self.blocks.get(&cords) {
            Some(LairBlockMod::AddOverlayTexture(*block, cords))
        }
        else {
            None
        }
    }

}

// ─────────────────────────────────────────────────────────────
// Event
// ─────────────────────────────────────────────────────────────

#[derive(Clone)]
pub enum BluePrintEvent {
    /// Replace the block list and re-write all lair mods.
    SetBlocks(Vec<([i32; 3], BlockTexture)>),
    /// Remove this blueprint from the lair layer.
    Remove,
}

impl BluePrintEvent {
    pub fn wrap_into_event(self, id: u64) -> crate::game_data::game_event_manager::event_manager::Event {
        LairBlockEntityEvent::BluePrintEvent(id, self).wrap_into_event()
    }

    /// Executed during event-dispatch; uses `GameEventManager` (not the full
    /// `EventManager` wrapper) to queue follow-up world events.
    pub fn execute(self, blueprint: &mut LairEntityBlueprint, gem: &mut GameEventManager) {
        match self {
            BluePrintEvent::SetBlocks(blocks) => {

            }
            BluePrintEvent::Remove => {
                
            }
        }
    }
}
