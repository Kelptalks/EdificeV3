use crate::game_data::{locations::world_area::WorldArea, screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block::LairBlock, texture_manager::texture::Texture, types::BlockTexture};

pub struct WorldLocation {
    // Var Texture
    var_texture: Texture,


    // Identity
    area: WorldArea,
    name: String,
    id: u32,


    // Visual
    lair_block_mods: Vec<LairBlock>,
}

impl WorldLocation {
    pub fn new(name: String, area: WorldArea, id: u32) -> WorldLocation {
        println!("Created Location: {}", name);
        
        WorldLocation {
            var_texture: Texture::BlockTexture(BlockTexture::Grass),

            // Identity
            area: area,
            name: name,
            id: id,


            lair_block_mods: Vec::new(),
        }
    }

    //=====================================
    // Visuals
    //=====================================

    pub fn get_texture(&self) -> &Texture {
        return &self.var_texture;
    }

    pub fn clear_lairblock_mods(&mut self) {
        self.lair_block_mods.clear();
    }

    pub fn get_lair_block_mods(&self) -> &Vec<LairBlock> {
        return &self.lair_block_mods;
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