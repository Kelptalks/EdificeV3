use crate::game_data::{drone_programming::var::{self, game_vars::game_var_type::GameVarType, var_type::{VarType, VarTypeKind}}, screen::widget::{widget::Widget, widget_calculations}, types::{BlockTexture, UITextures}};

pub struct VarSlot {
    // Parent rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Self Rendering
    external_buffers: [f32; 4],  
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    scale: [f32; 2],
    
    var_type_kind_allowed: VarTypeKind,
    var: Option<VarType>,

}

impl VarSlot {
    pub fn new(var_type_kind_allowed: VarTypeKind) -> VarSlot {
        VarSlot {
            // Parent Rendering
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],
            prefered_scale: [widget_calculations::get_button_scale(); 2],

            // Self Rendering
            external_buffers: [0.0; 4], 
            internal_buffers: [0.012; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],

            var_type_kind_allowed: var_type_kind_allowed,
            var: None,
        }
    }
}

impl Widget for VarSlot {
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }

    fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
    }

    fn size(&mut self) {
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::game_event_manager::GameEventManager
    ) {

        // Try and get var if mouse was released
        if screen_data.mouse_on_ndc_pos(self.pos) {
            if screen_data.was_left_pressed() {
                if let Some(var_held_by_mouse) = game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().get_var_held() {
                    if var_held_by_mouse.to_kind() == self.var_type_kind_allowed {
                        self.var = Some(*var_held_by_mouse);
                    }
                }
            }
            else if screen_data.was_right_pressed() {
                self.var = None;
            }
        }

        game_event_manager.get_event_tools().get_mouse_widget_data();
        if let Some(var) = self.var {
            texture_manager.render_texture_with_pos(var.get_texture(), self.pos);
        }
        else {
            texture_manager.render_texture_with_pos(self.var_type_kind_allowed.get_texture(), self.pos);
        }
        
    }
}