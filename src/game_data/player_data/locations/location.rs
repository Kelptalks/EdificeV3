use crate::game_data::{locations::world_area::WorldArea, texture_manager::texture::Texture, types::BlockTexture};

pub struct WorldLocation {
    // Var Texture
    var_texture: Texture,


    //
    area: WorldArea,
    name: String,
    id: u32
}

impl WorldLocation {
    pub fn new(name: String, area: WorldArea, id: u32) -> WorldLocation {
        println!("Created Location: {}", name);
        
        WorldLocation {
            var_texture: Texture::BlockTexture(BlockTexture::Grass),

            area: area,
            name: name,
            id: id,
        }
    }

    //=====================================
    // Var Converters
    //=====================================

    pub fn get_texture(&self) -> &Texture {
        return &self.var_texture;
    }

    //=====================================
    // Getters / Setters
    //=====================================

    // Name
    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub fn get_id(&self) -> u32 {
        return self.id;
    }
    // Area
    pub fn get_mut_area(&mut self) -> &mut WorldArea {
        return &mut self.area;
    }
    pub fn get_area(&self) -> &WorldArea {
        return &self.area;
    }

    pub fn set_area(&mut self, new_area: WorldArea) {
        self.area = new_area;
    }

    /// Set the name 
    /// 
    /// Why (crate)?
    /// to prevent renaming messing up name hashmap in location manager
    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }


    


}