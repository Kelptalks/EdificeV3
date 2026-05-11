use crate::game_data::{World, game_event_manager::{event_manager::{self, EventManager}, world_event_manager::{chunk_event::WorldChunkEvent, world_event_manager::WorldEvent}}, locations::world_area::WorldArea, types::BlockTexture};


#[derive(Clone)]
pub struct Cursor {
    world_cords: [i32; 3],
    zoom: usize,

    ghost_block: BlockTexture,
}

impl Cursor {
    pub fn new() -> Cursor{
        Cursor {
            world_cords: [0; 3],
            zoom: 5,

            ghost_block: BlockTexture::DroneBotRight,
        }
    }

    //=====================================
    // Getters
    //=====================================

    pub fn get_pos(&self) -> [f32; 3] {
        [
            self.world_cords[0] as f32,
            self.world_cords[1] as f32,
            self.world_cords[2] as f32,
        ]
    }

    pub fn get_cords(&self) -> [i32; 3] {
        self.world_cords
    }

    pub fn get_zoom(&self) -> usize {
        self.zoom
    }

    pub fn get_block_ghost(&self) -> BlockTexture {
        return self.ghost_block
    } 

    pub fn get_rendering_area(&self) -> WorldArea {
        let mut world_area = WorldArea::new_blank();

        let point_1 = self.world_cords.map(|f| f - self.zoom as i32);
        world_area.set_point_1_cords(point_1);

        let point_2 = self.world_cords.map(|f| f + self.zoom as i32);
        world_area.set_point_2_cords(point_2);

        world_area
    }

    //=====================================
    // Mutation
    //=====================================

    pub fn mod_cords(&mut self, cord_mod: &[i32; 3]) {
        self.world_cords[0] += cord_mod[0];
        self.world_cords[1] += cord_mod[1];
        self.world_cords[2] += cord_mod[2];
    }

    pub fn set_cords(&mut self, new_cord: &[i32; 3]) {
        self.world_cords = *new_cord
    }

    pub fn mod_zoom(&mut self, amount: &i32) {
        let new_zoom = self.zoom as i32 + amount;
        if new_zoom > 1 {
            self.zoom = new_zoom as usize;
        }
        else {
            self.zoom = 1;
        }
    }

    pub fn set_ghost_block(&mut self, block: BlockTexture) {
        self.ghost_block = block;
    }

    //=====================================
    // Tiking
    //=====================================

    pub fn tik(&mut self, event_manager: &mut EventManager) {
        // Load a 3 by 3 block area around 
        let view_distance = [3, 3, 3];
        let chunk_cords = World::world_cords_to_chunk_cords(self.get_cords());
        

        for z in -view_distance[2]..=view_distance[2] {
            for y in -view_distance[1]..=view_distance[1] {
                for x in -view_distance[0]..=view_distance[0] {
                    let cords: [i16; 3] = [
                        chunk_cords[0] + x,
                        chunk_cords[1] + y,
                        chunk_cords[2] + z,  
                    ];
                    event_manager.add_world_event(WorldEvent::ChunkEvent(WorldChunkEvent::LoadChunk(cords)));



                }
            }
        }
    }
        
    
}