use crate::game_data::{player_data::game_entity::components::{block_component::{BlockComponent, BlockComponentEvent}, locomotion_component::{LocomotionComponent, LocomotionComponentEvent}, pos_component::{PosComponent, PosComponentEvent}, powered_component::{PoweredComponent, PoweredComponentEvent}, vision_component::VisionComponent}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType, widget_calculations::TextSize}};

pub enum EntityComponent {
    Vision(VisionComponent),
    Block(BlockComponent),
    Powered(PoweredComponent),
    Locomotion(LocomotionComponent),
    Pos(PosComponent),
}

impl EntityComponent {
    pub fn into_widget(&self) -> WidgetType {
        let mut panel = Panel::new_blank();
        panel.set_orientation(
            PanelOrientation::Vertical,
            PanelAlignment::Center
        );

        match self {
            EntityComponent::Vision(vision) => {
                panel.add_text_display("Vision".to_string()).set_text_scale(TextSize::Small);
                panel.add_text_display(format!("Chunk's Loaded: ({})", vision.chunks_in_view())).set_text_scale(TextSize::ExtraSmall);
            },
            EntityComponent::Block(block) => {
                panel.add_text_display("Block".to_string()).set_text_scale(TextSize::Small);
                panel.add_text_display(format!("Cords: ({:?})", block.world_cords)).set_text_scale(TextSize::ExtraSmall);
                panel.add_text_display(format!("Block_type: ({})", block.block_type.get_name())).set_text_scale(TextSize::ExtraSmall);
            },
            EntityComponent::Powered(powered) => {
                return powered.get_widget()
            },
            EntityComponent::Locomotion(locomotion) => {
                panel.add_text_display("Locomotion".to_string()).set_text_scale(TextSize::Small);
                panel.add_text_display(format!("Velocity: ({:.2}, {:.2}, {:.2})", locomotion.velocity[0], locomotion.velocity[1], locomotion.velocity[2])).set_text_scale(TextSize::ExtraSmall);
                panel.add_text_display(format!("Gravity: {:.2}  Friction: {:.2}", locomotion.gravity, locomotion.friction)).set_text_scale(TextSize::ExtraSmall);
            },
            EntityComponent::Pos(pos) => {
                panel.add_text_display("Position".to_string()).set_text_scale(TextSize::Small);
                panel.add_text_display(format!("Pos: ({:.1}, {:.1}, {:.1})", pos.pos[0], pos.pos[1], pos.pos[2])).set_text_scale(TextSize::ExtraSmall);
                panel.add_text_display(format!("Texture: ({})", pos.block_type.get_name())).set_text_scale(TextSize::ExtraSmall);
            },
        }

        panel.wrap_into_widget()
    }
}



#[derive(Clone)]
pub enum EntityComponentEvent {
    Powered(PoweredComponentEvent),
    Block(BlockComponentEvent),
    Locomotion(LocomotionComponentEvent),
    Pos(PosComponentEvent),
}


impl EntityComponentEvent {

    pub fn execute(&self) {

    }


}
