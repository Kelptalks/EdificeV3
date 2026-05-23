#![allow(dead_code)]



use crate::game_data::chunk_manager::chunk_manager::{WorldChunkManager, WorldChunkType};
use crate::game_data::chunk_manager::loaded_chunk::{LoadedWorldChunk, WorldChunkEvent};
use crate::game_data::chunk_tile_map_manager::chunk_render_data::ChunkRenderData;
use crate::game_data::chunk_tile_map_manager::chunk_tile_set_manager::ChunkTileSetManager;
use crate::game_data::chunk_tile_map_manager::chunk_tile_set::{self, ChunkTileSet};
use crate::game_data::locations::world_area::WorldArea;
use crate::game_data::game_event_manager::event_manager::{Event, EventManager};
use crate::game_data::game_event_manager::game_event_manager::{GameEvent, GameEventManager};
use crate::game_data::game_event_manager::render_event_manager::render_event_manager::RenderEvent;
use crate::game_data::player_data::game_entity::{dynamic_entity_manager::dynamic_entity_manager::DynamicEntityId, game_entity_manager::GameEntityId};
use crate::game_data::player_data::player_data::PlayerData;
use crate::game_data::texture_manager::texture::Texture;
use crate::game_data::tik_manager::game_time::GameTime;
use crate::game_data::tools::iso_cord_tool::{self, flatten_world_cords};
use crate::game_data::types::BlockTexture;
use crate::game_data::world_gen::WorldGenManager;
use crate::game_data::{TextureManager};




#[derive(Clone, Copy)]
pub struct SpriteRenderRequest {
    pub world_pos: [f32; 3],
    pub texture: Texture,
}

pub struct World {
    pub world_chunk_manager: WorldChunkManager,
    pub chunk_tile_set_manager: ChunkTileSetManager,

    total_chunks : u32,

    world_gen_manager: WorldGenManager,

    sprite_render_queue: Vec<SpriteRenderRequest>,
}

impl World {
    pub fn new() -> Self
    {
        Self {
            world_chunk_manager: WorldChunkManager::new(),
            chunk_tile_set_manager: ChunkTileSetManager::new(),

            total_chunks: 0,
            world_gen_manager: WorldGenManager::new(),

            sprite_render_queue: Vec::new(),
        }
    }

    pub fn queue_sprite_render(&mut self, req: SpriteRenderRequest) {
        self.sprite_render_queue.push(req);
    }

    fn drain_sprite_render_queue(&mut self, texture_manager: &mut TextureManager, render_data: &ChunkRenderData) {
        let requests: Vec<SpriteRenderRequest> = self.sprite_render_queue.drain(..).collect();
        for req in requests {
            self.render_sprite_at_world_pos(texture_manager, render_data, req.world_pos, req.texture, 3, 2);
        }
    }


    //=====================================
    // Conversion Funcions
    //=====================================

    pub fn chunk_cords_to_key(cords : [i16 ; 3]) -> u64
    {
        // Cast through u16 to preserve bit pattern without sign extension
        let x = cords[0] as u16 as u64;
        let y = cords[1] as u16 as u64;
        let z = cords[2] as u16 as u64;
        
        return (z << 32) | (y << 16) | x
    }

    pub fn key_to_chunk_cords(key: u64) -> [i16; 3] {
        let x = (key & 0xFFFF) as u16 as i16;
        let y = ((key >> 16) & 0xFFFF) as u16 as i16;
        let z = ((key >> 32) & 0xFFFF) as u16 as i16;
        [x, y, z]
    }

    pub fn world_cords_to_chunk_cords(cords : [i32 ; 3]) -> [i16; 3]
    {
        let chunk_size = LoadedWorldChunk::get_chunk_size_i32();
    
        // Use div_euclid for proper floor division
        let chunk_x = cords[0].div_euclid(chunk_size) as i16;
        let chunk_y = cords[1].div_euclid(chunk_size) as i16;
        let chunk_z = cords[2].div_euclid(chunk_size) as i16;
        
        [chunk_x, chunk_y, chunk_z]
    }


    //=====================================
    // Chunk Getters / Setters
    //=====================================

    
    pub fn create_chunk(&mut self, key: u64) {
        let cords = Self::key_to_chunk_cords(key);
        let world_chunk = LoadedWorldChunk::new(cords);
        self.world_chunk_manager.add_chunk(world_chunk.wrap_into_chunk_type());
    }


