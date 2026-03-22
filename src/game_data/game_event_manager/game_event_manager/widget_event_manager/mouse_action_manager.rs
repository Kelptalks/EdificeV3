use crate::game_data::{drone_programming::var::var_type::Var, screen::widget::widget_calculations};

pub struct MouseWidgetData {
    mouse_data_rendering_scale: [f32; 2],
    
    var_held: Option<Var>,
}

impl MouseWidgetData {
    pub fn new() -> MouseWidgetData {
        return MouseWidgetData {
            mouse_data_rendering_scale: [widget_calculations::get_button_scale(); 2],

            var_held: None
        }
    }

    //=====================================
    // Var Setters / Getters
    //=====================================

    pub fn set_var_held(&mut self, var: Var) {
        self.var_held = Some(var);
    }

    pub fn get_var_held(&mut self) -> &Option<Var> {
        return &self.var_held;
    }

    pub fn release_var_held(&mut self) {
        self.var_held = None;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render(
        &mut self, 
        texture_manager: &mut crate::game_data::TextureManager, 
        screen_data: &crate::game_data::screen::ScreenData, 
    ) {
        if let Some(var) = self.var_held {
            texture_manager.render_texture_with_pos(
                var.get_texture(), 
                screen_data.get_mouse_centered_texture_rendering_pos(self.mouse_data_rendering_scale)
            );
        }
    }

}