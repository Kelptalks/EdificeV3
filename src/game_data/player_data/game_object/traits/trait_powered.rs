use std::f32::consts::E;

use crate::game_data::{player_data::game_object::traits::game_object_trait_manager::GameObjectTrait, tik_manager::game_time::{GameTime, GameTimeInterval}};


#[derive(Clone)]
pub struct PoweredTrait {
    pub max_power: u32,
    pub stored_power: u32,
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
            max_power: 10000,
            stored_power: 0,
            power_demand: 0,

            power_consume_interval: GameTimeInterval::Second,
            power_consume_per_interval: 0,

        }
    }

    pub fn tik(&mut self, game_time: &GameTime) -> bool {
        if self.stored_power > 0 {
            if self.power_consume_interval.is_interval(game_time) {
                if self.stored_power > self.power_consume_per_interval {
                    self.stored_power -= self.power_consume_per_interval;
                }
                else {
                    self.stored_power = 0;
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

pub enum PowerTraitEvent {
    
}