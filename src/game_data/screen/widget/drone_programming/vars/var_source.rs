use crate::game_data::{player_data::drone_programming::var::var_type::Var, screen::{render_centered_string_at_ndc, text::render_string_at_ndc, widget::{widget::Widget, widget_calculations::{self, buffer_pos}}}, types::UITextures};


#[derive(Clone)]
pub struct VarSource {
    // Parent rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Self Rendering
    external_buffers: [f32; 4],  
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    scale: [f32; 2],

    text_centered_ndc: [f32; 2],
    
    var: Var,
    held: bool,
}

impl VarSource {
    pub fn new(var: Var) -> VarSource {
        VarSource {
            // Parent Rendering
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],
            prefered_scale: [widget_calculations::get_button_scale(); 2],

            // Self Rendering
            external_buffers: [0.0; 4], 
            internal_buffers: [0.012; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],

            text_centered_ndc: [0.0; 2],

            var: var,
            held: false,
        }
    }
}
 

impl Widget for VarSource {
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
        self.text_centered_ndc = [self.pos[0] + (self.scale[0] / 2.0), self.pos[1]]
    }

    fn render(
        &mut self, 
        texture_manager: &mut crate::game_data::TextureManager, 
        screen_data: &crate::game_data::screen::ScreenData, 
        game_event_manager: &mut crate::game_data::game_event_manager::game_event_manager::EventManager
    ) {

        let rendering_pos;
        if self.held {
            rendering_pos = screen_data.get_mouse_centered_texture_rendering_pos(self.scale);
        }
        else {
            rendering_pos = self.pos;
        }
        texture_manager.render_texture_with_pos(self.var.get_texture(), rendering_pos);

        if screen_data.mouse_on_ndc_pos(self.pos) {
            let string = self.var.get_name();

            render_centered_string_at_ndc(texture_manager, string, crate::game_data::types::FontType::Basic, widget_calculations::get_button_text_scale(), self.text_centered_ndc);

            if screen_data.was_left_pressed() {
                if !self.held {
                    self.held = true;
                    game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().set_var_held(self.var.clone());
                }
            }
        }
        else if self.held && screen_data.was_left_released() {
            self.held = false;
            game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().release_var_held();     
        }

        
    }
}

