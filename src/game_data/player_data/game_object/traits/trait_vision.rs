use crate::game_data::{World, game_event_manager::{event_manager::EventManager, world_event_manager::chunk_event::WorldChunkEvent}};

#[derive(Clone)]
pub struct VisionTrait {
    chunks_in_view: Vec<[i16; 3]>
}

impl VisionTrait {
    pub fn new() -> VisionTrait {
        VisionTrait {
            chunks_in_view: Vec::new(),
        }
    }

    pub fn add_chunk_to_view(&mut self, chunk_cords: [i16; 3]) {
        self.chunks_in_view.push(chunk_cords);
    }

    pub fn tik(&mut self, event_manager: &mut EventManager) {
        for chunk_cords in &self.chunks_in_view {
            event_manager.add_event(
                WorldChunkEvent::LoadChunk(*chunk_cords).wrap_into_event()
            )
        }

    }
}