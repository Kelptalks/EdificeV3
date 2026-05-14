use core::fmt;
use std::collections::HashMap;

use crate::game_data::{player_data::game_entity::{components::entity_components::EntityComponent, dynamic_entity_manager::natural::puff::DynamicEntityPuff, game_entity_manager::{GameEntity, GameEntityId}}, screen::widget::widget::WidgetType};


#[derive(Clone, Copy)]
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
            DynamicEntity::Puff(puff_dynamic_entity) => puff_dynamic_entity.get_components(),
        }
    }

    pub fn get_window(self) -> Option<WidgetType> {
        match self {
            DynamicEntity::Puff(puff_dynamic_entity) => {
                None
            },
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
            DynamicEntity::Puff(puff_dynamic_entity) => {
                let id = puff_dynamic_entity.id;
                self.puffs.insert(id, puff_dynamic_entity);
            },
        }
    }
}