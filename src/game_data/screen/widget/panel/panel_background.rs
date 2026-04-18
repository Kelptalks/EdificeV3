use std::time::Instant;

use crate::game_data::{TextureManager, screen::widget::widget_calculations, texture_manager::rect::Pos, types::UITextures};

pub enum BackgroundType {
    Static(UITextures),
    Scrolling(UITextures),
}

pub struct PanelBackground {
    background: BackgroundType,

    shift_speed: u32,
    current_shift: f32,
    tile_scale: f32,
    last_shift_time: Instant,
}

impl PanelBackground {
    pub fn new(background_type: BackgroundType) -> PanelBackground {
        PanelBackground {
            background: background_type,

            shift_speed: 100,
            current_shift: 0.0,
            tile_scale: 0.2,
            last_shift_time: Instant::now(),
        }
    }

    fn render_scrolling(&mut self, texture_manager: &mut TextureManager, pos: [f32; 4], ui_texture: UITextures, bounds: Option<[f32; 4]>) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_shift_time).as_secs_f32();
        
        // shift_speed acts as shifts-per-second; interval = 1.0 / shift_speed
        let interval = 1.0 / self.shift_speed.max(1) as f32;

        if elapsed >= interval {
            self.current_shift += 0.001;
            self.last_shift_time = now;
        }

        if self.current_shift >= self.tile_scale {
            self.current_shift = self.current_shift % self.tile_scale;
        }


        let scale = widget_calculations::pos_to_scale(pos);
        let tiles_needed = [
            (scale[0] / self.tile_scale) as i32 + 2,
            (scale[1] / self.tile_scale) as i32 + 2,
        ]; 

        for x in 0..tiles_needed[0] {
            for y in 0..tiles_needed[1] {

                let x_cor = (self.tile_scale * x as f32) - self.current_shift;
                let y_cor = (self.tile_scale * y as f32) - self.current_shift;

                let pos = [
                    pos[0] + x_cor,
                    pos[1] + y_cor,
                    pos[0] + x_cor + self.tile_scale * 1.01,
                    pos[1] + y_cor + self.tile_scale * 1.01,
                ];


                if let Some(bounds ) = bounds {
                    texture_manager.render_texture_within_pos(ui_texture.wrap_into_texture(), pos, bounds);
                }
                else {
                    texture_manager.render_ui_element_with_pos(ui_texture, pos);
                }
            }
        }
    }

    pub fn render_background(&mut self, texture_manager: &mut TextureManager, pos: [f32; 4], bounds: Option<[f32; 4]>) {
        match self.background {
            BackgroundType::Static(ui_texture) => {
                if let Some(bounds ) = bounds {
                    texture_manager.render_texture_within_pos(ui_texture.wrap_into_texture(), pos, bounds);
                }
                else {
                    texture_manager.render_ui_element_with_pos(ui_texture, pos);
                }
            },
            BackgroundType::Scrolling(ui_texture) => {
                self.render_scrolling(texture_manager, pos, ui_texture, bounds);
            },
        }
    }
}