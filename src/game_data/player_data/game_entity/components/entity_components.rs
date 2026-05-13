use crate::game_data::{player_data::game_entity::components::{block_component::BlockComponent, powered_component::PoweredComponent, vision_component::VisionComponent}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType, widget_calculations::TextSize}};

pub enum EntityComponent {
    Vision(VisionComponent),
    Block(BlockComponent),
    Powered(PoweredComponent),
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
        }

        panel.wrap_into_widget()
    }
}
