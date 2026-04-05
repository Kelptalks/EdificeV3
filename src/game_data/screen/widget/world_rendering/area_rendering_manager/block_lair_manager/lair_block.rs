use crate::game_data::{texture_manager::texture::Texture, types::BlockTexture};

pub struct LairBlock {
    textures: Vec<BlockTexture>
}

impl LairBlock {
    pub fn new() -> LairBlock {
        LairBlock {
            textures: Vec::new()
        }
    }


    pub fn add_texture(&mut self, texture: BlockTexture) {
        self.textures.push(texture);
    }

    pub fn get_textures(&self) -> &Vec<BlockTexture> {
        return &self.textures;
    }
}