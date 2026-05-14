use crate::game_data::{World, game_event_manager::event_manager::{Event, EventManager}, player_data::game_entity::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId}, components::{block_component::WorldBlockComponent, entity_components::EntityComponent, powered_component::{PoweredComponent, PoweredComponentEvent}}, game_entity_manager::GameEntity}, tik_manager::game_time::GameTime, tools::id_gen::IdGen, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct BlockEntityBattery {
    pub id: u64,

    pub block: WorldBlockComponent,
    pub powered: PoweredComponent,
}

impl BlockEntityBattery {
    pub fn wrap_into_game_entity(self) -> GameEntity {
        BlockEntity::Battery(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityBattery {
        let id = ID_GEN.new_id();
        let game_entity_id = BlockEntityId::Battery(id).wrap_into_game_entity_id();

        let mut block = WorldBlockComponent::new(BlockTexture::Battery1, cords);
        block.give_animation(
            vec![BlockTexture::Battery1, BlockTexture::Battery2, BlockTexture::Battery3, BlockTexture::Battery4]
        );
        block.init(game_entity_id, event_manager);

        let mut powered = PoweredComponent::new(game_entity_id, cords);
        powered.power_stored = 100000;
        powered.max_power = 100000;
        powered.power_demand = 100;

        BlockEntityBattery {
            id,
            block,
            powered,
        }
    }

    pub fn tik(&mut self, game_time: &GameTime, world: &World, event_manager: &mut EventManager) {
        self.powered.tik(game_time, world, event_manager);
    }

    pub fn get_components(self) -> Vec<EntityComponent> {
        let mut components = Vec::new();
        components.push(self.block.wrap_into_component());
        components.push(self.powered.wrap_into_component());
        components
    }
}


/*
###################
## Battery Event ##
###################
*/

#[derive(Clone)]
pub enum BatteryEvent {
    PoweredComponentEvent(PoweredComponentEvent),
}

impl BatteryEvent {
    pub fn wrap_into_event(self, id: u64) -> Event {
        BlockEntityEvent::BatteryEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, battery: &mut BlockEntityBattery) {
        match self {
            BatteryEvent::PoweredComponentEvent(powered_component_event) => {
                powered_component_event.execute(&mut battery.powered);
            },
        }
    }
}
