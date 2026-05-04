use std::collections::HashMap;

use crate::game_data::{TextureManager, screen::{iso_cord_tool, widget::world_rendering::{area_rendering_manager::ray_caster::casted_tile::CastedTile, tile_map::{TileMap, TileMapId}, tile_map_cashe_manager::tile_map_texture_manager::TileMapTextureCasher}}, texture_manager};


pub struct TileMapManager {
    texture_manager: TileMapTextureCasher,
    map_lairs: HashMap<TileMapId, TileMap>,
    flattened_lair: TileMap,

}

impl TileMapManager {
    pub fn new() -> TileMapManager {
        TileMapManager {
            texture_manager: TileMapTextureCasher::new(),
            map_lairs: HashMap::new(),
            flattened_lair: TileMap::new(0),
            
        }
    }

    

    pub fn get_lairs(&self) -> &HashMap<TileMapId, TileMap> {
        &self.map_lairs
    }

    pub fn get_mut_tile_map(&mut self, id: TileMapId) -> Option<&mut TileMap> {
        self.map_lairs.get_mut(&id)
    }

    pub fn get_tile_map(&self, id: TileMapId) -> Option<&TileMap> {
        self.map_lairs.get(&id)
    }


    pub fn new_tile_map(&mut self, depth: i32) -> TileMapId {
        let new_map = TileMap::new(depth);
        let id = new_map.get_id();

        self.map_lairs.insert(new_map.get_id(), new_map);

        id
    }
    
    pub fn get_tile_with_flattened_cords(&self, tile_key: &[i32; 2]) -> Option<&CastedTile> {
        self.flattened_lair.get_tile_with_flattened_cords(tile_key)
    }

    // Compair all the lairs and set each tile to the one with the lowest depth
    pub fn flatten(&mut self) {
        for (id, lair) in &mut self.map_lairs {
            for (cords, lair_tile) in lair.get_tile_map() {
                
                let current_tile = self.flattened_lair.get_mut_tile_with_flattened_cords(cords);
                if let Some(current_tile) = current_tile {
                        
                    let lair_left_triangle = lair_tile.get_left_triangle(); 
                    let lair_right_triangle = lair_tile.get_right_triangle();

                    let current_left_triangle = current_tile.get_left_triangle().clone();
                    let current_right_triangle = current_tile.get_right_triangle().clone();


                    
                    if lair_left_triangle.has_struck_solid {
                        if !current_left_triangle.has_struck_solid {
                            current_tile.set_left_triangle(lair_left_triangle.clone());
                        }
                        else if current_left_triangle.get_struck_depth() < lair_left_triangle.get_struck_depth() {                                
                            current_tile.set_left_triangle(lair_left_triangle.clone());
                        }
                    }

                    if lair_right_triangle.has_struck_solid {   
                        if !current_right_triangle.has_struck_solid {
                            current_tile.set_right_triangle(lair_right_triangle.clone());
                        }
                        else if current_right_triangle.get_struck_depth() < lair_right_triangle.get_struck_depth() {
                            current_tile.set_right_triangle(lair_right_triangle.clone());
                        }
                    }
                }
                else {
                    self.flattened_lair.incert_tile_with_flattened_cords(*cords, lair_tile.clone());
                }
            }
        }
    }

    pub fn render(&mut self, texture_manager: &mut TextureManager, ndc_block_scale: f32, draw_cords: [f32; 2]) {
        for (id, lair) in &mut self.map_lairs {
            if lair.is_cashed() {

                if lair.is_cashe_dirty() {
                    self.texture_manager.render_tile_map(texture_manager, lair)
                }

            }
        }   
        
        
        self.flattened_lair.reset(0);
        self.flatten();
        self.flattened_lair.render(texture_manager, ndc_block_scale, draw_cords);
    }

    pub fn get_texture_cashe_manager(&mut self) -> &mut TileMapTextureCasher {
        &mut self.texture_manager
    }
}


