use core::fmt;
use std::collections::HashMap;

use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_entity::{components::entity_components::EntityComponent, dynamic_entity_manager::natural::puff::DynamicEntityPuff, game_entity_manager::{GameEntity, GameEntityId}}, screen::widget::widget::WidgetType, texture_manager::texture::Texture, tik_manager::game_time::GameTime};


#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum DynamicEntityId {
    Puff(u64)
}

impl DynamicEntityId {
    pub fn wrap_into_game_entity_id(self) -> GameEntityId {
        GameEntityId::DynamicEntity(self)
    }
}


impl fmt::Display for DynamicEntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynamicEntityId::Puff(id) => write!(f, "Puff_{}", id),
        }
    }
}




#[derive(Clone)]
pub enum DynamicEntity {
    Puff(DynamicEntityPuff)
}

impl DynamicEntity {
    pub fn wrap_into_game_entity(self) -> GameEntity {
        GameEntity::DynamicEntity(self)
    }

    pub fn get_components(self) -> Vec<EntityComponent> {
        match self {
            DynamicEntity::Puff(puff) => puff.get_components(),
        }
    }

    pub fn get_window(self) -> Option<WidgetType> {
        match self {
            DynamicEntity::Puff(_) => None,
        }
    }

    pub fn world_pos(&self) -> [f32; 3] {
        match self {
            DynamicEntity::Puff(puff) => puff.cords(),
        }
    }

    pub fn texture(&self) -> Texture {
        match self {
            DynamicEntity::Puff(puff) => puff.get_texture(),
        }
    }
}

pub struct DynamicEntityManager {
    puffs: HashMap<u64, DynamicEntityPuff>,
}

impl DynamicEntityManager {
    pub fn new() -> DynamicEntityManager {
        DynamicEntityManager {
            puffs: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, dynamic_entity: DynamicEntity) {
        match dynamic_entity {
            DynamicEntity::Puff(puff) => {
                self.puffs.insert(puff.id, puff);
            },
        }
    }

    pub fn clone_entity(&self, id: DynamicEntityId) -> Option<DynamicEntity> {
        match id {
            DynamicEntityId::Puff(id) => self.puffs.get(&id).cloned().map(DynamicEntity::Puff),
        }
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        for (_, puff) in &mut self.puffs {
            puff.tik(time, world, event_manager);
        }
    }

    /// Returns `(world_pos, texture)` for every dynamic entity.
    pub fn iter_world_pos_and_texture(&self) -> impl Iterator<Item = ([f32; 3], Texture)> + '_ {
        self.puffs.values().map(|puff| (puff.cords(), puff.get_texture()))
    }
}
