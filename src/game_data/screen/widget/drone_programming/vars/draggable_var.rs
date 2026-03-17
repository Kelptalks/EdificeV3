use crate::game_data::{drone_programming::var::var_type::VarType, screen::widget::{widget::Widget, widget_calculations::{self, buffer_pos}}, types::UITextures};

pub struct DraggableVar {
    // Parent rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Self Rendering
    external_buffers: [f32; 4],  
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    scale: [f32; 2],
    
    var: VarType,
    held: bool,
}

impl DraggableVar {
    pub fn new(var: VarType) -> DraggableVar {
        DraggableVar {
            // Parent Rendering
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],
            prefered_scale: [widget_calculations::get_button_scale(); 2],

            // Self Rendering
            external_buffers: [0.0; 4], 
            internal_buffers: [0.012; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],

            var: var,
            held: false,
        }
    }
}
 

impl Widget for DraggableVar {
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

        if screen_data.mouse_on_ndc_pos(self.pos) && screen_data.was_left_pressed() {
            if !self.held {
                self.held = true;
                game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().set_var_held(self.var);
            }
        }
        else if self.held && screen_data.was_left_released() {
            self.held = false;
            game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().release_var_held();     
        }

        let rendering_pos;
        if self.held {
            rendering_pos = screen_data.get_mouse_centered_texture_rendering_pos(self.scale);
        }
        else {
            rendering_pos = self.pos;
        }
        texture_manager.render_texture_with_pos(self.var.get_texture(), rendering_pos);
    }
}

