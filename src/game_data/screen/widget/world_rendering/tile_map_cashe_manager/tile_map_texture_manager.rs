use std::collections::HashMap;

use image::{ImageBuffer, RgbaImage};
use miniquad::{GlContext, RenderingBackend, TextureId};

use crate::game_data::{TextureManager, screen::widget::world_rendering::{tile_map::{TileMap, TileMapId}, tile_map_manager::{self, TileMapManager}}, texture_manager::texture_renderer::TextureRenderingManager};

pub struct TileMapTextureCasher {
    textures_to_render: Vec<TileMapId>,
    
    textures: HashMap<TileMapId, TextureId>
}

impl TileMapTextureCasher {
    pub fn new() -> TileMapTextureCasher {
        TileMapTextureCasher {
            textures_to_render: Vec::new(),

            textures: HashMap::new(),
        }
    }



    pub fn render_cashed_tile_map(&mut self, id: &TileMapId) {
        self.textures_to_render.push(*id);
    }


    pub fn render_tile_map(
        &mut self, 
        texture_manager: &mut TextureManager, 
        map: &mut TileMap
    ) {
        if let Some(texture) = self.textures.get(&map.get_id()) {
                



        }
    }

    pub fn flush(
        &mut self, 
        texture_renderer: &mut TextureRenderingManager, 
        ctx: &mut GlContext
    ) {
        while let Some(id) = &self.textures_to_render.pop() {
            if let Some(texture) = self.textures.get(id) {
                // println!("rendering texture");

            }
            else {
                println!("created texture");
                let height = 100;
                let width = 100;

                let mut image: RgbaImage = ImageBuffer::new(width, height);
                let rgba_bytes: Vec<u8> = image.clone().into_raw();
                let new_texture: TextureId = ctx.new_texture_from_rgba8(
                    height as u16,
                    width as u16,
                    &rgba_bytes,
                );

                self.textures.insert(*id, new_texture);
            }
        }
    }

}
