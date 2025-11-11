
use std::{time::{SystemTime, UNIX_EPOCH}, u128};

pub struct TikManager {
    current_tik: u32,
    tik_rate: u128,

    last_tik_millis: u128,


}

impl TikManager {
    pub fn new() -> Self {
        Self {
            current_tik: 0,
            tik_rate: 200,

            last_tik_millis: 0,
        }
    }

    pub fn set_tik_rate(&mut self, new_tik_rate: u128) {
        self.tik_rate = new_tik_rate;
    } 

    // Called every frame to update the tik
    pub fn update_tik_manager(&mut self) {
        let current_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

        if (current_millis > (self.last_tik_millis + self.tik_rate)) {
            self.current_tik += 1;
            self.last_tik_millis = current_millis;
        }
    }


}