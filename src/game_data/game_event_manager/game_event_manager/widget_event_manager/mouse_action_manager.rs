
use crate::game_data::{player_data::drone_script::var::var::Var, screen::widget::widget_calculations};



pub struct MouseWidgetData {
    mouse_data_rendering_scale: [f32; 2],
    
    var_held: Option<Var>,
}

impl MouseWidgetData {
    pub fn new() -> MouseWidgetData {
        return MouseWidgetData {
            mouse_data_rendering_scale: [widget_calculations::get_button_scale(); 2],

            var_held: None,
        }
    }

    //=====================================
    // Var Setters / Getters
    //=====================================

    pub fn set_var_held(&mut self, var: Option<Var>) {
        self.var_held = var;
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
        // Drop If Mouse Is not Held
        if !screen_data.is_left_mouse_held() {
            self.release_var_held();
        } 


        if let Some(var) = self.var_held.clone() {
            texture_manager.render_texture(
                var.get_kind_texture(), 
                screen_data.get_mouse_centered_texture_rendering_pos(self.mouse_data_rendering_scale)
            );

            texture_manager.render_texture(
                var.get_texture(), 
                screen_data.get_mouse_centered_texture_rendering_pos(self.mouse_data_rendering_scale)
            );
        }



    }

}