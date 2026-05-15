use crate::game_data::{World, game_event_manager::event_manager::{Event, EventManager}, player_data::game_entity::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId}, components::{block_component::{BlockComponent, BlockComponentEvent}, entity_components::{EntityComponent, EntityComponentEvent}, powered_component::{PoweredComponent, PoweredComponentEvent}, vision_component::VisionComponent}, game_entity_manager::GameEntity}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType, widget_calculations::TextSize}, tik_manager::game_time::GameTime, tools::id_gen::IdGen, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();

#[derive(Clone)]
pub struct BlockEntityRadar {
    pub id: u64,

    pub vision: VisionComponent,
    pub block: BlockComponent,
    pub powered: PoweredComponent,
}

impl BlockEntityRadar {
    pub fn wrap_into_game_entity(self) -> GameEntity {
        BlockEntity::Radar(self).wrap_into_game_entity()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityRadar {
        let id = ID_GEN.new_id();
        let game_entity_id = BlockEntityId::Radar(id).wrap_into_game_entity_id();

        let mut block = BlockComponent::new(BlockTexture::LBM, cords, game_entity_id);
        block.init(game_entity_id, event_manager);

        let vision = VisionComponent::new(cords);

        let mut powered = PoweredComponent::new(game_entity_id, cords);
        powered.power_stored = 10000;

        BlockEntityRadar {
            id,
            vision,
            block,
            powered,
        }
    }

    //=====================================
    // Ticking
    //=====================================

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        if self.powered.tik(time, world, event_manager) {
            self.vision.tik(event_manager);
        }
    }

    //=====================================
    // Widget
    //=====================================

    pub fn get_window(self) -> WidgetType {
        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display("Radar".to_string()).set_text_scale(TextSize::Medium);

        panel.add_text_display("Vision Range".to_string());
        let vision_range_panel = panel.add_sub_panel();
        for i in 0..3 {
            let sub_panel = vision_range_panel.add_sub_panel();
            sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

            match i {
                0 => { sub_panel.add_text_display(format!("X = {}", self.vision.vision_radius[0])); }
                1 => { sub_panel.add_text_display(format!("Y = {}", self.vision.vision_radius[1])); }
                2 => { sub_panel.add_text_display(format!("Z = {}", self.vision.vision_radius[2])); }
                _ => {}
            }

            let axis_mod_button = sub_panel.add_button();
            axis_mod_button.set_icon(crate::game_data::types::UITextures::ModIcon);

            let mut range = [0i16; 3];
            range[i] += 1;
            axis_mod_button.add_left_click_event(RadarEvent::ModRange(range).wrap_into_event(self.id));

            let mut range = [0i16; 3];
            range[i] -= 1;
            axis_mod_button.add_right_click_event(RadarEvent::ModRange(range).wrap_into_event(self.id));
        }

        panel.add_widget(self.powered.get_widget());
        return panel.wrap_into_widget();
    }

    //=====================================
    // Getters
    //=====================================

    pub fn get_components(self) -> Vec<EntityComponent> {
        let mut components = Vec::new();
        components.push(self.block.wrap_into_component());
        components.push(self.powered.wrap_into_component());
        components.push(self.vision.wrap_into_component());
        components
    }
}



#[derive(Clone)]
pub enum RadarEvent {
    ModRange([i16; 3]),
    ComponentEvent(EntityComponentEvent),
}

impl RadarEvent {
    pub fn wrap_into_event(self, id: u64) -> Event {
        BlockEntityEvent::RadarEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, radar: &mut BlockEntityRadar) {
        match self {
            RadarEvent::ModRange(range_mod) => {
                radar.vision.mod_range(range_mod);
                let power_needed = radar.vision.chunks_in_view() as u32;
                radar.powered.power_consume_per_interval = power_needed;
            },
            RadarEvent::ComponentEvent(component_event) => {
                match component_event {
                    EntityComponentEvent::Powered(powered_component_event) => {
                        powered_component_event.execute(&mut radar.powered);
                    },
                    EntityComponentEvent::Block(_) => {},
                    EntityComponentEvent::Locomotion(_) => {},
                    EntityComponentEvent::Pos(_) => {},
                }

            },
        }
    }
}
