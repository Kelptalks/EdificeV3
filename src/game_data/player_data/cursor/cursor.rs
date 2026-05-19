use crate::game_data::{World, game_event_manager::event_manager::EventManager, locations::world_area::WorldArea, types::BlockTexture, world::world::WorldEvent};


#[derive(Clone)]
pub struct Cursor {
    world_cords: [i32; 3],
    zoom: f32,

    ghost_block: BlockTexture,

    initialized: bool,

    chunk_load_distance: [i16; 3],
}

impl Cursor {
    pub fn new() -> Cursor{
        Cursor {
            world_cords: [0, 0, -25],
            zoom: 64.0,

            ghost_block: BlockTexture::DroneBotRight,

            initialized: false,

            chunk_load_distance: [5, 5, 2],
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

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_block_ghost(&self) -> BlockTexture {
        return self.ghost_block
    }

    pub fn get_chunk_load_distance(&self) -> [i16; 3] {
        self.chunk_load_distance
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

    pub fn mod_zoom(&mut self, amount: &f32) {
        self.zoom *= amount;
    }

    pub fn set_ghost_block(&mut self, block: BlockTexture) {
        self.ghost_block = block;
    }

    pub fn set_chunk_load_distance(&mut self, distance: [i16; 3]) {
        self.chunk_load_distance = distance;
    }

    //=====================================
    // Tiking
    //=====================================

    fn try_init_height(&mut self, world: &World) {
        let x = self.world_cords[0];
        let y = self.world_cords[1];
        for z in (-200..=50).rev() {
            if world.get_world_value([x, y, z]) != 0 {
                self.world_cords[2] = z + 1;
                self.initialized = true;
                return;
            }
        }
    }

    pub fn tik(&mut self, world: &World, event_manager: &mut EventManager) {
        if !self.initialized {
            self.try_init_height(world);
        }

        let view_distance = self.chunk_load_distance;
        let chunk_cords = World::world_cords_to_chunk_cords(self.get_cords());
        

        for z in -view_distance[2]..=view_distance[2] {
            for y in -view_distance[1]..=view_distance[1] {
                for x in -view_distance[0]..=view_distance[0] {
                    let cords: [i16; 3] = [
                        chunk_cords[0] + x,
                        chunk_cords[1] + y,
                        chunk_cords[2] + z,  
                    ];
                    event_manager.add_world_event(WorldEvent::LoadChunk(cords));
                }
            }
        }
    }
        
    
}