use crate::game_data::{World, game_event_manager::{event_manager::EventManager, world_event_manager::chunk_event::WorldChunkEvent}, player_data::game_entity::components::entity_components::EntityComponent};

#[derive(Clone)]
pub struct VisionComponent {
    chunks_in_view: Vec<[i16; 3]>,
    center_world_cords: [i32; 3],

    pub vision_radius: [i16; 3],
}

impl VisionComponent {

    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::Vision(self)
    }

    pub fn new(world_cords: [i32; 3]) -> VisionComponent {
        let mut vision = VisionComponent {
            chunks_in_view: Vec::new(),
            center_world_cords: world_cords,
            vision_radius: [1; 3]
        };

        vision.set_radius([1; 3]);

        vision
    }

    //=====================================
    // Manipulation
    //=====================================

    fn add_chunk_to_view(&mut self, chunk_cords: [i16; 3]) {
        self.chunks_in_view.push(chunk_cords);
    }

    pub fn chunks_in_view(&self) -> usize {
        self.chunks_in_view.len()
    }

    pub fn mod_range(&mut self, range: [i16; 3]) {
        for i in 0..3 {
            self.vision_radius[i] = (self.vision_radius[i] + range[i]).max(1);
        }
        self.update_chunks_in_view();
    }

    pub fn set_radius(&mut self, range: [i16; 3]) {
        self.vision_radius = range;
        self.update_chunks_in_view();
    }

    pub fn update_chunks_in_view(&mut self) {
        let range = self.vision_radius;
        self.chunks_in_view.clear();
        let chunk_cords = World::world_cords_to_chunk_cords(self.center_world_cords);
        for x in -range[0]..=range[0] {
            for y in -range[1]..=range[1] {
                for z in -range[2]..=range[2] {
                    let chunk_to_load = [
                        chunk_cords[0] + x,
                        chunk_cords[1] + y,
                        chunk_cords[2] + z,
                    ];
                    self.add_chunk_to_view(chunk_to_load);
                }
            }
        }
    }

    //=====================================
    // Ticking
    //=====================================

    pub fn tik(&mut self, event_manager: &mut EventManager) {
        for chunk_cords in &self.chunks_in_view {
            event_manager.add_event(
                WorldChunkEvent::LoadChunk(*chunk_cords).wrap_into_event()
            )
        }
    }
}
