use std::{cell::RefCell, rc::Rc};

use crate::game_data::{TextureManager, game_event_manager::prelude::EventManager, player_data::drone_script::var::{self, var::Var, var_type::{VarKind, VarType}}, screen::{ScreenData, text::render_string_at_ndc, widget::{widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties}}, texture_manager::{self, texture::Texture}, types::UITextures};



pub struct VarSlot {
    widget_properties: WidgetProperties,

    var: Var,
    var_string_ndc: [f32; 2],

    allow_setting: bool,
    allow_dragging: bool,
    allow_clearing: bool,
}

impl VarSlot {
    pub fn new_with_var(var_slot_type: Var) -> VarSlot {
        let mut wp = WidgetProperties::new_blank();
        wp.prefered_scale = [widget_calculations::get_button_scale(); 2];
        wp.internal_buffers = [0.012; 4];

        VarSlot {
            widget_properties: wp,
            var: var_slot_type,
            var_string_ndc: [0.0; 2],
            allow_setting: true,
            allow_dragging: true,
            allow_clearing: true,
        }
    }

    pub fn new_with_type(var_type: VarType) -> VarSlot {
        Self::new_with_var(Var::new_with_var_type(var_type))
    }

    pub fn new_with_kind(var_kind: VarKind) -> VarSlot {
        Self::new_with_var(Var::new_blank_with_kind(var_kind))
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::VarSlot(self)
    }

    //=====================================
    // Input Properties
    //=====================================

    pub fn set_dragging_properties(&mut self, allow_dragging: bool, allow_setting: bool, allow_clearing: bool) {
        self.allow_dragging = allow_dragging;
        self.allow_setting = allow_setting;
        self.allow_clearing = allow_clearing;
    }

    //=====================================
    // Control Management Functions
    //=====================================

    fn try_and_set_var(&mut self, var_held_by_mouse: &Option<Var>) {
        if self.allow_setting {
            if let Some(var_held_by_mouse) = var_held_by_mouse {
                self.var.set_with_var(var_held_by_mouse);
            }
        }
    }

    fn try_and_get_var(&mut self) -> Option<Var> {
        if self.allow_dragging {
            return Some(self.var.clone());
        } else {
            return None;
        }
    }

    fn render_string(&self, texture_manager: &mut TextureManager) {
        let string = self.var.get_name();
        let text_scale = widget_calculations::TextSize::ExtraExtraSmall.get_scale();
        let string_centering_offset = (string.len() as f32 * text_scale) / 2.0;

        let pos   = self.widget_properties.pos;
        let scale = self.widget_properties.scale;

        let string_ndc = [
            (pos[0] + scale[0] / 2.0) - string_centering_offset,
            pos[1] - (scale[1] / 4.0),
        ];

        render_string_at_ndc(
            texture_manager,
            string,
            crate::game_data::types::FontType::Basic,
            text_scale,
            string_ndc,
        );
    }

}

impl Widget for VarSlot {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.widget_properties.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
    }

    fn size(&mut self) {
        self.widget_properties.scale_based_off_parent();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
    ) {
        let bounds = self.widget_properties.bounds;
        let pos = self.widget_properties.pos;

        if self.var.is_null() {
            texture_manager.render_texture_within_pos_option(self.var.get_kind_texture(), pos, bounds);
        }
        texture_manager.render_texture_within_pos_option(self.var.get_texture(), pos, bounds);

        if screen_data.mouse_on_ndc_pos(pos) {
            self.render_string(texture_manager);

            if screen_data.was_left_released() {
                let var_held_by_mouse = game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().get_var_held();
                self.try_and_set_var(var_held_by_mouse);
            }

            if screen_data.was_left_pressed() {
                game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().set_var_held(self.try_and_get_var());
            }

            if self.allow_setting && screen_data.was_right_pressed() {
                self.var.clear();
            }
        }
    }
}
