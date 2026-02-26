use miniquad::MouseButton;

use crate::game_data::{TextureManager, game_event_manager::game_event_manager::GameEventManager, locations::world_area::WorldArea, screen::{Button, ScreenData, play_view::play_view_data::{self, PlayViewData}}, types::UITextures};



pub struct AreaSelectionGUI {
    area: WorldArea,
    buttons: [Button; 2],
    ndc: [f32; 2],
    scale: [f32; 2],

}

impl AreaSelectionGUI {
    pub fn new() -> AreaSelectionGUI{


        AreaSelectionGUI {
            area: WorldArea::new_blank(),
            buttons: [Button::new_blank(UITextures::ButtonCircle), Button::new_blank(UITextures::ButtonCircle)],
            ndc: [0.0, 0.0],
            scale: [0.0, 0.0],
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    fn resize_world_points(&mut self) {
        let button_scale = self.scale[0] / 2.0;
        let stride = button_scale;

        for (index, button) in self.buttons.iter_mut().enumerate() {
            let button_ndc = [
                self.ndc[0] + (index as f32 * stride),
                self.ndc[1],
            ];
            button.set_ndc(button_ndc);
            button.set_scale(button_scale);
        }

    }

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
        self.resize_world_points();
    }

    pub fn set_x_scale(&mut self, scale: f32) {
        self.scale[0] = scale;
        self.scale[1] = scale / 4.0;
        self.resize_world_points();
    }

    pub fn get_world_area(&self) -> &WorldArea {
        return &self.area;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_view(
        &mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager,
    ) {
        for button in &mut self.buttons {
            button.render_button(texture_manager, screen_data);
        }
    }


    pub fn mouse_button_down_event(&mut self, 
        event_manager: &mut GameEventManager, 
        play_view_data: &mut PlayViewData, 
        screen_data: &ScreenData, 
        button: MouseButton
    ) {
        if MouseButton::Left == button {

            if self.buttons[0].is_mouse_on_button() {
                self.area.set_point_1(play_view_data.get_world_cords());
                let text = format!("{:?}", play_view_data.get_world_cords());
                self.buttons[0].set_text(text);
            }
            else if self.buttons[1].is_mouse_on_button() {
                self.area.set_point_2(play_view_data.get_world_cords());
                let text = format!("{:?}", play_view_data.get_world_cords());
                self.buttons[1].set_text(text);
            }
        }
    }
}
