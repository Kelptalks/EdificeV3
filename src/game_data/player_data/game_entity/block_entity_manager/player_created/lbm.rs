use crate::game_data::{
    World,
    game_event_manager::event_manager::{Event, EventManager},
    player_data::{
        cursor::cursor_event_scheduler::CursorEvent,
        game_entity::{
            block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId},
            components::{
                block_components::block_component::BlockComponent,
                entity_components::{EntityComponent, EntityComponentEvent},
                inventory_component::InventoryComponent,
                powered_component::PoweredComponent,
            },
            game_entity_manager::GameEntity,
        },
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
    world::world::{SpriteRenderRequest, WorldEvent},
};

static ID_GEN: IdGen = IdGen::new();

/// Half-extent of the cube the LBM can reach, in blocks.
const RANGE: i32 = 10;
/// Power drained per mine / place action.
const ACTION_POWER_COST: u32 = 100;

/// Lazer Block Manipulator — a stationary block entity that mines and places
/// blocks within a fixed cube around itself.
#[derive(Clone)]
pub struct BlockEntityLBM {
    pub id: u64,

    pub block: BlockComponent,
    pub powered: PoweredComponent,
    pub inventory: InventoryComponent,

    /// Block type placed on a right-click.
    place_block: BlockTexture,
}

impl BlockEntityLBM {
    pub fn wrap_into_game_entity(self) -> GameEntity {
        BlockEntity::LBM(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityLBM {
        let id = ID_GEN.new_id();
        let game_entity_id = BlockEntityId::LBM(id).wrap_into_game_entity_id();

        let mut block = BlockComponent::new(BlockTexture::LBM, cords, game_entity_id);
        block.init(game_entity_id, event_manager);

        let mut powered = PoweredComponent::new(game_entity_id, cords);
        powered.max_power = 100000;
        powered.power_stored = 100000;

        BlockEntityLBM {
            id,
            block,
            powered,
            inventory: InventoryComponent::new(),
            place_block: BlockTexture::Stone,
        }
    }

    /// True when `cords` is inside the LBM's reach cube.
    fn in_range(&self, cords: [i32; 3]) -> bool {
        let origin = self.block.world_cords;
        (cords[0] - origin[0]).abs() <= RANGE
            && (cords[1] - origin[1]).abs() <= RANGE
            && (cords[2] - origin[2]).abs() <= RANGE
    }

    pub fn tik(&mut self, game_time: &GameTime, world: &World, event_manager: &mut EventManager) {
        self.powered.tik(game_time, world, event_manager);
    }

    pub fn get_components(self) -> Vec<EntityComponent> {
        vec![
            self.block.wrap_into_component(),
            self.powered.wrap_into_component(),
            self.inventory.wrap_into_component(),
        ]
    }

    pub fn play_view(&self, world_view_data: &WorldViewData, screen_data: &ScreenData, world: &World, event_manager: &mut EventManager) -> WidgetType {
        // Lock the camera onto the LBM.
        event_manager.add_event(CursorEvent::SetCords(self.block.world_cords).wrap_into_event());

        if let Some(triangle) = &world_view_data.mouse_triangle {
            if triangle.has_struck_solid {
                let target = triangle.get_solid_block_struck_cords();

                if self.in_range(target) && target != self.block.world_cords {
                    // Highlight the block under the cursor.
                    event_manager.add_world_event(WorldEvent::RenderSprite(SpriteRenderRequest {
                        world_pos: [target[0] as f32, target[1] as f32, target[2] as f32],
                        texture: BlockTexture::Selector.wrap_into_texture(),
                    }));

                    let has_power = self.powered.power_stored >= ACTION_POWER_COST;

                    // Left click: mine the targeted block into the inventory.
                    if screen_data.was_left_pressed() && has_power {
                        let mined = world.get_world_value_as_block(target);
                        event_manager.add_world_event(WorldEvent::ModBlock(target, BlockTexture::Air));
                        event_manager.add_event(LBMEvent::Mine(mined).wrap_into_event(self.id));
                    }

                    // Right click: place a block on top of the targeted block.
                    if screen_data.was_right_pressed() && has_power {
                        let place_target = [target[0], target[1], target[2] + 1];
                        let space_free = !world.get_world_value_as_block(place_target).is_solid();
                        let has_item = self.inventory.inventory().has_item(
                            self.place_block.item(),
                            self.place_block.item_quantity() as i32,
                        );
                        if self.in_range(place_target) && space_free && has_item {
                            event_manager.add_world_event(WorldEvent::ModBlock(place_target, self.place_block));
                            event_manager.add_event(LBMEvent::Place(self.place_block).wrap_into_event(self.id));
                        }
                    }
                }
            }
        }

        // Overlay: the LBM's name, top-center.
        let mut panel = Panel::new_blank();
        panel.set_color(PanelColor::Clear);
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display(format!("LBM {}", self.id)).set_text_scale(TextSize::Medium);
        panel.wrap_into_widget()
    }
}


#[derive(Clone)]
pub enum LBMEvent {
    ComponentEvent(EntityComponentEvent),
    Mine(BlockTexture),
    Place(BlockTexture),
}

impl LBMEvent {
    pub fn wrap_into_event(self, id: u64) -> Event {
        BlockEntityEvent::LBMEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, lbm: &mut BlockEntityLBM) {
        match self {
            LBMEvent::ComponentEvent(component_event) => {
                match component_event {
                    EntityComponentEvent::Powered(powered_event) => {
                        powered_event.execute(&mut lbm.powered);
                    },
                    EntityComponentEvent::Block(_) => {},
                    EntityComponentEvent::MoveableBlock(_) => {},
                    EntityComponentEvent::Locomotion(_) => {},
                    EntityComponentEvent::Pos(_) => {},
                }
            },
            LBMEvent::Mine(block) => {
                lbm.inventory.inventory_mut().add_item(block.item(), block.item_quantity() as i32);
                lbm.powered.power_stored = lbm.powered.power_stored.saturating_sub(ACTION_POWER_COST);
            },
            LBMEvent::Place(block) => {
                lbm.inventory.inventory_mut().remove_item(block.item(), block.item_quantity() as i32);
                lbm.powered.power_stored = lbm.powered.power_stored.saturating_sub(ACTION_POWER_COST);
            },
        }
    }
}
