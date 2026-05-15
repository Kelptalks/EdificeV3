use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_entity::{components::entity_components::{EntityComponent, EntityComponentEvent}, dynamic_entity_manager::dynamic_entity_manager::DynamicEntityId}, tik_manager::game_time::GameTime, types::BlockTexture, world::world::WorldEvent};

#[derive(Clone)]
pub struct PosComponent {
    id: DynamicEntityId,

    pub pos: [f32; 3],
    pub block_type: BlockTexture,

    pending_move: Option<[f32; 3]>,
    registered_chunk: Option<[i16; 3]>,
}

impl PosComponent {
    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::Pos(self)
    }

    pub fn new(pos: [f32; 3], block_type: BlockTexture, id: DynamicEntityId) -> PosComponent {
        PosComponent {
            id,
            pos,
            block_type,
            pending_move: None,
            registered_chunk: None,
        }
    }

    pub fn init(&mut self, event_manager: &mut EventManager) {
        let chunk = self.current_chunk();
        event_manager.add_world_event(WorldEvent::AddDynamicEntity(chunk, self.id));
        self.registered_chunk = Some(chunk);
    }

    pub fn tik(&mut self, _time: &GameTime, _world: &World, event_manager: &mut EventManager) {
        if let Some(delta) = self.pending_move.take() {
            self.pos[0] += delta[0];
            self.pos[1] += delta[1];
            self.pos[2] += delta[2];
        }

        let current_chunk = self.current_chunk();
        if self.registered_chunk != Some(current_chunk) {
            if let Some(old_chunk) = self.registered_chunk {
                event_manager.add_world_event(WorldEvent::RemoveDynamicEntity(old_chunk, self.id));
            }
            event_manager.add_world_event(WorldEvent::AddDynamicEntity(current_chunk, self.id));
            self.registered_chunk = Some(current_chunk);
        }
    }

    /// Moves by `delta` if the destination block is air. Returns false and leaves
    /// pos unchanged if the destination is solid (caller should cancel velocity).
    pub fn apply_delta(&mut self, delta: [f32; 3], world: &World) -> bool {
        let new_pos = [
            self.pos[0] + delta[0],
            self.pos[1] + delta[1],
            self.pos[2] + delta[2],
        ];
        let new_block = [
            new_pos[0].floor() as i32,
            new_pos[1].floor() as i32,
            new_pos[2].floor() as i32,
        ];
        if world.get_world_value_as_block(new_block) == BlockTexture::Air {
            self.pos = new_pos;
            true
        } else {
            false
        }
    }

    pub fn world_block_cords(&self) -> [i32; 3] {
        [
            self.pos[0].floor() as i32,
            self.pos[1].floor() as i32,
            self.pos[2].floor() as i32,
        ]
    }

    fn current_chunk(&self) -> [i16; 3] {
        World::world_cords_to_chunk_cords(self.world_block_cords())
    }
}



#[derive(Clone)]
pub enum PosComponentEvent {
    Move([f32; 3]),
    SetPos([f32; 3]),
    SetTexture(BlockTexture),
}

impl PosComponentEvent {
    pub fn wrap_into_component_event(self) -> EntityComponentEvent {
        EntityComponentEvent::Pos(self)
    }

    pub fn execute(self, pos: &mut PosComponent) {
        match self {
            PosComponentEvent::Move(delta) => {
                pos.pending_move = Some(delta);
            },
            PosComponentEvent::SetPos(new_pos) => {
                pos.pos = new_pos;
                pos.pending_move = None;
            },
            PosComponentEvent::SetTexture(block_type) => {
                pos.block_type = block_type;
            },
        }
    }
}
