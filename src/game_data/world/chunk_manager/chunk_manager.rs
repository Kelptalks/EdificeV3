use std::collections::{HashMap, HashSet};

use crate::game_data::{World, chunk_manager::{lazy_chunk::LazyWorldChunk, unloaded_chunk::UnloadedWorldChunk, loaded_chunk::LoadedWorldChunk}, game_event_manager::event_manager::EventManager, player_data::player_data::PlayerData, tik_manager::game_time::GameTime};



pub enum WorldChunkType {
    Loaded(LoadedWorldChunk),
    Lazy(LazyWorldChunk),
    Unloaded(UnloadedWorldChunk),
}

impl WorldChunkType {
    pub fn get_cords(&self) -> [i16; 3] {
        match self {
            WorldChunkType::Loaded(loaded_world_chunk) => {
                loaded_world_chunk.get_cords()
            },
            WorldChunkType::Lazy(lazy_world_chunk) => {
                lazy_world_chunk.get_cords()
            },
            WorldChunkType::Unloaded(unloaded_chunk) => {
                unloaded_chunk.get_cords()
            },
        }
    }
}

pub struct WorldChunkManager {
    pub chunks: HashMap<u64, WorldChunkType>,

    pub loaded_chunks: HashSet<u64>,

    pub lazy_chunks: HashSet<u64>,
    
    pub unloaded_chunks: HashSet<u64>,
}

impl WorldChunkManager {
    //=====================================
    // Key Functions
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

    
    pub fn new() -> WorldChunkManager {
        WorldChunkManager {
            chunks: HashMap::new(),

            loaded_chunks: HashSet::new(),

            lazy_chunks: HashSet::new(),

            unloaded_chunks: HashSet::new(),
        }
    }

    //=====================================
    // Chunk Getters
    //=====================================
    
    pub fn get_mut_chunk(&mut self, cords: [i16; 3]) -> Option<&mut WorldChunkType> {
        let key = Self::chunk_cords_to_key(cords);
        self.chunks.get_mut(&key)
    }

    pub fn get_mut_loaded_chunk(&mut self, cords: [i16; 3]) -> Option<&mut LoadedWorldChunk> {
        if let Some(WorldChunkType::Loaded(chunk)) = self.get_mut_chunk(cords) {
            return Some(chunk)
        }
        else {
            None
        }
    }


    pub fn get_loaded_chunk(&self, key: &u64) -> Option<&LoadedWorldChunk> {
        if let Some(WorldChunkType::Loaded(chunk)) = self.get_chunk(key) {
            return Some(chunk)
        }
        else {
            None
        }
    }

    pub fn get_loaded_chunk_with_cords(&self, cords: [i16; 3]) -> Option<&LoadedWorldChunk> {
        if let Some(WorldChunkType::Loaded(chunk)) = self.get_chunk_with_cords(cords) {
            return Some(chunk)
        }
        else {
            None
        }
    }


    pub fn get_chunk(&self, key: &u64) -> Option<&WorldChunkType> {
        self.chunks.get(&key)
    }

    pub fn get_chunk_with_cords(&self, cords: [i16; 3]) -> Option<&WorldChunkType> {
        let key = Self::chunk_cords_to_key(cords);
        self.chunks.get(&key)
    }


    
    //=====================================
    // 
    //=====================================

    pub fn set_chunk_load_time(&mut self, cords: &[i16; 3], time: u64) {
        if let Some(chunk_type) = self.get_mut_chunk(*cords) {
            match chunk_type {
                WorldChunkType::Unloaded(unloaded_world_chunk) => {
                    unloaded_world_chunk.load = true;
                },
                WorldChunkType::Loaded(loaded_world_chunk) => {
                    loaded_world_chunk.time_till_unload = time;
                }
                _ => {
                    
                }
            } 
        }
        else {
            let mut chunk = UnloadedWorldChunk::new(*cords);
            chunk.load = true;
            self.add_chunk(chunk.wrap_into_chunk_type());
        }
    }

    pub fn unload_chunk(&mut self, event_manager: &mut EventManager, cords: &[i16; 3]) {
        self.remove_chunk(event_manager, *cords);

    }

