use std::collections::HashMap;

use crate::game_data::{TextureManager, World, screen::{iso_cord_tool, widget::world_rendering::{area_rendering_manager::ray_caster::casted_tile::CastedTile, tile_map::{self, TileMap, TileMapId}}}, texture_manager};


pub struct TileMapManager {
    block_ncd_scale: f32,
    draw_offset: [f32; 2],
    
    map_lairs: HashMap<TileMapId, TileMap>,
    flattened_lair: TileMap,

}

impl TileMapManager {
    pub fn new() -> TileMapManager {
        TileMapManager {
            block_ncd_scale: 0.0,
            draw_offset: [0.0; 2],

            map_lairs: HashMap::new(),
            flattened_lair: TileMap::new(),
        }
    }

    pub fn get_block_scale(&self) -> f32 {
        self.block_ncd_scale
    }

    pub fn get_draw_offset(&self) -> [f32; 2] {
        self.draw_offset
    }

    pub fn update_rendering_data(&mut self, block_ncd_scale: f32, draw_offset: [f32; 2]) {
        self.block_ncd_scale = block_ncd_scale;
        self.draw_offset = draw_offset;
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


    pub fn new_tile_map(&mut self) -> TileMapId {
        let new_map = TileMap::new();
        let id = new_map.id;

        self.map_lairs.insert(new_map.id, new_map);

        id
    }
    
    pub fn get_tile_with_flattened_cords(&self, tile_key: &[i32; 2]) -> Option<&CastedTile> {
        self.flattened_lair.get_tile_with_flattened_cords(tile_key)
    }

    // Compair all the lairs and set each tile to the one with the lowest depth
    pub fn flatten(&mut self) {
        for (id, lair) in &mut self.map_lairs {
            for (cords, lair_tile) in &mut lair.map {
                
                let current_tile = self.flattened_lair.get_mut_tile_with_flattened_cords(&cords);
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

    pub fn flatten_lairs(&mut self, texture_manager: &mut TextureManager) {        
        // self.flattened_lair.reset(0);
        // self.flatten();

        for (id, lair) in &mut self.map_lairs {
            lair.render(texture_manager, self.block_ncd_scale, self.draw_offset);
        }
    }

}


