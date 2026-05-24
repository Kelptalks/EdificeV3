use std::collections::HashMap;

use crate::game_data::{
    game_event_manager::{
        event_manager::{Event, EventManager},
        game_event_manager::game_event_manager::GameEventManager,
    },
    player_data::game_entity::{
        game_entity_manager::{GameEntityEvent, GameEntityId},
        lair_block_entity_manager::lair_block_entitys::blueprint::blueprint::{
            BluePrintEvent, LairEntityBlueprint,
        },
    },
    screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block::LairBlockMod,
    tik_manager::game_time::GameTime,
};

// ─────────────────────────────────────────────────────────────
// LairBlockEntityId
//
// Stored in the chunk's lair_block_data array — one entry per
// occupied block position.  The embedded LairBlockMod lets the
// ray caster collect mods without ever touching the entity manager.
// ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub enum LairBlockEntityId {
    BluePrint(u64),
}

impl LairBlockEntityId {
    pub fn wrap_into_game_entity_id(self) -> GameEntityId {
        GameEntityId::LairBlockEntity(self)
    }
}

impl std::fmt::Display for LairBlockEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LairBlockEntityId::BluePrint(id) => write!(f, "BluePrint_{}", id),
        }
    }
}

// ─────────────────────────────────────────────────────────────
// LairBlockEntity
// ─────────────────────────────────────────────────────────────

#[derive(Clone)]
pub enum LairBlockEntity {
    BluePrint(LairEntityBlueprint),
}

impl LairBlockEntity {
    pub fn tik(&mut self, time: &GameTime, event_manager: &mut EventManager) {
        match self {
            LairBlockEntity::BluePrint(bp) => bp.tik(time, event_manager),
        }
    }
}

// ─────────────────────────────────────────────────────────────
// LairBlockEntityManager
// ─────────────────────────────────────────────────────────────

pub struct LairBlockEntityManager {
    blueprints: HashMap<u64, LairEntityBlueprint>,
}

impl LairBlockEntityManager {
    pub fn new() -> Self {
        LairBlockEntityManager {
            blueprints: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: LairBlockEntity) {
        match entity {
            LairBlockEntity::BluePrint(bp) => {
                self.blueprints.insert(bp.id, bp);
            }
        }
    }

    pub fn get_mod(&self, cords: &[i32; 3], entity_id: LairBlockEntityId) -> Option<LairBlockMod> {
        match entity_id {
            LairBlockEntityId::BluePrint(id) => {
                if let Some(blueprint) = self.blueprints.get(&id) {
                    return blueprint.get_mod(*cords)
                }
            },
        }
        None
    }

    pub fn tik(&mut self, time: &GameTime, event_manager: &mut EventManager) {
        for (_, bp) in &mut self.blueprints {
            bp.tik(time, event_manager);
        }
    }
}

// ─────────────────────────────────────────────────────────────
// LairBlockEntityEvent
// ─────────────────────────────────────────────────────────────

#[derive(Clone)]
pub enum LairBlockEntityEvent {
    BluePrintEvent(u64, BluePrintEvent),
}

impl LairBlockEntityEvent {
    pub fn wrap_into_event(self) -> Event {
        GameEntityEvent::LairBlockEntityEvent(self).wrap_into_event()
    }

    pub fn execute(self, manager: &mut LairBlockEntityManager, gem: &mut GameEventManager) {
        match self {
            LairBlockEntityEvent::BluePrintEvent(id, bp_event) => {
                if let Some(bp) = manager.blueprints.get_mut(&id) {
                    bp_event.execute(bp, gem);
                } else {
                    eprintln!("LairBlockEntityEvent: blueprint id({}) not found", id);
                }
            }
        }
    }
}
