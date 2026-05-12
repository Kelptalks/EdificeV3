use crate::game_data::{game_event_manager::event_manager::Event, player_data::game_object::{game_object_manager::{GameObject, GameObjectEvent, GameObjectId}, traits::{trait_block::BlockTrait, trait_powered::{PoweredTrait, PoweredTraitEvent}, trait_vision::VisionTrait}}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType, widget_calculations::TextSize}};

pub enum GameObjectTrait {
    Vision(VisionTrait),
    BlockTrait(BlockTrait),
    Powered(PoweredTrait),

}

impl GameObjectTrait {
    pub fn into_widget(&self) -> WidgetType {
        let mut panel = Panel::new_blank();
        panel.set_orientation(
            PanelOrientation::Vertical, 
            PanelAlignment::Center
        );

        match self {
            GameObjectTrait::Vision(vision_trait) => {
                panel.add_text_display("Vision".to_string()).set_text_scale(TextSize::Small);
                panel.add_text_display(format!("Chunk's Loaded: ({})", vision_trait.chunks_in_view())).set_text_scale(TextSize::ExtraSmall);
            },
            GameObjectTrait::BlockTrait(block_trait) => {
                panel.add_text_display("Block".to_string()).set_text_scale(TextSize::Small);
                panel.add_text_display(format!("Cords: ({:?})", block_trait.world_cords)).set_text_scale(TextSize::ExtraSmall);
                panel.add_text_display(format!("Block_type: ({})", block_trait.block_type.get_name())).set_text_scale(TextSize::ExtraSmall);

            },
            GameObjectTrait::Powered(powered_trait) => {
                panel.add_text_display("Powered".to_string()).set_text_scale(TextSize::Small);
                panel.add_text_display(format!("Power Stored: ({})", powered_trait.power_stored)).set_text_scale(TextSize::ExtraSmall);
            },
        }

        panel.wrap_into_widget()
    }
}


#[derive(Clone)]
pub enum TraitEvent {
    PoweredEvent(PoweredTraitEvent),
}

impl TraitEvent {

    pub fn wrap_into_event(self, id: GameObjectId) -> Event {
        GameObjectEvent::TraitEvent(id, self).wrap_into_event()
    }

    pub fn execute(self, object: &mut GameObject) {
        
    }
}