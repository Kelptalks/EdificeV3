use std::collections::HashMap;

use crate::game_data::{World, game_event_manager::{event_manager::EventManager, render_event_manager::{render_event_manager::RenderEvent, window_manager_event::WindowManagerEvent}}, player_data::game_object::{block_entity_manager::{natural::flour::BlockEntityFlour, player_created::radar::BlockEntityRadar}, game_object_manager::{GameObject, GameObjectId}, traits::game_object_trait_manager::GameObjectTrait}, screen::widget::{self, panel::panel::Panel, widget::{Widget, WidgetType}, window_manager::{window::WidgetWindow, windows::window_type::{Window, WindowType}}}, tik_manager::game_time::GameTime};

#[derive(Clone)]
pub enum BlockEntity {
    // player
    Radar(BlockEntityRadar),

    // natural
    Flour(BlockEntityFlour)
}

impl BlockEntity {
    pub fn get_traits(self) -> Vec<GameObjectTrait> {
        match self {
            BlockEntity::Radar(block_entity_radar) => {
                block_entity_radar.get_traits()
            },
            BlockEntity::Flour(block_entity_flour) => todo!(),
        }
    }
}


#[derive(Clone, Copy)]
pub enum BlockEntityId {
    // Player
    RadarID(u64),

    // Natural
    FlourID(u64),
}

impl BlockEntityId {
    pub fn wrap_into_game_object_id(self) -> GameObjectId {
        GameObjectId::BlockEntity(self)
    }
}

impl BlockEntity {
    pub fn wrap_into_game_object(self) -> GameObject {
        GameObject::BlockEntity(self)
    }

    

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        match self {
            BlockEntity::Radar(block_entity_radar) => {
                block_entity_radar.tik(time, world, event_manager);
            },
            BlockEntity::Flour(block_entity_flour) => {
                block_entity_flour.tik(time, world, event_manager)
            },
        }
    } 
}

pub struct BlockEntityManager {
    radar_entitys: HashMap<u64, BlockEntityRadar>,
    flour_entitys: HashMap<u64, BlockEntityFlour>,
}

impl BlockEntityManager {
    pub fn new() -> BlockEntityManager {
        BlockEntityManager {
            radar_entitys: HashMap::new(),
            flour_entitys: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, new_block_entity: BlockEntity) {
        match new_block_entity {
            BlockEntity::Radar(block_entity_radar) => {
                self.radar_entitys.insert(block_entity_radar.id, block_entity_radar);
            },
            BlockEntity::Flour(block_entity_flour) => {
                self.flour_entitys.insert(block_entity_flour.id, block_entity_flour);
            },
        }
    }

    pub fn get_block_entity(&self, block_entity_id: BlockEntityId) -> Option<BlockEntity> {
        match block_entity_id {
            BlockEntityId::RadarID(id) => {
                if let Some(radar_entity) = self.radar_entitys.get(&id).cloned() {
                    Some(BlockEntity::Radar(radar_entity))
                }
                else {
                    None
                }
                
            },
            BlockEntityId::FlourID(id) => {
                if let Some(flour_entity) = self.flour_entitys.get(&id).cloned() {
                    Some(BlockEntity::Flour(flour_entity))
                }
                else {
                    None
                }
            },
        }
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        for (key, entity) in &mut self.radar_entitys {
            entity.tik(time, world, event_manager);
        }


        for (key, entity) in &mut self.flour_entitys {
            entity.tik(time, world, event_manager);
        }
    }
}