    pub fn set_chunk_block_data(&mut self, key: u64, block_data: Box<[u16; crate::game_data::chunk_manager::loaded_chunk::CHUNK_VOLUME]>) {
        let cords = Self::key_to_chunk_cords(key);
        if self.world_chunk_manager.get_mut_loaded_chunk(cords).is_none() {
            self.create_chunk(key);
        }
        if let Some(chunk) = self.world_chunk_manager.get_mut_loaded_chunk(cords) {
            chunk.set_block_data(block_data);
        }
    }

    //=====================================
    // Single Block
    //=====================================

    // Get the modded world cords that give the local chunk cords
    pub fn world_cords_to_internal_chunk_cords(cords : [i32 ; 3]) -> [usize; 3]
    {
        // Mod cords to get internal chunk cords 
        let chunk_size = LoadedWorldChunk::get_chunk_size_i32();
        // Use rem_euclid for proper modulo that handles negatives correctly
        let x_moded_cord = cords[0].rem_euclid(chunk_size) as usize;
        let y_moded_cord = cords[1].rem_euclid(chunk_size) as usize;
        let z_moded_cord = cords[2].rem_euclid(chunk_size) as usize;


        return [x_moded_cord, y_moded_cord, z_moded_cord];
    }

    pub fn set_world_value(&mut self, value : u16, world_cords : [i32 ; 3]) {
        
        let chunk_cords = Self::world_cords_to_chunk_cords(world_cords);

        if let Some(chunk_type) = self.world_chunk_manager.get_mut_chunk(chunk_cords) {
            match chunk_type {
                WorldChunkType::Loaded(loaded_world_chunk) => {
                    let internal_chunk_cords = Self::world_cords_to_internal_chunk_cords(world_cords);
                    loaded_world_chunk.set_chunk_value(value, internal_chunk_cords);
                },
                WorldChunkType::Lazy(_lazy_world_chunk) => {
                    todo!("How does a lazy chunk exist");
                },
                WorldChunkType::Unloaded(unloaded_world_chunk) => {
                    unloaded_world_chunk.terrain_gen_events.push(
                        WorldEvent::ModBlock(
                            world_cords, 
                            BlockTexture::from_id(value)
                        )
                    );
                },
            }
        }
        else {
            self.world_chunk_manager.set_chunk_load_time(&chunk_cords, 100);
            self.set_world_value(value, world_cords);
        }
    }

    pub fn get_world_value (&self, cords : [i32 ; 3]) -> u16
    {   
        let chunk_cords = Self::world_cords_to_chunk_cords(cords);
        if let Some(loaded_chunk) = self.world_chunk_manager.get_loaded_chunk_with_cords(chunk_cords) {
            let internal_chunk_cords = Self::world_cords_to_internal_chunk_cords(cords);
            return loaded_chunk.get_chunk_value(internal_chunk_cords)
        }
        else {
            return 0;
        }

    }

    pub fn get_world_value_as_block(&self, cords : [i32 ; 3]) -> BlockTexture {
        BlockTexture::from_id(self.get_world_value(cords))
    }

    pub fn world_snapshot(&self, world_area: WorldArea) -> World {
        let mut snapshot = World::new();
        for chunk_type in self.world_chunk_manager.chunks.values() {
            if let WorldChunkType::Loaded(chunk) = chunk_type {
                if chunk.overlaps_world_area(&world_area) {
                    let mut new_chunk = LoadedWorldChunk::new(chunk.get_cords());
                    new_chunk.set_block_data(chunk.clone_block_data());
                    snapshot.world_chunk_manager.add_chunk(new_chunk.wrap_into_chunk_type());
                }
            }
        }
        snapshot
    }


    pub fn get_block_entity(&self, cords: [i32; 3]) -> Option<GameEntityId> {
        let chunk_cords = Self::world_cords_to_chunk_cords(cords);
        if let Some(chunk) = self.world_chunk_manager.get_loaded_chunk_with_cords(chunk_cords) {
            chunk.block_entities.get(&cords).cloned()
        } else {
            None
        }
    }

    //=====================================
    // Tik
    //=====================================


