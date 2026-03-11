use crate::game_data::{
    TextureManager,
    game_event_manager::game_event_manager::{Event, GameEventManager},
    screen::{
        ScreenData,
        render_centered_string_at_ndc,
        widget::{
            bar_button::bar_button_texture_manager::BarButtonTextureManager,
            widget::Widget,
            widget_calculations,
            widget_calculations::TextSize,
        },
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
    event: Event,

    // Appearance
    texture_manager: BarButtonTextureManager,
    text: String,
    text_ndc: [f32; 2],
    prefered_char_scale: f32,
}

impl BarButtonWidget {
    pub fn new(text: String, event: Event, buffers: [f32; 4]) -> BarButtonWidget {
        let prefered_char_scale = TextSize::Large.get_scale();
        let segment_scale = widget_calculations::get_button_scale();
        let length = ((text.len() as f32 * prefered_char_scale / segment_scale).ceil() as u32 + 2).max(3);

        BarButtonWidget {
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],

            needs_resizing: true,
            external_buffers: buffers,

            pos: [0.0; 4],
            scale: [0.0; 2],

            prefered_scale: [segment_scale * length as f32, segment_scale],

            event,

            texture_manager: BarButtonTextureManager::new(),
            text,
            text_ndc: [0.0; 2],
            prefered_char_scale,
        }
    }

    pub fn set_text_scale(&mut self, size: TextSize) {
        self.prefered_char_scale = size.get_scale();
        self.needs_resizing = true;
    }

    pub fn size(&mut self) {
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);

        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);

        // Compute preferred scale from char scale and text length
        let segment_scale = widget_calculations::get_button_scale();
        let length = ((self.text.len() as f32 * self.prefered_char_scale / segment_scale).ceil() as u32 + 2).max(3);
        self.prefered_scale = [segment_scale * length as f32, segment_scale];

        // Build a pos with the correct segment_scale height, centered within self.pos
        let center_y = (self.pos[1] + self.pos[3]) / 2.0;
        let bar_pos = [
            self.pos[0],
            center_y - segment_scale / 2.0,
            self.pos[2],
            center_y + segment_scale / 2.0,
        ];

        // Size the texture renderer
        self.texture_manager.size(bar_pos);

        // Compute text center
        self.text_ndc = [
            (bar_pos[0] + bar_pos[2]) / 2.0,
            (bar_pos[1] + bar_pos[3]) / 2.0,
        ];

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

    fn get_prefered_scale(&self) -> [f32; 2] {
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
        game_event_manager: &mut GameEventManager,
    ) {
        if self.needs_resizing {
            self.size();
        }

        let is_hovered = screen_data.mouse_on_ndc_pos(self.pos);

        if is_hovered && screen_data.was_left_pressed() {
            game_event_manager.add_event(self.event.clone());
        }

        self.texture_manager.render(texture_manager, is_hovered);

        render_centered_string_at_ndc(
            texture_manager,
            self.text.clone(),
            FontType::Basic,
            self.prefered_char_scale,
            self.text_ndc,
        );
    }
}
