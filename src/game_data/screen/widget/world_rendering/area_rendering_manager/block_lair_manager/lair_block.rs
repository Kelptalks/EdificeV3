

use crate::game_data::{types::BlockTexture};

pub enum LairBlockMod {
    AddOverlayTexture(BlockTexture, [i32; 3]),
    AddUnderlayTexture(BlockTexture, [i32; 3]),
    
    Cursor([i32; 3], i32),

}

pub struct LairBlock {
    overlay_textures: Vec<BlockTexture>,
    underlay_textures: Vec<BlockTexture>
}

impl LairBlock {
    pub fn new() -> LairBlock {
        LairBlock {
            overlay_textures: Vec::new(),
            underlay_textures: Vec::new(),
        }
    }


    pub fn add_overlay_texture(&mut self, texture: BlockTexture) {
        self.overlay_textures.push(texture);
    }

    pub fn get_overlay_textures(&self) -> &Vec<BlockTexture> {
        return &self.overlay_textures;
    }

    pub fn add_underlay_texture(&mut self, texture: BlockTexture) {
        self.underlay_textures.push(texture);
    }

    pub fn get_underlay_textures(&self) -> &Vec<BlockTexture> {
        return &self.underlay_textures;
    }

}