    pub fn tik(&mut self, game_time: &GameTime, player_data: &PlayerData, event_manager: &mut EventManager) {   
        // Collect Debug Data

        self.world_chunk_manager.tik(game_time, player_data, event_manager);

    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_sprite_at_world_pos(
        &self,
        texture_manager: &mut TextureManager,
        render_data: &ChunkRenderData,
        world_pos: [f32; 3],
        texture: Texture,
        num_lairs: i16,
        area_radius: i32,
    ) {
        let ndc = iso_cord_tool::world_pos_to_ndc_cords(render_data.scale, world_pos);
        let draw_pos = [
            ndc[0] + render_data.offset[0],
            ndc[1] + render_data.offset[1],
            ndc[0] + render_data.offset[0] + render_data.scale * 2.0,
            ndc[1] + render_data.offset[1] + render_data.scale * 2.0,
        ];
        texture_manager.render_texture(texture, draw_pos);

        let sprite_depth = world_pos[0] as i32 + world_pos[1] as i32 + world_pos[2] as i32;
        let base_cords = [
            world_pos[0].round() as i32,
            world_pos[1].round() as i32,
            world_pos[2].round() as i32,
        ];

        for dx in -area_radius..=area_radius {
            for dy in -area_radius..=area_radius {
                let world_cords = [base_cords[0] + dx, base_cords[1] + dy, base_cords[2]];
                let tiles = self.chunk_tile_set_manager.get_obscuring(world_cords, num_lairs);
                for tile in &tiles {
                    let [left_depth, right_depth] = tile.get_triangles_depths();
                    if left_depth > sprite_depth {
                        tile.render_left_triangle(texture_manager, render_data.scale, render_data.offset);
                    }
                    if right_depth > sprite_depth {
                        tile.render_right_triangle(texture_manager, render_data.scale, render_data.offset);
                    }
                }
            }
        }
    }

    pub fn render_world(
        &mut self,
        player_data: &PlayerData,
        texture_manager: &mut TextureManager,
        render_data: &ChunkRenderData,
    ) {
        let loaded_chunks: Vec<u64> = self.world_chunk_manager.loaded_chunks.iter().copied().collect();
        
        self.chunk_tile_set_manager.clean(&self.world_chunk_manager, texture_manager);
        self.chunk_tile_set_manager.render(texture_manager, render_data);


        let cursor = player_data.get_cursor();
        let cursor_pos = cursor.get_pos();
        self.render_sprite_at_world_pos(
            texture_manager,
            render_data,
            cursor_pos,
            Texture::BlockTexture(BlockTexture::Selector),
            3,
            2,
        );


        for (pos, texture) in player_data.game_entity_manager.iter_dynamic_world_pos_and_texture() {
            self.render_sprite_at_world_pos(
                texture_manager,
                render_data,
                pos,
                texture,
                3,
                2,
            );
        }

        for key in loaded_chunks {
            if let Some(WorldChunkType::Loaded(chunk)) = self.world_chunk_manager.get_chunk(&key) {
                if self.chunk_tile_set_manager.get_set(&key).is_none() {
                    let chunk_tile_set = ChunkTileSet::new(chunk);
                    self.chunk_tile_set_manager.add_chunk_tile_set(chunk_tile_set, &key);
                }
            }
            else {
                eprintln!("NON LOADED OR NULL CHUNKS SHOULD NOT EXIST HERE");
            }
        }

        self.drain_sprite_render_queue(texture_manager, render_data);
    }

    pub fn get_chunk_debug_data(&self, world_cords: [i32; 3]) -> Vec<String> {
        let mut data = Vec::new();
        let chunk_cords = Self::world_cords_to_chunk_cords(world_cords);
        if let Some(chunk) = self.world_chunk_manager.get_loaded_chunk_with_cords(chunk_cords) {
            data.push(format!("Chunk Cords: ({:?})", chunk.get_cords()));
            data.push(format!("Chunk Dirty: {}", chunk.dirty         ));
            data.push(format!("Block Entities: {}", chunk.block_entities.len()));
            data.push(format!("Dynamic Entities: {}", chunk.dynamic_entities.len()));
            data.push(format!("Time till unload: {}", chunk.time_till_unload));
            
        }
        else {
            data.push(format!("NO CHUNK FOUND AT ({:?})", world_cords));
        }
        data
    }

}





/*
#################
## World Event ##
#################
Enum and logic for executing events pertaining 
to modifications to the world
*/

#[derive(Clone)]
pub enum WorldEvent {
    // Direct
    Clear,
    
    
    // Chunk
    LoadedChunkEvent([i16; 3], WorldChunkEvent),
    LoadChunk([i16; 3]),
    SetChunkLoadTime([i16; 3], u64),

    // Modifcation
    ModBlock([i32; 3], BlockTexture),                                   // Params: (Block Cords, Block Type) | Modify a block in the world
    ReplaceBlock([i32; 3], BlockTexture),                               // Only sets block if it is currently air (0)
    FillChunk([i16; 3], BlockTexture),                                  // Fill entire chunk with one block type
                                  
    // Block entities (grid-anchored, one per cell)
    AddGameEntity([i32; 3], GameEntityId),
    RemoveGameEntity([i32; 3]),

