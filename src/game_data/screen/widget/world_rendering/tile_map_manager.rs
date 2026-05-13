use std::collections::HashMap;

use crate::game_data::{TextureManager, World, game_event_manager::{self, event_manager::{self, Event, EventManager}}, player_data::player_data::PlayerData, screen::{iso_cord_tool, widget::world_rendering::{area_rendering_manager::ray_caster::casted_tile::CastedTile, tile_map::{self, TileMap, TileMapId}}}, texture_manager::{self, texture::Texture}};


pub struct TileMapManager {
    draw_block_scale: f32,
    draw_offset: [f32; 2],
    
    map_lairs: HashMap<TileMapId, TileMap>,
    
    dirty_maps: Vec<TileMapId>,
    lairs_to_flatten: Vec<TileMapId>,
    
    flattened_lair: HashMap<[i32; 2], Vec<TileMapId>>,

}

impl TileMapManager {
    pub fn new() -> TileMapManager {
        TileMapManager {
            draw_block_scale: 0.0,
            draw_offset: [0.0; 2],

            map_lairs: HashMap::new(),
            
            dirty_maps: Vec::new(),

            lairs_to_flatten: Vec::new(),
            flattened_lair: HashMap::new(),
        }
    }

    pub fn get_block_scale(&self) -> f32 {
        self.draw_block_scale
    }

    pub fn get_draw_offset(&self) -> [f32; 2] {
        self.draw_offset
    }

    pub fn update_rendering_data(&mut self, block_ncd_scale: f32, draw_offset: [f32; 2]) {
        self.draw_block_scale = block_ncd_scale;
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
    
    pub fn get_tile_with_flattened_cords(&mut self, tile_key: &[i32; 2]) -> Option<CastedTile> {

        

        if let Some(lairs_at_cords) = &mut self.flattened_lair.get_mut(tile_key).cloned() {
            // Remove dead lairs in-place, no extra Vec needed
            lairs_at_cords.retain(|id| self.map_lairs.contains_key(id));
            
            let mut option_current_tile: Option<CastedTile> = None;

            for lair_id in lairs_at_cords {
                if let Some(tile_map) = self.get_tile_map(*lair_id) {
                    if let Some(lair_tile) = tile_map.get_tile_with_tile_key(tile_key) {
                        if let Some(current_tile) = &mut option_current_tile {
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
                            option_current_tile = Some(lair_tile.clone());
                        }
                    } 
                }
                else {
                    println!("Dead Lair")
                }
            }
            return option_current_tile;
        }
        None   
        
    }

    //=====================================
    // Map Cleaning
    //=====================================

    pub fn dirty_id(&mut self, id: &TileMapId) {
        self.dirty_maps.push(*id);
    }

    pub fn clean(&mut self, texture_manager: &mut TextureManager, world: &World) {
        if let Some(id) = self.dirty_maps.pop() {
            if let Some(tile_map) = self.map_lairs.get_mut(&id) {
                if !tile_map.clean(world, texture_manager) {
                    self.dirty_maps.push(id);
                }
                else {
                    self.lairs_to_flatten.push(id);
                }
            }
        }
    }

    //=====================================
    // Flattened Lair managment
    //=====================================

    // Compair all the lairs and set each tile to the one with the lowest depth
    pub fn flatten(&mut self) {
        while let Some(id) = self.lairs_to_flatten.pop() {
            // Collect keys first, dropping the borrow on self
            let keys: Vec<_> = self.get_tile_map(id)
                .map(|tm| tm.map.keys().copied().collect())
                .unwrap_or_default();

            for key in keys {
                self.flattened_lair
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(id);
            }
        }
    }

    //=====================================
    // Entity
    //=====================================

    pub fn render_enitity_at_world_pos(
        &mut self, 
        texture_manager: &mut TextureManager, 
        player_data: &PlayerData, 
        world_pos: [f32; 3], 
        texture: Texture
    ) {
        // render the entity texture
        let cursor_cords = player_data.get_cursor().get_cords();
        let offset_world_cords = [
            world_pos[0] - cursor_cords[0] as f32 - 0.25, // add the 0.5 to center on the block
            world_pos[1] - cursor_cords[1] as f32,
            world_pos[2] - cursor_cords[2] as f32,
        ];

        let draw_cords = iso_cord_tool::world_pos_to_ndc_cords(self.draw_block_scale, offset_world_cords);


        // Scale to size of block
        let draw_pos = [
            draw_cords[0] - self.draw_block_scale, 
            draw_cords[1] - self.draw_block_scale,
            draw_cords[0] + self.draw_block_scale, 
            draw_cords[1] + self.draw_block_scale,
        ];
        
        texture_manager.render_texture(texture, draw_pos);

        let sprite_depth = iso_cord_tool::get_depth_from_world_cords(iso_cord_tool::world_pos_to_world_cords(world_pos));
        let flattened_cords = iso_cord_tool::world_pos_to_tile_cords(world_pos);

        for x in -3..=3 {
            for y in -3..=3 {
                let cords = [
                    flattened_cords[0] + x,
                    flattened_cords[1] + y,
                ];
                let tile = self.get_tile_with_flattened_cords(&cords);

                if let Some(tile) = tile {
                    let left_tile_world_cords = tile.get_left_triangle().get_solid_block_struck_cords();
                    let left_tile_depth = iso_cord_tool::get_depth_from_world_cords(left_tile_world_cords);
                    let re_render_left = left_tile_depth > sprite_depth;


                    let right_tile_world_cords = tile.get_right_triangle().get_solid_block_struck_cords();
                    let right_tile_depth = iso_cord_tool::get_depth_from_world_cords(right_tile_world_cords);
                    let re_render_right = right_tile_depth > sprite_depth;


                    if re_render_left {
                        tile.render_left_triangle(
                            texture_manager,
                            self.draw_block_scale,
                            self.draw_offset
                        );
                    }
                    if re_render_right {
                        tile.render_right_triangle(
                            texture_manager,
                            self.draw_block_scale,
                            self.draw_offset
                        );
                    }
                }
            }
        }
    }


    pub fn render_tile_map(&mut self, tile_map_id: TileMapId, texture_manager: &mut TextureManager) {
        let draw_block_scale: f32 =  self.draw_block_scale;
        let draw_offset = self.draw_offset;
        if let Some(tile_map) = self.get_mut_tile_map(tile_map_id) {
            tile_map.render(texture_manager, draw_block_scale, draw_offset);
        }
    }

    pub fn free_id(&mut self, id: &TileMapId) -> Vec<Event> {
        if let Some(mut map) = self.map_lairs.remove(id) {
            map.free()
        }
        else {
            Vec::new()
        }
    }

}


