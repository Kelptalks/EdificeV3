
use std::collections::HashMap;

use crate::game_data::{World, game_event_manager::{event_manager::{Event, EventManager}, player_data_event_manager::player_event_manager::PlayerDataEvent}, player_data::{cursor::cursor_event_scheduler::CursorEvent, game_entity::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId}, components::{block_components::moveable_block_component::{DirectionTextures, MoveableBlockComponent, MoveableBlockComponentEvent}, entity_components::{EntityComponent, EntityComponentEvent}, inventory_component::InventoryComponent, powered_component::PoweredComponent, tool_manager_component::ToolManagerComponent}, game_entity_manager::GameEntity, lair_block_entity_manager::{lair_block_entity_manager::LairBlockEntity, lair_block_entitys::blueprint::blueprint::{BluePrintEvent, LairEntityBlueprint}}}}, screen::{ScreenData, widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::PanelColor, widget::WidgetType, widget_calculations::TextSize, world_rendering::world_view_data::WorldViewData}}, tik_manager::game_time::GameTime, tools::id_gen::IdGen, types::BlockTexture, world::world::{SpriteRenderRequest, WorldEvent}};


static ID_GEN: IdGen = IdGen::new();


#[derive(Clone)]
pub struct BlockEntityDrone {
    pub id: u64,

    moveable_block_component: MoveableBlockComponent,
    powered_component: PoweredComponent,
    inventory_component: InventoryComponent,
    tool_manager_component: ToolManagerComponent,
}



impl BlockEntityDrone {

    pub fn wrap_into_game_entity(self) -> GameEntity {
        BlockEntity::Drone(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityDrone {
        let id = ID_GEN.new_id();
        let game_entity_id = BlockEntityId::Drone(id).wrap_into_game_entity_id();

        // Drones only have four sprites; map each to the nearest diagonal and
        // share each across its two adjacent directions. Names don't match the
        // direction scheme yet — best-effort mapping.
        let textures = DirectionTextures::new(
            BlockTexture::DroneUpRight,  // north
            BlockTexture::Debug,  // north_east
            BlockTexture::DroneUpLeft, // east
            BlockTexture::Debug, // south_east
            BlockTexture::DroneBotLeft,  // south
            BlockTexture::Debug,  // south_west
            BlockTexture::DroneBotRight,   // west
            BlockTexture::Debug,   // north_west
        );
        let mut moveable_block_component = MoveableBlockComponent::new(cords, game_entity_id, textures);
        moveable_block_component.init(game_entity_id, event_manager);


        let mut powered_component = PoweredComponent::new(game_entity_id, cords);
        powered_component.max_power = 100000;


        BlockEntityDrone {
            id,
            moveable_block_component,
            powered_component,
            inventory_component: InventoryComponent::new(),
            tool_manager_component: ToolManagerComponent::new(),
        }
    }

    pub fn get_components(self) -> Vec<EntityComponent> {
        vec![
            self.moveable_block_component.wrap_into_component(),
            self.inventory_component.wrap_into_component(),
            self.tool_manager_component.wrap_into_component(),
        ]
    }


    pub fn tik(&mut self, game_time: &GameTime, world: &World, event_manager: &mut EventManager) {
        self.moveable_block_component.tik(game_time, world, event_manager);
    }

    pub fn play_view(&self, world_view_data: &WorldViewData, screen_data: &ScreenData, world: &World, event_manager: &mut EventManager) -> WidgetType {
        // Lock the camera onto the drone.
        let cords = self.moveable_block_component.block().world_cords;
        event_manager.add_event(CursorEvent::SetCords(cords).wrap_into_event());

        // Path target = the block above the first solid block under the cursor.
        // Valid only if that block above is itself not solid (standable). When
        // valid, highlight it and path there on right click.
        if let Some(triangle) = &world_view_data.mouse_triangle {
            if triangle.has_struck_solid {
                let solid = triangle.get_solid_block_struck_cords();
                
                
                
                let mut target = [solid[0], solid[1], solid[2]];
                target[2] += 1;
                if !world.get_world_value_as_block(target).is_solid() {
                    event_manager.add_world_event(WorldEvent::RenderSprite(SpriteRenderRequest {
                        world_pos: [target[0] as f32, target[1] as f32, target[2] as f32],
                        texture: BlockTexture::Selector.wrap_into_texture(),
                    }));


                    if screen_data.was_right_pressed() {
                        let path_event = MoveableBlockComponentEvent::PathTo(target).wrap_into_component_event();
                        event_manager.add_event(DroneEvent::ComponentEvent(path_event).wrap_into_event(self.id));
                    }
                }
            }
        }

        // Overlay: the drone's name, top-center.
        let mut panel = Panel::new_blank();
        panel.set_color(PanelColor::Clear);
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display(format!("Drone {}", self.id)).set_text_scale(TextSize::Medium);
        panel.wrap_into_widget()
    }


}



#[derive(Clone)]
pub enum DroneEvent {
    ComponentEvent(EntityComponentEvent)
}


impl DroneEvent {
    pub fn wrap_into_event(self, id: u64) -> Event {
        BlockEntityEvent::DroneEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, drone: &mut BlockEntityDrone) {
        match self {
            DroneEvent::ComponentEvent(component_event) => {
                match component_event {
                    EntityComponentEvent::Powered(powered_component_event) => {
                        powered_component_event.execute(&mut drone.powered_component);
                    },
                    EntityComponentEvent::Block(block_component_event) => {
                        block_component_event.execute(drone.moveable_block_component.block_mut());
                    },
                    EntityComponentEvent::MoveableBlock(moveable_block_component_event) => {
                        moveable_block_component_event.execute(&mut drone.moveable_block_component);
                    },
                    EntityComponentEvent::Locomotion(_) => {},
                    EntityComponentEvent::Pos(_) => {},
                }

            },
        }
    }
}