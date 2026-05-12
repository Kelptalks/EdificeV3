use std::collections::HashMap;

use crate::game_data::{World, game_event_manager::{event_manager::{Event, EventManager}, render_event_manager::{render_event_manager::RenderEvent, window_manager_event::WindowManagerEvent}}, player_data::game_object::{block_entity_manager::{self, natural::flungle::BlockEntityFlungle, player_created::{battery::BlockEntityBattery, radar::{BlockEntityRadar, RadarEvent}}}, game_object_manager::{GameObject, GameObjectEvent, GameObjectId}, traits::game_object_trait_manager::GameObjectTrait}, screen::widget::{self, panel::panel::Panel, widget::{Widget, WidgetType}, window_manager::{window::WidgetWindow, windows::window_type::{Window, WindowType}}}, tik_manager::game_time::GameTime};

#[derive(Clone)]
pub enum BlockEntity {
    // player
    Radar(BlockEntityRadar),
    Battery(BlockEntityBattery),

    // natural
    Flour(BlockEntityFlungle)
}

impl BlockEntity {
    pub fn get_traits(self) -> Vec<GameObjectTrait> {
        match self {
            BlockEntity::Radar(block_entity_radar) => {
                block_entity_radar.get_traits()
            },
            BlockEntity::Battery(block_entity_batery) => {
                block_entity_batery.get_traits()
            }
            
            BlockEntity::Flour(block_entity_flungle) => {
                block_entity_flungle.get_traits()
            },
        }
    }

    pub fn get_window(self) -> Option<WidgetType> {
        match self {
            BlockEntity::Radar(block_entity_radar) => {
                Some(block_entity_radar.get_window())
            },
            _ => {
                None
            }
        }
    }
}


#[derive(Clone, Copy)]
pub enum BlockEntityId {
    // Player
    RadarID(u64),
    Battery(u64),

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
            BlockEntity::Battery(block_entity_battery) => {

            },

            BlockEntity::Flour(block_entity_flour) => {
                block_entity_flour.tik(time, world, event_manager)
            },
        }
    } 
}

pub struct BlockEntityManager {
    radars: HashMap<u64, BlockEntityRadar>,
    batterys: HashMap<u64, BlockEntityBattery>,

    flungles: HashMap<u64, BlockEntityFlungle>,


}

impl BlockEntityManager {
    pub fn new() -> BlockEntityManager {
        BlockEntityManager {
            radars: HashMap::new(),
            batterys: HashMap::new(),

            flungles: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, new_block_entity: BlockEntity) {
        match new_block_entity {
            BlockEntity::Radar(block_entity_radar) => {
                self.radars.insert(block_entity_radar.id, block_entity_radar);
            },
            BlockEntity::Battery(block_entity_battery) => {
                self.batterys.insert(block_entity_battery.id, block_entity_battery);
            }

            BlockEntity::Flour(block_entity_flour) => {
                self.flungles.insert(block_entity_flour.id, block_entity_flour);
            },
        }
    }

    pub fn clone_block_entity(&self, block_entity_id: BlockEntityId) -> Option<BlockEntity> {
        match block_entity_id {
            BlockEntityId::RadarID(id) => {
                if let Some(radar_entity) = self.radars.get(&id).cloned() {
                    Some(BlockEntity::Radar(radar_entity))
                }
                else {
                    None
                }
            },
            BlockEntityId::Battery(id) => {
                if let Some(radar_entity) = self.batterys.get(&id).cloned() {
                    Some(BlockEntity::Battery(radar_entity))
                }
                else {
                    None
                }
            },
            BlockEntityId::FlourID(id) => {
                if let Some(flour_entity) = self.flungles.get(&id).cloned() {
                    Some(BlockEntity::Flour(flour_entity))
                }
                else {
                    None
                }
            },
        }
    }


    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        for (_, entity) in &mut self.radars {
            entity.tik(time, world, event_manager);
        }
        
        for (_, entity) in &mut self.batterys {
            entity.tik(time, world, event_manager);
        }

        for (_, entity) in &mut self.flungles {
            entity.tik(time, world, event_manager);
        }


    }
}


#[derive(Clone)]
pub enum BlockEntityEvent {
    RadarEvent(u64, RadarEvent), // Id, Event
}

impl BlockEntityEvent {
    pub fn wrap_into_event(self) -> Event {
        GameObjectEvent::BlockEntityEvent(self).wrap_into_event()
    }

    pub fn execute(self, block_entity_manager: &mut BlockEntityManager) {
        match self {
            BlockEntityEvent::RadarEvent(id, radar_event) => {
                if let Some(radar) = block_entity_manager.radars.get_mut(&id) {
                    radar_event.execute(radar)
                }
                else {
                    eprintln!("Error radar entity id({}) does not exist", id);
                }
            },
        }
        

    }
}