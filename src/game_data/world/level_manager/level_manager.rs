use crate::game_data::level_manager::{level::Level, levels::{flat_field::{self, FlatField}, monoliths::Monoliths, wall::Wall}};

pub struct LevelManager {
    levels: Vec<Box<dyn Level>>,
    
}

impl LevelManager {
    pub fn new() -> Self {
        let mut levels: Vec<Box<dyn Level>> = Vec::new();
        
        // Add different level types
        levels.push(Box::new(FlatField::new()));
        levels.push(Box::new(Wall::new()));
        levels.push(Box::new(Monoliths::new()));

        Self { levels }
    }

    pub fn get_total_levels(&self) -> usize {
        return self.levels.len();
    }

    // Or if you want to panic on invalid index
    pub fn get_level_at_index(&self, index: usize) -> &dyn Level {
        self.levels[index].as_ref()
    }
}
