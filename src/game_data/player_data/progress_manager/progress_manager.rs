#![allow(dead_code)]
use std::collections::HashMap;

use crate::game_data::{screen::widget::window_manager::windows::window_type::WindowType, types::BlockTexture};



pub struct ProgressManager {
    block_count_mined: HashMap<BlockTexture, u32>,

    ui_unlocks: Vec<WindowType>

}


impl ProgressManager {
    pub fn new() -> ProgressManager {
        ProgressManager {
            block_count_mined: HashMap::new(),

            ui_unlocks: Vec::new(),
        }
    }


    pub fn update_unlocks(&mut self) {


    }
}