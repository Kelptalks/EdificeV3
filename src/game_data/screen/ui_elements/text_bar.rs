use miniquad::{KeyCode, MouseButton};

use crate::game_data::{TextureManager, screen::{ScreenData, text::render_string_at_ndc, ui_elements::panel::{Panel, PanelColor}}, types::{CharType, FontType, UITextures}};

pub struct TextBar {
    // Text
    text: String,
    max_text_length: usize,

    // Controls
    is_mouse_on: bool,
    clicked: bool,

    // Rendering
    ndc: [f32; 2],
    ndc_y_scale: f32,
    panel: Panel,
}

impl TextBar {

    //=====================================
    // Constructors
    //=====================================

    pub fn new_blank() -> TextBar {
        TextBar {
            // Text
            text: "".to_string(),
            max_text_length: 32,

            // Controls
            is_mouse_on: false,
            clicked: false,

            // Rendering
            ndc: [0.0, 0.0],
            ndc_y_scale: 0.0,
            panel: {
                let mut p = Panel::new_blank();
                p.set_color(PanelColor::Dark);
                p
            },
        }
    }

    //=====================================
    // Helpers
    //=====================================

    fn tile_scale(&self) -> f32 {
        self.ndc_y_scale / 4.0
    }

    // Buffer between the tile border and the text
    fn buffer(&self) -> f32 {
        self.tile_scale() * 0.5
    }

    fn text_scale(&self) -> f32 {
        let inner = self.ndc_y_scale - self.tile_scale() * 2.0;
        inner - self.buffer() * 2.0
    }

    fn ndc_x_scale(&self) -> f32 {
        self.text_scale() * self.max_text_length as f32 + self.tile_scale() * 2.0 + self.buffer() * 2.0
    }

    fn recalculate_panel(&mut self) {
        let tile = self.tile_scale();
        let x = self.ndc_x_scale();
        let y = self.ndc_y_scale;
        self.panel.set_ndc(self.ndc);
        self.panel.set_ndc_scale([x, y]);
        self.panel.set_tile_ndc_scale(tile);
    }

    //=====================================
    // Getters and setters
    //=====================================

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
        self.recalculate_panel();
    }

    pub fn set_ndc_y_scale(&mut self, y_scale: f32) {
        self.ndc_y_scale = y_scale;
        self.recalculate_panel();
    }

    pub fn set_max_text_length(&mut self, max: usize) {
        self.max_text_length = max;
        self.recalculate_panel();
    }

    pub fn set_color(&mut self, color: PanelColor) {
        self.panel.set_color(color);
    }

    pub fn set_text(&mut self, text: String) {
        if text.len() <= self.max_text_length {
            self.text = text;
        } else {
            self.text = text[..self.max_text_length].to_string();
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_ndc_scale(&self) -> [f32; 2] {
        [self.ndc_x_scale(), self.ndc_y_scale]
    }

    pub fn is_mouse_on(&self) -> bool {
        self.is_mouse_on
    }

    //=====================================
    // Input
    //=====================================

    pub fn handle_mouse_button_down(&mut self, button: MouseButton, screen_data: &ScreenData) {
        if button != MouseButton::Left {
            return;
        }

        let bounds = [
            self.ndc[0],
            self.ndc[1],
            self.ndc[0] + self.ndc_x_scale(),
            self.ndc[1] + self.ndc_y_scale,
        ];

        if screen_data.mouse_on_ndc_pos(bounds) {
            self.clicked = true;
            self.panel.set_color(PanelColor::Light);
        } else {
            self.clicked = false;
            self.panel.set_color(PanelColor::Dark);
        }
    }

    pub fn handle_keydown(&mut self, keycode: KeyCode) {
        if !self.clicked {
            return;
        }

        match keycode {
            KeyCode::Escape => {
                self.clicked = false;
                self.panel.set_color(PanelColor::Dark);
            }
            KeyCode::Enter => {
                self.clicked = false;
                self.panel.set_color(PanelColor::Dark);
            }
            KeyCode::Backspace => {
                self.text.pop();
            }
            _ => {
                let c = CharType::keycode_to_char(keycode);
                if self.text.len() < self.max_text_length {
                    self.text.push(c);
                }
            }
        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        let x_scale = self.ndc_x_scale();

        // Update mouse hover state
        let mouse = screen_data.get_mouse_ndc();
        self.is_mouse_on =
            mouse[0] >= self.ndc[0] &&
            mouse[0] <= self.ndc[0] + x_scale &&
            mouse[1] >= self.ndc[1] &&
            mouse[1] <= self.ndc[1] + self.ndc_y_scale;

        // Render panel background
        self.panel.render(texture_manager);

        // Render text left-aligned with buffer, vertically centered in the inner area
        let tile = self.tile_scale();
        let buffer = self.buffer();
        let text_scale = self.text_scale();
        let inner_height = self.ndc_y_scale - tile * 2.0;
        let text_y_offset = (inner_height - text_scale) / 2.0;
        let text_ndc = [
            self.ndc[0] + tile + buffer,
            self.ndc[1] + tile + text_y_offset,
        ];
        render_string_at_ndc(
            texture_manager,
            self.text.clone(),
            FontType::Basic,
            text_scale,
            text_ndc,
        );

        // Render cursor at the next character position
        if self.clicked {
            let cursor_x = text_ndc[0] + text_scale * self.text.len() as f32;
            let cursor_pos = [cursor_x, text_ndc[1], cursor_x + text_scale, text_ndc[1] + text_scale];
            texture_manager.render_ui_element_with_pos(UITextures::Play, cursor_pos);
        }
    }

}
