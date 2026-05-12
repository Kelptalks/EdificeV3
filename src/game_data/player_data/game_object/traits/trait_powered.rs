use std::f32::consts::E;

use crate::game_data::{game_event_manager::{self, event_manager::{Event, EventManager}}, player_data::game_object::{game_object_manager::{GameObject, GameObjectId}, traits::game_object_trait_manager::{GameObjectTrait, TraitEvent}}, tik_manager::game_time::{GameTime, GameTimeInterval}};


#[derive(Clone)]
pub struct PoweredTrait {
    pub power_links: Vec<GameObjectId>,
    pub power_requests: Vec<(GameObjectId, u32)>,

    pub max_power: u32,
    pub power_stored: u32,
    pub power_demand: u32,

    pub power_consume_interval: GameTimeInterval,
    pub power_consume_per_interval: u32,
}

impl PoweredTrait {
    pub fn wrap_into_trait(self) -> GameObjectTrait {
        GameObjectTrait::Powered(self)
    }
    
    pub fn new() -> PoweredTrait {
        PoweredTrait {
            power_links: Vec::new(),
            power_requests: Vec::new(),

            max_power: 10000,
            power_stored: 0,
            power_demand: 5000,

            power_consume_interval: GameTimeInterval::Second,
            power_consume_per_interval: 0,

        }
    }

    pub fn power_surplus(&self) -> u32 {
        self.power_stored - self.power_demand
    }

    pub fn tik(&mut self, game_time: &GameTime, event_manager: &mut EventManager) -> bool {
        
        // Handle Power requests
        while let Some((object_id, amount_requested)) = self.power_requests.pop() {
            if self.power_surplus() < amount_requested {
                self.power_stored -= amount_requested;
                
            }
        }
        
        
        
        
        
        
        if self.power_stored > 0 {
            if self.power_consume_interval.is_interval(game_time) {
                if self.power_stored > self.power_consume_per_interval {
                    self.power_stored -= self.power_consume_per_interval;
                }
                else {
                    self.power_stored = 0;
                    return false
                }
            }
        }
        else {
            return false
        }
        return true;
    }

    pub fn set_power_consumption(&mut self, interval: GameTimeInterval, amount: u32) {
        self.power_consume_interval = interval;
        self.power_consume_per_interval = amount;
    }

}


#[derive(Clone)]
pub enum PoweredTraitEvent {
    AddPowerLink(GameObjectId), // Object Supplying 
    
    PowerRequest(GameObjectId, u32),

    PowerReceived(u32), // amount
}

impl PoweredTraitEvent {
    pub fn wrap_into_event(self, id: GameObjectId) -> Event {
        TraitEvent::PoweredEvent(self).wrap_into_event(id)
    } 
    
    pub fn execute(self, power_trait: &mut PoweredTrait) {
        match self {
            PoweredTraitEvent::AddPowerLink(game_object_id) => {
                power_trait.power_links.push(game_object_id);
            },
            PoweredTraitEvent::PowerRequest(game_object_id, amount) => {
                power_trait.power_requests.push((game_object_id, amount));
                
            },
            PoweredTraitEvent::PowerReceived(amount) => {
                power_trait.power_stored += amount;
            },
        }
    }
}