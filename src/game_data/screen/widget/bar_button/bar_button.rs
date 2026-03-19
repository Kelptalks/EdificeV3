use crate::game_data::{
    TextureManager,
    game_event_manager::game_event_manager::{GameEvent, EventManager},
    screen::{
        ScreenData, render_centered_string_at_ndc, text::render_string_at_ndc, widget::{
            bar_button::bar_button_texture_manager::BarButtonTextureManager,
            widget::Widget,
            widget_calculations::{self, TextSize},
        }
    },
    types::FontType,
};

pub struct BarButtonWidget {
    // Parent Rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],

    // Rendering
    needs_resizing: bool,
    external_buffers: [f32; 4],
    

    pos: [f32; 4],
    scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Input
    events: Vec<GameEvent>,

    // Appearance
    texture_manager: BarButtonTextureManager,
    text: String,
    text_ndc: [f32; 2],
    prefered_char_scale: f32,
    bar_button_text_buffers: f32,
}

impl BarButtonWidget {
    pub fn new(text: String, buffers: [f32; 4]) -> BarButtonWidget {
        BarButtonWidget {
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],

            needs_resizing: true,
            external_buffers: buffers,

            pos: [0.0; 4],
            scale: [0.0; 2],

            prefered_scale: [0.0; 2],

            events: Vec::new(),

            texture_manager: BarButtonTextureManager::new(),
            text,
            text_ndc: [0.0; 2],
            prefered_char_scale: TextSize::Small.get_scale(),
            bar_button_text_buffers: 0.0,
        }


    }

    //=====================================
    // Events
    //=====================================
    pub fn add_event(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    //=====================================
    // Aperence
    //=====================================

    pub fn set_text_scale(&mut self, size: TextSize) {
        self.prefered_char_scale = size.get_scale();
        self.needs_resizing = true;
    }


    //=====================================
    // Sizing
    //=====================================

    pub fn size(&mut self) {
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);

        // Compute preferred scale from char scale and text length

        self.bar_button_text_buffers = self.prefered_char_scale / 1.0;

        self.prefered_scale = [
            self.prefered_char_scale * self.text.len() as f32 + self.bar_button_text_buffers,
            self.prefered_char_scale + self.bar_button_text_buffers
        ];

        self.text_ndc = [
            self.pos[0] + self.bar_button_text_buffers / 2.0,
            self.pos[1] + self.bar_button_text_buffers / 2.0,
        ];

        // Size the texture renderer
        self.texture_manager.size(self.pos);

        // Compute text center
        self.needs_resizing = false;
    }
}

impl Widget for BarButtonWidget {
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
        self.needs_resizing = true;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
        self.needs_resizing = true;
    }

    fn size(&mut self) {
        self.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
    ) {
        if self.needs_resizing {
            self.size();
        }

        let is_hovered = screen_data.mouse_on_ndc_pos(self.pos);

        if is_hovered && screen_data.was_left_released() {
            for event in &self.events {
                game_event_manager.add_event(event.clone());
            }
        }

        self.texture_manager.render(texture_manager, is_hovered);

        render_string_at_ndc(
            texture_manager,
            self.text.clone(),
            FontType::Basic,
            self.prefered_char_scale,
            self.text_ndc,
        );
    }
}
