use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    World,
    game_event_manager::{
        event_manager::EventManager,
        prelude::WidgetEvent,
    },
    player_data::game_entity::{
        components::{entity_components::EntityComponent, pos_component::PosComponent},
        dynamic_entity_manager::dynamic_entity_manager::{DynamicEntity, DynamicEntityId},
        game_entity_manager::{GameEntity, GameEntityId},
    },
    screen::widget::{
        panel::panel::{Panel, PanelAlignment, PanelOrientation},
        widget::WidgetType,
        widget_calculations::TextSize,
    },
    texture_manager::texture::Texture,
    tik_manager::game_time::GameTime,
    tools::id_gen::IdGen,
    types::BlockTexture,
    world::world::WorldEvent,
};

static ID_GEN: IdGen = IdGen::new();

#[derive(Clone)]
pub struct CursorEntity {
    pub id: u64,
    pub game_id: GameEntityId,

    pub x: Rc<RefCell<f32>>,
    pub y: Rc<RefCell<f32>>,
    pub z: Rc<RefCell<f32>>,
    pub zoom: Rc<RefCell<f32>>,

    pos: PosComponent,
}

impl CursorEntity {
    pub fn new(start: [i32; 3], event_manager: &mut EventManager) -> Self {
        let id = ID_GEN.new_id();
        let dynamic_id = DynamicEntityId::CursorEntity(id);
        let game_id = dynamic_id.wrap_into_game_entity_id();

        let start_f = [start[0] as f32, start[1] as f32, start[2] as f32];
        let mut pos = PosComponent::new(start_f, BlockTexture::DroneBotRight, dynamic_id);
        pos.init(event_manager);

        Self {
            id,
            game_id,
            x: Rc::new(RefCell::new(start_f[0])),
            y: Rc::new(RefCell::new(start_f[1])),
            z: Rc::new(RefCell::new(start_f[2])),
            zoom: Rc::new(RefCell::new(64.0)),
            pos,
        }
    }

    pub fn wrap_into_game_entity(self) -> GameEntity {
        DynamicEntity::CursorEntity(self).wrap_into_game_entity()
    }

    pub fn world_pos(&self) -> [f32; 3] {
        self.pos.pos
    }

    pub fn get_zoom(&self) -> f32 {
        *self.zoom.borrow()
    }

    pub fn texture(&self) -> Texture {
        self.pos.block_type.wrap_into_texture()
    }

    pub fn get_components(self) -> Vec<EntityComponent> {
        vec![self.pos.wrap_into_component()]
    }

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        // Apply any Modf32Event changes into PosComponent
        let tx = *self.x.borrow();
        let ty = *self.y.borrow();
        let tz = *self.z.borrow();
        let delta = [
            tx - self.pos.pos[0],
            ty - self.pos.pos[1],
            tz - self.pos.pos[2],
        ];
        if delta[0] != 0.0 || delta[1] != 0.0 || delta[2] != 0.0 {
            self.pos.apply_delta(delta, world);
            // Sync refs back in case movement was blocked by terrain
            *self.x.borrow_mut() = self.pos.pos[0];
            *self.y.borrow_mut() = self.pos.pos[1];
            *self.z.borrow_mut() = self.pos.pos[2];
        }

        self.pos.tik(time, world, event_manager);

        // Load chunks in a 2-chunk radius around the entity
        let chunk = World::world_cords_to_chunk_cords(self.pos.world_block_cords());
        for dz in -2i16..=2 {
            for dy in -2i16..=2 {
                for dx in -2i16..=2 {
                    event_manager.add_world_event(WorldEvent::LoadChunk([
                        chunk[0] + dx,
                        chunk[1] + dy,
                        chunk[2] + dz,
                    ]));
                }
            }
        }
    }

    pub fn get_window(self) -> WidgetType {
        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display("Cursor Entity".to_string())
            .set_text_scale(TextSize::Medium);

        // X / Y / Z controls
        let axes = [
            ("X", self.x.clone(), 1.0_f32),
            ("Y", self.y.clone(), 1.0_f32),
            ("Z", self.z.clone(), 1.0_f32),
        ];
        for (label, rc, step) in axes {
            let row = panel.add_sub_panel();
            row.add_text_display(label.to_string());

            let plus = row.add_button();
            plus.set_text("+".to_string());
            plus.add_left_click_event(
                WidgetEvent::Modf32Event(rc.clone(), step).wrap_into_event()
            );

            let minus = row.add_button();
            minus.set_text("-".to_string());
            minus.add_left_click_event(
                WidgetEvent::Modf32Event(rc.clone(), -step).wrap_into_event()
            );
        }

        // Zoom control
        let zoom_row = panel.add_sub_panel();
        zoom_row.add_text_display("Zoom".to_string());

        let zoom_plus = zoom_row.add_button();
        zoom_plus.set_text("+".to_string());
        zoom_plus.add_left_click_event(
            WidgetEvent::Modf32Event(self.zoom.clone(), 8.0).wrap_into_event()
        );

        let zoom_minus = zoom_row.add_button();
        zoom_minus.set_text("-".to_string());
        zoom_minus.add_left_click_event(
            WidgetEvent::Modf32Event(self.zoom.clone(), -8.0).wrap_into_event()
        );

        panel.wrap_into_widget()
    }
}
