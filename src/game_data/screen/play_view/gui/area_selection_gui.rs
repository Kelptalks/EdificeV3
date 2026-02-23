use crate::game_data::{TextureManager, screen::{Button, ScreenData}};



struct WorldPointerButtonSetter {
    button: Button,
    world_cords: [i32; 3],
    is_set: bool,
}

impl WorldPointerButtonSetter {
    fn new() -> WorldPointerButtonSetter {
        WorldPointerButtonSetter {
            button: Button::new_blank(crate::game_data::types::UITextures::ButtonCircle),
            world_cords: [0, 0, 0],
            is_set: false,
        }
    }
}

pub struct AreaSelectionGUI {
    world_points: [WorldPointerButtonSetter; 3],
    ndc: [f32; 2],
    scale: [f32; 2],

}

impl AreaSelectionGUI {
    pub fn new() -> AreaSelectionGUI{
        let world_points = [
            WorldPointerButtonSetter::new(),
            WorldPointerButtonSetter::new(),
            WorldPointerButtonSetter::new(),
        ];
        
        AreaSelectionGUI {
            world_points,
            ndc: [0.0, 0.0],
            scale: [0.0, 0.0],
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    fn resize_world_points(&mut self) {
        let button_scale = (3.0 * self.scale[0] / 11.0).min(self.scale[1]);
        let stride = button_scale + button_scale / 3.0;

        for (index, world_points) in self.world_points.iter_mut().enumerate() {
            let button_ndc = [
                self.ndc[0] + (index as f32 * stride),
                self.ndc[1],
            ];
            world_points.button.set_ndc(button_ndc);
            world_points.button.set_scale(button_scale);
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

    //=====================================
    // Rendering
    //=====================================

    pub fn render_view(
        &mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager, 
    ) {
        for world_points in &mut self.world_points {
            world_points.button.render_button(texture_manager, screen_data);
        }
    }
}