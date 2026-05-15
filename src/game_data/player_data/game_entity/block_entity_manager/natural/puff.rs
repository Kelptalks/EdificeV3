use crate::game_data::{World, game_event_manager::event_manager::{Event, EventManager}, player_data::game_entity::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId}, components::{entity_components::{EntityComponent, EntityComponentEvent}, locomotion_component::{LocomotionComponent, LocomotionComponentEvent}, pos_component::{PosComponent, PosComponentEvent}}, game_entity_manager::{GameEntity, GameEntityId}}, texture_manager::texture::Texture, tik_manager::game_time::GameTime, tools::{direction_tool::AxisDirection, id_gen::IdGen}, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct BlockEntityPuff {
    pub id: u64,
    pub game_id: GameEntityId,

    pos: PosComponent,
    locomotion: LocomotionComponent,
}

impl BlockEntityPuff {

    pub fn wrap_into_game_entity(self) -> GameEntity {
        BlockEntity::Puff(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3]) -> BlockEntityPuff {
        let id = ID_GEN.new_id();
        let game_entity_id = BlockEntityId::Puff(id).wrap_into_game_entity_id();

        let pos = PosComponent::new(
            [cords[0] as f32, cords[1] as f32, cords[2] as f32],
            BlockTexture::PuffDown,
        );
        let locomotion = LocomotionComponent::new(0.01, 0.15);

        BlockEntityPuff {
            id,
            game_id: game_entity_id,
            pos,
            locomotion,
        }
    }

    pub fn cords(&self) -> [f32; 3] {
        self.pos.pos
    }

    pub fn get_texture(&self) -> Texture {
        self.pos.block_type.wrap_into_texture()
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        if time.is_second {
            let direction = AxisDirection::random();
            let [dx, dy, dz] = direction.get_cords();
            let impulse = [dx as f32 * 0.4, dy as f32 * 0.4, dz as f32 * 0.4];

            event_manager.add_event(
                PuffEvent::ComponentEvent(
                    LocomotionComponentEvent::Accelerate(impulse).wrap_into_component_event()
                ).wrap_into_event(self.id)
            );

            let new_texture = match direction {
                AxisDirection::North => BlockTexture::PuffUpRight,
                AxisDirection::South => BlockTexture::PuffLeftDown,
                AxisDirection::East  => BlockTexture::PuffUpRight,
                AxisDirection::West  => BlockTexture::PuffDownRight,
                AxisDirection::Up    => BlockTexture::PuffUp,
                AxisDirection::Down  => BlockTexture::PuffDown,
            };

            event_manager.add_event(
                PuffEvent::ComponentEvent(
                    PosComponentEvent::SetTexture(new_texture).wrap_into_component_event()
                ).wrap_into_event(self.id)
            );
        }

        let delta = self.locomotion.tik(time);
        self.pos.apply_delta(delta);
        self.pos.tik(time, world, event_manager);
    }

    pub fn get_components(self) -> Vec<EntityComponent> {
        vec![
            self.pos.wrap_into_component(),
            self.locomotion.wrap_into_component(),
        ]
    }
}



#[derive(Clone)]
pub enum PuffEvent {
    ComponentEvent(EntityComponentEvent),
}

impl PuffEvent {

    pub fn wrap_into_event(self, id: u64) -> Event {
        BlockEntityEvent::PuffEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, puff: &mut BlockEntityPuff) {
        match self {
            PuffEvent::ComponentEvent(component_event) => {
                match component_event {
                    EntityComponentEvent::Powered(_) => {},
                    EntityComponentEvent::Block(_) => {},
                    EntityComponentEvent::Locomotion(locomotion_event) => {
                        locomotion_event.execute(&mut puff.locomotion);
                    },
                    EntityComponentEvent::Pos(pos_event) => {
                        pos_event.execute(&mut puff.pos);
                    },
                }
            },
        }
    }
}