    pub fn add_chunk(&mut self, chunk: WorldChunkType) {
        let cords = chunk.get_cords();
        let key = Self::chunk_cords_to_key(cords);

        if !self.chunks.contains_key(&key) {
            match &chunk {
                WorldChunkType::Loaded(_) => {self.loaded_chunks.insert(key);},
                WorldChunkType::Lazy(_) => {self.lazy_chunks.insert(key);},
                WorldChunkType::Unloaded(_) => {self.unloaded_chunks.insert(key);},
            }
            self.chunks.insert(key, chunk);
        }
        else {
            eprintln!("Cannot add chunk to ocupied slot");
        }

        
    }

    pub fn remove_chunk(&mut self, event_manager: &mut EventManager, cords: [i16; 3]) {
        // remove the old chunk from the type hashsets
        if let Some(old_chunk) = self.get_chunk_with_cords(cords) {
            match old_chunk {
                WorldChunkType::Loaded(loaded_world_chunk) => {
                    self.loaded_chunks.remove(&loaded_world_chunk.get_key());
                },
                WorldChunkType::Lazy(lazy_world_chunk) => {
                    self.lazy_chunks.remove(&lazy_world_chunk.get_key());
                },
                WorldChunkType::Unloaded(unloaded_world_chunk) => {
                    self.unloaded_chunks.remove(&unloaded_world_chunk.get_key());
                },
            }
        }

        let key = World::chunk_cords_to_key(cords);
        let removed_chunk = self.chunks.remove(&key);
        if let Some(chunk) = removed_chunk {
            match chunk {
                WorldChunkType::Loaded(loaded_world_chunk) => {
                    event_manager.add_events(&loaded_world_chunk.free());
                },
                WorldChunkType::Lazy(_) => {
                    
                },
                WorldChunkType::Unloaded(_) => {
                    
                },
            }
        }

    }

    pub fn replace_chunk(&mut self, event_manager: &mut EventManager, new_chunk: WorldChunkType) {
        let chunk_cords = new_chunk.get_cords();

        self.remove_chunk(event_manager, chunk_cords);

        self.add_chunk(new_chunk);
    }
    
    
    //=====================================
    // Tik 
    //=====================================

    pub fn tik(&mut self, game_tik: &GameTime, player_data: &PlayerData, event_manager: &mut EventManager) {   
        
        let debug_data = event_manager.get_mut_debug_data();
        debug_data.clear("World");
        debug_data.record("World", format!("total chunks: ({})", self.chunks.len()));
        debug_data.record("World", format!("loaded_chunks: ({})", self.loaded_chunks.len()));
        debug_data.record("World", format!("lazy_chunks: ({})", self.lazy_chunks.len()));
        debug_data.record("World", format!("unloaded_chunks: ({})", self.unloaded_chunks.len()));



        let mut chunks_to_replace: Vec<WorldChunkType> = Vec::new();

        for (_key, chunk_type) in self.chunks.iter_mut() {
            match chunk_type {
                WorldChunkType::Loaded(loaded_world_chunk) => {

                    
                    if game_tik.is_second {
                        if loaded_world_chunk.time_till_unload > 0 {
                            loaded_world_chunk.time_till_unload -= 1;
                        }
                        else {
                            let _key = World::chunk_cords_to_key(loaded_world_chunk.get_cords());
                            let cords = loaded_world_chunk.get_cords();


                            chunks_to_replace.push(UnloadedWorldChunk::new(cords).wrap_into_chunk_type());
                        }
                    }

                },
                WorldChunkType::Lazy(_lazy_world_chunk) => {
                    
                },
                WorldChunkType::Unloaded(unloaded_world_chunk) => {
                    if unloaded_world_chunk.load {
                        let mut world_chunk = LoadedWorldChunk::new(unloaded_world_chunk.get_cords());
                        
                        // Get block mods from naboring chunks generation
                        let mut world_gen_events = Vec::new();
                        world_gen_events.append(&mut unloaded_world_chunk.terrain_gen_events);

                        // Gen terrain over
                        let area = world_chunk.get_world_area();
                        world_gen_events.append(&mut player_data.world_gen.generate_area(area));

                        event_manager.add_world_events(world_gen_events);

                        world_chunk.terrain_generated = true;

                        chunks_to_replace.push(world_chunk.wrap_into_chunk_type());
                    }
                },
            }
        }


        for chunk in chunks_to_replace {
            self.replace_chunk(event_manager, chunk);
        }

        
        

    }

}