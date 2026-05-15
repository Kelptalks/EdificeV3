use core::fmt;
use std::collections::HashMap;

use crate::game_data::{World, game_event_manager::{event_manager::{Event, EventManager}, game_event_manager::GameEvent, render_event_manager::{render_event_manager::RenderEvent, window_manager_event::WindowManagerEvent}}, player_data::game_entity::{block_entity_manager::{self, natural::flungle::BlockEntityFlungle, player_created::{battery::{BatteryEvent, BlockEntityBattery}, drone::BlockEntityDrone, radar::{BlockEntityRadar, RadarEvent}}}, components::{block_component::BlockComponentEvent, entity_components::{EntityComponent, EntityComponentEvent}, powered_component::PoweredComponentEvent}, game_entity_manager::{GameEntity, GameEntityEvent, GameEntityId}}, screen::widget::{self, panel::panel::Panel, widget::{Widget, WidgetType}, window_manager::{window::WidgetWindow, windows::window_type::{Window, WindowType}}}, tik_manager::game_time::GameTime};

/*
#####################
## Block Entity Id ##
#####################
*/
#[derive(Clone, Copy)]
pub enum BlockEntityId {
    // Player
    Radar(u64),
    Battery(u64),
    Drone(u64),

    // Natural
    Flungle(u64),
}

impl BlockEntityId {
    pub fn wrap_into_game_entity_id(self) -> GameEntityId {
        GameEntityId::BlockEntity(self)
    }

    pub fn get_component_event(&self, component_event: EntityComponentEvent) -> Option<Event> {
        match self {
            BlockEntityId::Radar(id) => {
                Some(RadarEvent::ComponentEvent(component_event).wrap_into_event(*id))
            },
            BlockEntityId::Battery(id) => {
                Some(BatteryEvent::ComponentEvent(component_event).wrap_into_event(*id))
            },
            BlockEntityId::Drone(_id) => None,
            BlockEntityId::Flungle(_id) => None,
        }
    }
}

impl fmt::Display for BlockEntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockEntityId::Radar(id)   => write!(f, "Radar_{}", id),
            BlockEntityId::Battery(id) => write!(f, "Battery_{}", id),
            BlockEntityId::Drone(id)   => write!(f, "Drone_{}", id),
            BlockEntityId::Flungle(id) => write!(f, "Flungle_{}", id),
        }
    }
}


/*
##################
## Block Entity ##
##################
*/

#[derive(Clone)]
pub enum BlockEntity {
    // player
    Drone(BlockEntityDrone),
    Radar(BlockEntityRadar),
    Battery(BlockEntityBattery),

    // natural
    Flour(BlockEntityFlungle),
}

impl BlockEntity {
    pub fn get_components(self) -> Vec<EntityComponent> {
        match self {
            BlockEntity::Radar(radar)     => radar.get_components(),
            BlockEntity::Battery(battery) => battery.get_components(),
            BlockEntity::Drone(drone)     => drone.get_components(),
            BlockEntity::Flour(flungle)   => flungle.get_components(),
        }
    }

    pub fn get_window(self) -> Option<WidgetType> {
        match self {
            BlockEntity::Radar(radar) => Some(radar.get_window()),
            _ => None,
        }
    }

    pub fn wrap_into_game_entity(self) -> GameEntity {
        GameEntity::BlockEntity(self)
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        match self {
            BlockEntity::Radar(radar)     => radar.tik(time, world, event_manager),
            BlockEntity::Battery(battery) => battery.tik(time, world, event_manager),
            BlockEntity::Drone(drone)     => drone.tik(time, world, event_manager),
            BlockEntity::Flour(flungle)   => flungle.tik(time, world, event_manager),
        }
    }
}

pub struct BlockEntityManager {
    radars:   HashMap<u64, BlockEntityRadar>,
    batterys: HashMap<u64, BlockEntityBattery>,
    drones:   HashMap<u64, BlockEntityDrone>,
    flungles: HashMap<u64, BlockEntityFlungle>,
}

impl BlockEntityManager {
    pub fn new() -> BlockEntityManager {
        BlockEntityManager {
            radars:   HashMap::new(),
            batterys: HashMap::new(),
            drones:   HashMap::new(),
            flungles: HashMap::new(),
        }
    }

    //=====================================
    // Direct Object Getters
    //=====================================

    pub fn get_radar(&mut self, id: u64) -> Option<&mut BlockEntityRadar> {
        self.radars.get_mut(&id)
    }

    pub fn get_battery(&mut self, id: u64) -> Option<&mut BlockEntityBattery> {
        self.batterys.get_mut(&id)
    }

    pub fn get_flungle(&mut self, id: u64) -> Option<&mut BlockEntityFlungle> {
        self.flungles.get_mut(&id)
    }

    //=====================================
    // Entity Management
    //=====================================

    pub fn add_entity(&mut self, new_block_entity: BlockEntity) {
        match new_block_entity {
            BlockEntity::Radar(radar)     => { self.radars.insert(radar.id, radar); },
            BlockEntity::Battery(battery) => { self.batterys.insert(battery.id, battery); },
            BlockEntity::Drone(drone)     => { self.drones.insert(drone.id, drone); },
            BlockEntity::Flour(flungle)   => { self.flungles.insert(flungle.id, flungle); },
        }
    }

    pub fn clone_block_entity(&self, block_entity_id: BlockEntityId) -> Option<BlockEntity> {
        match block_entity_id {
            BlockEntityId::Radar(id)   => self.radars.get(&id).cloned().map(BlockEntity::Radar),
            BlockEntityId::Battery(id) => self.batterys.get(&id).cloned().map(BlockEntity::Battery),
            BlockEntityId::Drone(id)   => self.drones.get(&id).cloned().map(BlockEntity::Drone),
            BlockEntityId::Flungle(id) => self.flungles.get(&id).cloned().map(BlockEntity::Flour),
        }
    }

    //=====================================
    // Tick Management
    //=====================================

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        for (_, entity) in &mut self.radars   { entity.tik(time, world, event_manager); }
        for (_, entity) in &mut self.batterys { entity.tik(time, world, event_manager); }
        for (_, entity) in &mut self.flungles { entity.tik(time, world, event_manager); }
        for (_, entity) in &mut self.drones   { entity.tik(time, world, event_manager); }
    }
}

/*
######################
## BlockEntityEvent ##
######################
*/

#[derive(Clone)]
pub enum BlockEntityEvent {
    RadarEvent(u64, RadarEvent),
    BatteryEvent(u64, BatteryEvent),
}

impl BlockEntityEvent {
    pub fn wrap_into_event(self) -> Event {
        GameEntityEvent::BlockEntityEvent(self).wrap_into_event()
    }

    pub fn execute(self, block_entity_manager: &mut BlockEntityManager) {
        match self {
            BlockEntityEvent::RadarEvent(id, radar_event) => {
                if let Some(radar) = block_entity_manager.radars.get_mut(&id) {
                    radar_event.execute(radar);
                } else {
                    eprintln!("Error radar entity id({}) does not exist", id);
                }
            },
            BlockEntityEvent::BatteryEvent(id, battery_event) => {
                if let Some(battery) = block_entity_manager.batterys.get_mut(&id) {
                    battery_event.execute(battery);
                } else {
                    eprintln!("Error battery entity id({}) does not exist", id);
                }
            },
        }
    }
}