    // Dynamic entities (free-floating, many per chunk)
    AddDynamicEntity([i16; 3], DynamicEntityId),
    RemoveDynamicEntity([i16; 3], DynamicEntityId),

    /// Unload the chunk containing this world coord and free its tile set.
    UnloadChunkAtCords([i32; 3]),
    
    /// Toggle Selector-block border overlay on all chunk tile sets.
    ToggleChunkBorders,
    
    /// Free the tile set for a chunk that has been unloaded (keyed by chunk key).
    FreeChunkTileSet(u64),

    /// Queue a sprite to be rendered with correct occlusion this frame.
    RenderSprite(SpriteRenderRequest),
}

impl WorldEvent {

    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::WorldEvent(self));
    }

    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        return vec![self.wrap_into_event()];
    }

    //=====================================
    // Execution
    //=====================================
    pub fn execute_world_event(self, world: &mut World, event_manager: &mut GameEventManager) {
        match self {
            WorldEvent::Clear => {
                eprintln!("Clear World not implemented");
            },
            
            // Chunk
            WorldEvent::LoadedChunkEvent(cords, chunk_event) => {
                if let Some(chunk) = world.world_chunk_manager.get_mut_loaded_chunk(cords) { 
                    chunk_event.execute_chunk_event(chunk, event_manager);
                }
            }
            WorldEvent::LoadChunk(cords) => {
                world.world_chunk_manager.set_chunk_load_time(&cords, 1);
            },
            WorldEvent::SetChunkLoadTime(_cords, _time) => {
                
            }
            WorldEvent::ModBlock(cords, block_type) => {
                world.set_world_value(block_type.id_as_u16(), cords);
                let key = World::chunk_cords_to_key(World::world_cords_to_chunk_cords(cords));
                world.chunk_tile_set_manager.queue_dirty_chunk(key);
                event_manager.add_render_event(RenderEvent::ReRenderBlock(cords));
            }
            WorldEvent::ReplaceBlock(cords, block_type) => {
                if world.get_world_value(cords) == 0 {
                    world.set_world_value(block_type.id_as_u16(), cords);
                    let key = World::chunk_cords_to_key(World::world_cords_to_chunk_cords(cords));
                    world.chunk_tile_set_manager.queue_dirty_chunk(key);
                    event_manager.add_render_event(RenderEvent::ReRenderBlock(cords));
                }
            }
            WorldEvent::FillChunk(chunk_cords, block_type) => {
                if let Some(chunk) = world.world_chunk_manager.get_mut_loaded_chunk(chunk_cords) {
                    chunk.fill(block_type.id_as_u16());
                    chunk.dirty = true;
                }
            }
            WorldEvent::AddGameEntity(world_cords, game_entity_id) => {
                let chunk_cords = World::world_cords_to_chunk_cords(world_cords);
                if let Some(chunk) = world.world_chunk_manager.get_mut_loaded_chunk(chunk_cords) {
                    chunk.block_entities.insert(world_cords, game_entity_id);
                }
            },
            WorldEvent::RemoveGameEntity(world_cords) => {
                let chunk_cords = World::world_cords_to_chunk_cords(world_cords);
                if let Some(chunk) = world.world_chunk_manager.get_mut_loaded_chunk(chunk_cords) {
                    chunk.block_entities.remove(&world_cords);
                }
            },
            WorldEvent::AddDynamicEntity(chunk_cords, entity_id) => {
                if let Some(chunk) = world.world_chunk_manager.get_mut_loaded_chunk(chunk_cords) {
                    chunk.dynamic_entities.insert(entity_id);
                }
            },
            WorldEvent::RemoveDynamicEntity(chunk_cords, entity_id) => {
                if let Some(chunk) = world.world_chunk_manager.get_mut_loaded_chunk(chunk_cords) {
                    chunk.dynamic_entities.remove(&entity_id);
                }
            },
            WorldEvent::UnloadChunkAtCords(cords) => {
                let chunk_cords = World::world_cords_to_chunk_cords(cords);
                let key = World::chunk_cords_to_key(chunk_cords);
                world.world_chunk_manager.chunks.remove(&key);
                world.world_chunk_manager.loaded_chunks.remove(&key);
                world.chunk_tile_set_manager.queue_free_chunk(key);
            },
            WorldEvent::ToggleChunkBorders => {
                world.chunk_tile_set_manager.toggle_borders();
            },
            WorldEvent::FreeChunkTileSet(key) => {
                world.chunk_tile_set_manager.queue_free_chunk(key);
            },
            WorldEvent::RenderSprite(req) => {
                world.queue_sprite_render(req);
            },
        }
    }
}
