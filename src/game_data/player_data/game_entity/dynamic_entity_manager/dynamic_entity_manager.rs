use core::fmt;
use std::collections::HashMap;

use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_entity::{components::entity_components::EntityComponent, dynamic_entity_manager::{natural::puff::DynamicEntityPuff, player_created::cursor_entity::CursorEntity}, game_entity_manager::{GameEntity, GameEntityId}}, screen::{ScreenData, widget::{panel::panel::Panel, widget::WidgetType, world_rendering::world_view_data::WorldViewData}}, texture_manager::texture::Texture, tik_manager::game_time::GameTime};


#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum DynamicEntityId {
    Puff(u64),
    CursorEntity(u64),
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
            DynamicEntityId::CursorEntity(id) => write!(f, "CursorEntity_{}", id),
        }
    }
}


#[derive(Clone)]
pub enum DynamicEntity {
    Puff(DynamicEntityPuff),
    CursorEntity(CursorEntity),
}

impl DynamicEntity {
    pub fn wrap_into_game_entity(self) -> GameEntity {
        GameEntity::DynamicEntity(self)
    }

    pub fn get_components(self) -> Vec<EntityComponent> {
        match self {
            DynamicEntity::Puff(puff) => puff.get_components(),
            DynamicEntity::CursorEntity(e) => e.get_components(),
        }
    }

    pub fn get_window(self) -> Option<WidgetType> {
        match self {
            DynamicEntity::Puff(_) => None,
            DynamicEntity::CursorEntity(e) => Some(e.get_window()),
        }
    }

    pub fn world_pos(&self) -> [f32; 3] {
        match self {
            DynamicEntity::Puff(puff) => puff.cords(),
            DynamicEntity::CursorEntity(e) => e.world_pos(),
        }
    }

    pub fn texture(&self) -> Texture {
        match self {
            DynamicEntity::Puff(puff) => puff.get_texture(),
            DynamicEntity::CursorEntity(e) => e.texture(),
        }
    }

    pub fn play_view(&self, _world_view_data: &WorldViewData, _screen_data: &ScreenData, _world: &World, _event_manager: &mut EventManager) -> WidgetType {
        // No dynamic entity implements a spectate view yet.
        Panel::new_blank().wrap_into_widget()
    }
}

pub struct DynamicEntityManager {
    puffs: HashMap<u64, DynamicEntityPuff>,
    cursor_entities: HashMap<u64, CursorEntity>,
}

impl DynamicEntityManager {
    pub fn new() -> DynamicEntityManager {
        DynamicEntityManager {
            puffs: HashMap::new(),
            cursor_entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, dynamic_entity: DynamicEntity) {
        match dynamic_entity {
            DynamicEntity::Puff(puff) => {
                self.puffs.insert(puff.id, puff);
            },
            DynamicEntity::CursorEntity(e) => {
                self.cursor_entities.insert(e.id, e);
            },
        }
    }

    pub fn clone_entity(&self, id: DynamicEntityId) -> Option<DynamicEntity> {
        match id {
            DynamicEntityId::Puff(id) => self.puffs.get(&id).cloned().map(DynamicEntity::Puff),
            DynamicEntityId::CursorEntity(id) => self.cursor_entities.get(&id).cloned().map(DynamicEntity::CursorEntity),
        }
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        for (_, puff) in &mut self.puffs {
            puff.tik(time, world, event_manager);
        }
        for (_, entity) in &mut self.cursor_entities {
            entity.tik(time, world, event_manager);
        }
    }

    /// Returns `(world_pos, texture)` for every dynamic entity.
    pub fn iter_world_pos_and_texture(&self) -> impl Iterator<Item = ([f32; 3], Texture)> + '_ {
        let puffs = self.puffs.values().map(|p| (p.cords(), p.get_texture()));
        let cursors = self.cursor_entities.values().map(|e| (e.world_pos(), e.texture()));
        puffs.chain(cursors)
    }
}
