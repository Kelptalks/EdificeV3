use std::collections::HashMap;

use crate::game_data::screen::{iso_cord_tool, widget::world_rendering::area_rendering_manager::ray_caster::casted_tile::CastedTile};

pub struct TileMap {
    map_root_world_cords: [i32; 3], // Used to convert world cords to map index
    map: HashMap<[i32; 2], CastedTile>,

}



impl TileMap {
    pub fn new(root_world_cords: [i32; 3]) -> TileMap {
        TileMap {
            map_root_world_cords: root_world_cords,
            map: HashMap::new(),
        }
    }

    pub fn get_tile_with_flattened_cords(&self, iso_cords: &[i32; 2]) -> Option<&CastedTile> {
        self.map.get(iso_cords)
    }


    pub fn incert_tile_with_flattened_cords(&mut self, flattened_iso_cords: [i32; 2], tile: CastedTile) {
        self.map.insert(flattened_iso_cords, tile);
    }
    

    pub fn get_tile_at_world_cords(&self, world_cords: &[i32; 3]) -> Option<&CastedTile> {
        let flattened_iso_cords = iso_cord_tool::flatten_world_cords(*world_cords);
        self.get_tile_with_flattened_cords(&flattened_iso_cords)
    }

    pub fn add_tile_with_world_cords(&mut self, world_cords: [i32; 3], tile: CastedTile) {
        let flattened_iso_cords = iso_cord_tool::flatten_world_cords(world_cords); 
        self.incert_tile_with_flattened_cords(flattened_iso_cords, tile);
    }

    pub fn reset(&mut self, root_world_cords: [i32; 3]) {
        self.map_root_world_cords = root_world_cords;

        self.map.clear();
    }

    pub fn get_map(&self) -> &HashMap<[i32; 2], CastedTile> {
        return &self.map
    }

    

}