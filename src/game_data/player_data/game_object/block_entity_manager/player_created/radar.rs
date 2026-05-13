use std::fmt::format;

use crate::game_data::{World, game_event_manager::event_manager::{Event, EventManager}, player_data::game_object::{block_entity_manager::block_entity_manager::{BlockEntity, BlockEntityEvent, BlockEntityId}, game_object_manager::{GameObject, GameObjectEvent}, id_gen::IdGen, traits::{game_object_trait_manager::GameObjectTrait, trait_block::{self, BlockTrait}, trait_powered::{self, PoweredTrait, PoweredTraitEvent}, trait_vision::VisionTrait}}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType, widget_calculations::TextSize}, tik_manager::game_time::GameTime, types::BlockTexture};

static ID_GEN: IdGen = IdGen::new();

#[derive(Clone)]
pub struct BlockEntityRadar {
    pub id: u64,
    
    pub trait_vision: VisionTrait,
    pub trait_block: BlockTrait,
    
    pub trait_powered: PoweredTrait,
}

impl BlockEntityRadar {
    pub fn wrap_into_game_object(self) -> GameObject {
        BlockEntity::Radar(self).wrap_into_game_object()
    }

    pub fn new(cords: [i32; 3], event_manager: &mut EventManager) -> BlockEntityRadar {
        let id = ID_GEN.new_id();
        let game_objcet_id: crate::game_data::player_data::game_object::game_object_manager::GameObjectId = BlockEntityId::RadarID(id).wrap_into_game_object_id();
        
        // Block
        let mut block_trait = 
            BlockTrait::new(BlockTexture::LBM, cords);
        block_trait.init(game_objcet_id, event_manager);

        // Vission
        let trait_vision = VisionTrait::new(cords);
        
        // Powered
        let mut trait_powered = PoweredTrait::new(game_objcet_id, cords);
        trait_powered.power_stored = 10000;

        BlockEntityRadar {
            id,
            trait_vision: trait_vision,
            trait_block: block_trait,
            trait_powered: trait_powered
        }
    }


    //=====================================
    // Tiking
    //=====================================

    pub fn tik(&mut self, time: &GameTime, world: &World, event_manager: &mut EventManager) {
        // Tik vision
        if self.trait_powered.tik(time, world, event_manager) {
            self.trait_vision.tik(event_manager);
        }
    }

    //=====================================
    // Widget
    //=====================================

    pub fn get_window(self) -> WidgetType {
        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display("Radar".to_string()).set_text_scale(TextSize::Medium);
        
        // Vision
        panel.add_text_display("Vision Range".to_string());
        let vision_range_panel = panel.add_sub_panel();
        for i in 0..3 {
            let sub_panel = vision_range_panel.add_sub_panel();
            sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
            
            // Axis Header
            match i {
                0 => {
                    sub_panel.add_text_display(format!("X = {}", self.trait_vision.vision_radius[0]));
                }
                1 => {
                    sub_panel.add_text_display(format!("Y = {}", self.trait_vision.vision_radius[1]));
                }
                2 => {
                    sub_panel.add_text_display(format!("Z = {}", self.trait_vision.vision_radius[2]));
                }
                _ => {

                }
            }
            
            // Axis Minipulation Button
            let axis_mod_button = sub_panel.add_button();
            axis_mod_button.set_icon(crate::game_data::types::UITextures::ModIcon);

            let mut range = [0; 3];
            range[i] += 1;
            axis_mod_button.add_left_click_event(RadarEvent::ModRange(range).wrap_into_event(self.id));

            let mut range = [0; 3];
            range[i] -= 1;
            axis_mod_button.add_right_click_event(RadarEvent::ModRange(range).wrap_into_event(self.id));
        }
        

        // Power
        panel.add_text_display("Vision Range".to_string());
        let power_panel = panel.add_sub_panel();
        power_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        power_panel.add_text_display(
            format!(
                "Stored Power: {} / {}", 
                self.trait_powered.power_stored, 
                self.trait_powered.max_power
            )
        );

        power_panel.add_text_display(
            format!(
                "Power Consumption {} / {}", 
                self.trait_powered.power_consume_per_interval, 
                self.trait_powered.power_consume_interval.to_string()
            )
        );

        return panel.wrap_into_widget();
    }

    //=====================================
    // Getters
    //=====================================

    pub fn get_traits(self) -> Vec<GameObjectTrait> {
        let mut traits = Vec::new();

        traits.push(self.trait_block.wrap_into_trait());
        traits.push(self.trait_powered.wrap_into_trait());
        traits.push(self.trait_vision.wrap_into_trait());

        traits
    }
}



#[derive(Clone)]
pub enum RadarEvent {
    ModRange([i16; 3]),
    PoweredTraitEvent(PoweredTraitEvent),
}

impl RadarEvent {
    pub fn wrap_into_event(self, id: u64) -> Event {
        BlockEntityEvent::RadarEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, radar: &mut BlockEntityRadar) {
        match self {
            RadarEvent::ModRange(range_mod) => {
                radar.trait_vision.mod_range(range_mod);

                let power_needed = radar.trait_vision.chunks_in_view() as u32;
                radar.trait_powered.power_consume_per_interval = power_needed;
            },
            RadarEvent::PoweredTraitEvent(powered_trait_event) => {
                powered_trait_event.execute(&mut radar.trait_powered);
            },
        }
    }
}