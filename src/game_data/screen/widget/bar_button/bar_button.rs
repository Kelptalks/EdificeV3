use crate::game_data::{
    TextureManager,
    game_event_manager::{event_manager::EventManager, game_event_manager::GameEvent},
    screen::{
        ScreenData, text::render_string_at_ndc, widget::{
            bar_button::bar_button_texture_manager::BarButtonTextureManager,
            widget::Widget,
            widget_calculations::{self, TextSize},
            widget_properties::WidgetProperties,
        }
    },
    types::FontType,
};

pub struct BarButtonWidget {
    widget_properties: WidgetProperties,

    needs_resizing: bool,

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
        let mut wp = WidgetProperties::new_blank();
        wp.external_buffers = buffers;
        wp.prefered_scale = [0.0; 2];

        BarButtonWidget {
            widget_properties: wp,

            needs_resizing: true,

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
    // Appearance
    //=====================================
    pub fn set_text_scale(&mut self, size: TextSize) {
        self.prefered_char_scale = size.get_scale();
        self.needs_resizing = true;
    }

    //=====================================
    // Sizing
    //=====================================
    pub fn size(&mut self) {
        self.widget_properties.scale_based_off_parent();

        let pos = self.widget_properties.pos;

        self.bar_button_text_buffers = self.prefered_char_scale / 1.0;

        self.widget_properties.prefered_scale = [
            self.prefered_char_scale * self.text.len() as f32 + self.bar_button_text_buffers,
            self.prefered_char_scale + self.bar_button_text_buffers,
        ];

        self.text_ndc = [
            pos[0] + self.bar_button_text_buffers / 2.0,
            pos[1] + self.bar_button_text_buffers / 2.0,
        ];

        self.texture_manager.size(pos);

        self.needs_resizing = false;
    }
}

impl Widget for BarButtonWidget {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.widget_properties.external_buffers = buffers;
        self.needs_resizing = true;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
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

        let is_hovered = screen_data.mouse_on_ndc_pos(self.widget_properties.pos);

        if is_hovered && screen_data.was_left_released() {
            for event in &self.events {
                game_event_manager.add_game_event(event.clone());
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
