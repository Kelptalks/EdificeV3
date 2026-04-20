use crate::game_data::{player_data::drone_script::action::{self, action::Action}, screen::widget::{drone_programming::scripting_widget_type::ScriptingElementWidget, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot}, widget::{Widget, WidgetType}, widget_calculations::TextSize, widget_properties::WidgetProperties}, texture_manager::texture::Texture};

pub struct ActionSlot {
    panel: Panel,
    line: usize,
}


impl ActionSlot {
    pub fn new(action: &Action, line: usize) -> ActionSlot {
        let mut panel = Panel::new_blank();

        // Identity panel
        let identity_sub_panel = panel.add_sub_panel();

        // Button for displaying identity
        let button = identity_sub_panel.add_button();
        button.add_texture(action.get_texture());
        button.set_text(action.get_name());


        // Params
        let param_refs = action.get_params_var_refs();
        if param_refs.len() != 0 {
            let param_sub_panel = panel.add_sub_panel();
            param_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
            param_sub_panel.add_text_display("Params".to_string()).set_text_scale(TextSize::ExtraSmall);

            let slot_sub_panel = param_sub_panel.add_sub_panel();
            for param_var_ref in param_refs {
                let var_slot = VarSlot::new_with_var_ref(param_var_ref.clone());
                slot_sub_panel.add_widget(var_slot.wrap_into_widget());
            }
        }

        // Return
        let return_var_kind_option = action.get_return_var();
        if let Some(return_var) = return_var_kind_option {
            let return_sub_panel = panel.add_sub_panel();
            return_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
            return_sub_panel.add_text_display("Return".to_string()).set_text_scale(TextSize::ExtraSmall);

            let slot_sub_panel = return_sub_panel.add_sub_panel();
            let mut var_slot = VarSlot::new_with_var(return_var.clone());
            var_slot.set_dragging_properties(true, false, false);
            slot_sub_panel.add_widget(var_slot.wrap_into_widget());
        }

        ActionSlot {
            panel: panel,
            line
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::ActionSlot(self)
    }
}


impl Widget for ActionSlot {
    fn get_widget_properties(&self) -> &WidgetProperties {
        self.panel.get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        self.panel.get_mut_widget_properties()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.panel.set_buffers(pos)
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.panel.set_parent_pos(pos)
    }

    fn size(&mut self) {
        self.panel.size()
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager);
        
        // self.panel.set_color(PanelColor::Light);
    }
}

impl ScriptingElementWidget for ActionSlot {
    fn get_line_index(&self) -> usize {
        self.line
    }

    fn set_highlighted(&mut self) {
        self.panel.set_color(PanelColor::DarkBlue);
    }
}
