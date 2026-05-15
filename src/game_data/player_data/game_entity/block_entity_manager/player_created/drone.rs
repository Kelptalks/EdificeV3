
use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_entity::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityId}, components::{block_component::BlockComponent, entity_components::{EntityComponent, EntityComponentEvent}, powered_component::{self, PoweredComponent}}, game_entity_manager::GameEntity}, tik_manager::game_time::GameTime, tools::id_gen::IdGen, types::BlockTexture};


static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct BlockEntityDrone {
    pub id: u64,

    block_component: BlockComponent,
    powered_component: PoweredComponent,
}



impl BlockEntityDrone {

    pub fn wrap_into_game_entity(self) -> GameEntity {
        BlockEntity::Drone(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityDrone {
        let id = ID_GEN.new_id();
        let game_entity_id = BlockEntityId::Drone(id).wrap_into_game_entity_id();

        let mut block_component = BlockComponent::new(BlockTexture::DroneBotRight, cords, game_entity_id);
        block_component.init(game_entity_id, event_manager);

        
        let mut powered_component = PoweredComponent::new(game_entity_id, cords);
        powered_component.max_power = 100000;
        

        BlockEntityDrone {
            id,
            block_component,
            powered_component,
        }
    }
    
    pub fn get_components(self) -> Vec<EntityComponent> {
        vec![
            self.block_component.wrap_into_component()
        ]
    }


    pub fn tik(&mut self, game_time: &GameTime, world: &World, event_manager: &mut EventManager) {
        
    }


}



pub enum DroneEvent {
    ComponentEvent(EntityComponentEvent)
}


impl DroneEvent {
    pub fn execute(self, drone: &mut BlockEntityDrone) {
        match self {
            DroneEvent::ComponentEvent(component_event) => {
                match component_event {
                    EntityComponentEvent::Powered(powered_component_event) => {
                        powered_component_event.execute(&mut drone.powered_component);
                    },
                    EntityComponentEvent::Block(block_component_event) => {
                        block_component_event.execute(&mut drone.block_component);
                    },
                    EntityComponentEvent::Locomotion(_) => {},
                    EntityComponentEvent::Pos(_) => {},
                }

            },
        }
    }
}