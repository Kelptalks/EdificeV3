use crate::game_data::locations::world_area::WorldArea;

pub struct Location {
    area: WorldArea,
    name: String

}

impl Location {
    pub fn new(name: String, area: WorldArea) -> Location {
        println!("Created Location: {}", name);
        
        Location {
            area: area,
            name: name
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    // Name
    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    // Area
    pub fn get_mut_area(&mut self) -> &mut WorldArea {
        return &mut self.area;
    }


    


}