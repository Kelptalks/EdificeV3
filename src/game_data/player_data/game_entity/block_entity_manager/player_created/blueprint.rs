use crate::game_data::{
    World,
    game_event_manager::event_manager::{Event, EventManager},
    player_data::game_entity::{
        block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId},
        components::{
            block_components::block_component::BlockComponent,
            entity_components::{EntityComponent, EntityComponentEvent},
        },
        game_entity_manager::GameEntity,
    },
    screen::{
        ScreenData,
        widget::{
            panel::panel::{Panel, PanelAlignment, PanelOrientation},
            prelude::PanelColor,
            widget::WidgetType,
            widget_calculations::TextSize,
            world_rendering::world_view_data::WorldViewData,
        },
    },
    tik_manager::game_time::GameTime,
    tools::id_gen::IdGen,
    types::BlockTexture,
};

static ID_GEN: IdGen = IdGen::new();

/// A blueprint marker entity. Occupies no visible block (Air) but registers
/// a game entity at its position so constructors can find and build toward it.
#[derive(Clone)]
pub struct BlockEntityBluePrint {
    pub id: u64,
    pub block: BlockComponent,
}

impl BlockEntityBluePrint {
    pub fn wrap_into_game_entity(self) -> GameEntity {
        BlockEntity::BluePrint(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityBluePrint {
        let id = ID_GEN.new_id();
        let game_entity_id = BlockEntityId::BluePrint(id).wrap_into_game_entity_id();

        let mut block = BlockComponent::new(BlockTexture::Air, cords, game_entity_id);
        block.init(game_entity_id, event_manager);

        BlockEntityBluePrint { id, block }
    }

    pub fn tik(&mut self, _game_time: &GameTime, _world: &World, _event_manager: &mut EventManager) {}

    pub fn get_components(self) -> Vec<EntityComponent> {
        vec![self.block.wrap_into_component()]
    }

    pub fn play_view(
        &self,
        _world_view_data: &WorldViewData,
        _screen_data: &ScreenData,
        _world: &World,
        _event_manager: &mut EventManager,
    ) -> WidgetType {
        let mut panel = Panel::new_blank();
        panel.set_color(PanelColor::Clear);
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display(format!("BluePrint {}", self.id)).set_text_scale(TextSize::Medium);
        panel.wrap_into_widget()
    }
}

#[derive(Clone)]
pub enum BluePrintEvent {
    ComponentEvent(EntityComponentEvent),
}

impl BluePrintEvent {
    pub fn wrap_into_event(self, id: u64) -> Event {
        BlockEntityEvent::BluePrintEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, _blueprint: &mut BlockEntityBluePrint) {
        match self {
            BluePrintEvent::ComponentEvent(_) => {}
        }
    }
}
