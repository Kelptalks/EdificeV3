
use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_entity::{components::{entity_components::EntityComponent, locomotion_component::LocomotionComponent, pos_component::PosComponent}, dynamic_entity_manager::dynamic_entity_manager::{DynamicEntity, DynamicEntityId}, game_entity_manager::{GameEntity, GameEntityId}}, texture_manager::texture::Texture, tik_manager::game_time::GameTime, tools::{direction_tool::AxisDirection, id_gen::IdGen}, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct DynamicEntityPuff {
    pub id: u64,
    pub game_id: GameEntityId,

    pos: PosComponent,
    locomotion: LocomotionComponent,
}

impl DynamicEntityPuff {

    pub fn wrap_into_game_entity(self) -> GameEntity {
        DynamicEntity::Puff(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> DynamicEntityPuff {
        let id = ID_GEN.new_id();
        let game_entity_id = DynamicEntityId::Puff(id).wrap_into_game_entity_id();

        let dynamic_id = DynamicEntityId::Puff(id);
        let mut pos = PosComponent::new(
            [cords[0] as f32, cords[1] as f32, cords[2] as f32],
            BlockTexture::PuffEast,
            dynamic_id,
        );
        pos.init(event_manager);

        let locomotion = LocomotionComponent::new(0.0001, 0.01);

        DynamicEntityPuff {
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
        
        
        let cords = self.pos.world_block_cords();
        let five_below_solid = (1..=5).all(|dz| {
            world.get_world_value_as_block([cords[0], cords[1], cords[2] - dz]) != BlockTexture::Air
        });
        if five_below_solid {
            self.locomotion.apply_impulse([0.0, 0.0, 0.002]);
        }
        else {
            self.locomotion.apply_impulse([0.0, 0.0, -0.001]);
        }
        

        if !self.locomotion.is_moving() {
            if time.is_second && time.second % 5 == 0 {
                let impulse = [
                    (rand::random::<f32>() - 0.5) * 0.2,
                    (rand::random::<f32>() - 0.5) * 0.2,
                    0.0,
                ];
                self.locomotion.apply_impulse(impulse);

                let dir = AxisDirection::from_impulse(impulse);
                match dir {
                    AxisDirection::Up | AxisDirection::Down => {},
                    dir => {
                        self.pos.block_type = match dir {
                            AxisDirection::North     => BlockTexture::PuffNorth,
                            AxisDirection::NorthEast => BlockTexture::PuffNorthEast,
                            AxisDirection::East      => BlockTexture::PuffEast,
                            AxisDirection::SouthEast => BlockTexture::PuffSouthEast,
                            AxisDirection::South     => BlockTexture::PuffSouth,
                            AxisDirection::SouthWest => BlockTexture::PuffSouthWest,
                            AxisDirection::West      => BlockTexture::PuffWest,
                            AxisDirection::NorthWest => BlockTexture::PuffNorthWest,
                            _ => unreachable!(),
                        };
                    }
                }
            }
        }

        self.locomotion.tik(time, &mut self.pos, world);
        self.pos.tik(time, world, event_manager);
    }

    pub fn get_components(self) -> Vec<EntityComponent> {
        vec![
            self.pos.wrap_into_component(),
            self.locomotion.wrap_into_component(),
        ]
    }
}
