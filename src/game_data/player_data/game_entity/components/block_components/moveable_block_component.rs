use crate::game_data::{
    World,
    game_event_manager::event_manager::EventManager,
    player_data::{
        drones::drone_actions::advanced_actions::path_planner::get_path,
        game_entity::{
            components::{
                block_components::block_component::{BlockComponent, BlockComponentEvent},
                entity_components::{EntityComponent, EntityComponentEvent},
            },
            game_entity_manager::GameEntityId,
        },
    },
    tik_manager::game_time::GameTime,
    tools::direction_tool::AxisDirection,
    types::BlockTexture,
};

/// One texture per horizontal facing. Up/Down have no facing texture.
#[derive(Clone)]
pub struct DirectionTextures {
    pub north: BlockTexture,
    pub north_east: BlockTexture,
    pub east: BlockTexture,
    pub south_east: BlockTexture,
    pub south: BlockTexture,
    pub south_west: BlockTexture,
    pub west: BlockTexture,
    pub north_west: BlockTexture,
}

impl DirectionTextures {
    pub fn new(
        north: BlockTexture,
        north_east: BlockTexture,
        east: BlockTexture,
        south_east: BlockTexture,
        south: BlockTexture,
        south_west: BlockTexture,
        west: BlockTexture,
        north_west: BlockTexture,
    ) -> DirectionTextures {
        DirectionTextures {
            north,
            north_east,
            east,
            south_east,
            south,
            south_west,
            west,
            north_west,
        }
    }

    /// All eight directions use BlockTexture::Debug. Placeholder until real art is wired in.
    pub fn debug() -> DirectionTextures {
        DirectionTextures {
            north: BlockTexture::Debug,
            north_east: BlockTexture::Debug,
            east: BlockTexture::Debug,
            south_east: BlockTexture::Debug,
            south: BlockTexture::Debug,
            south_west: BlockTexture::Debug,
            west: BlockTexture::Debug,
            north_west: BlockTexture::Debug,
        }
    }

    /// Texture for a horizontal facing. Up/Down return None (no facing texture).
    pub fn get(&self, direction: &AxisDirection) -> Option<BlockTexture> {
        match direction {
            AxisDirection::North     => Some(self.north),
            AxisDirection::NorthEast => Some(self.north_east),
            AxisDirection::East      => Some(self.east),
            AxisDirection::SouthEast => Some(self.south_east),
            AxisDirection::South     => Some(self.south),
            AxisDirection::SouthWest => Some(self.south_west),
            AxisDirection::West      => Some(self.west),
            AxisDirection::NorthWest => Some(self.north_west),
            AxisDirection::Up | AxisDirection::Down => None,
        }
    }
}

/// A block-backed entity component that manages its own block, swapping the
/// rendered texture to match its current facing and walking planned paths.
#[derive(Clone)]
pub struct MoveableBlockComponent {
    block: BlockComponent,
    textures: DirectionTextures,

    /// A goal requested via PathTo, planned into `path` on the next tik.
    pending_goal: Option<[i32; 3]>,
    /// Remaining path steps, in reverse order — pop from the end to walk it.
    path: Vec<[i32; 3]>,
}

impl MoveableBlockComponent {
    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::MoveableBlock(self)
    }

    pub fn new(
        cords: [i32; 3],
        id: GameEntityId,
        textures: DirectionTextures,
    ) -> MoveableBlockComponent {
        let block = BlockComponent::new(textures.east, cords, id);
        MoveableBlockComponent {
            block,
            textures,
            pending_goal: None,
            path: Vec::new(),
        }
    }

    pub fn block(&self) -> &BlockComponent {
        &self.block
    }

    pub fn block_mut(&mut self) -> &mut BlockComponent {
        &mut self.block
    }

    /// Swaps the block texture to match a horizontal facing. Up/Down keep the
    /// current texture. Only updates the block component's texture field — the
    /// block component emits the world update itself when it tiks, so painting
    /// here would fight its move repaint and leave the old block behind.
    pub fn face(&mut self, direction: AxisDirection) {
        if let Some(texture) = self.textures.get(&direction) {
            self.block.block_type = texture;
        }
    }

    pub fn init(&mut self, id: GameEntityId, event_manager: &mut EventManager) {
        self.block.init(id, event_manager);
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        // Plan a path when one has been requested.
        if let Some(goal) = self.pending_goal.take() {
            self.path = get_path(world, self.block.world_cords, goal);
        }

        // Walk one path step per second.
        if time.is_second {
            if let Some(step) = self.path.pop() {
                BlockComponentEvent::Move(step).execute(&mut self.block);
                if step[0] != 0 || step[1] != 0 {
                    let dir = AxisDirection::from_impulse([step[0] as f32, step[1] as f32, 0.0]);
                    self.face(dir);
                }
            }
        }

        self.block.tik(time, world, event_manager);
    }
}

#[derive(Clone)]
pub enum MoveableBlockComponentEvent {
    /// Requests the component plan and walk a path to the given world cords.
    PathTo([i32; 3]),
}

impl MoveableBlockComponentEvent {
    pub fn wrap_into_component_event(self) -> EntityComponentEvent {
        EntityComponentEvent::MoveableBlock(self)
    }

    pub fn execute(self, component: &mut MoveableBlockComponent) {
        match self {
            MoveableBlockComponentEvent::PathTo(goal) => {
                component.pending_goal = Some(goal);
            }
        }
    }
}
