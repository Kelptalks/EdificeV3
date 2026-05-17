
use crate::game_data::{TextureManager, game_event_manager::{event_manager::EventManager, prelude::Event}, player_data::player_data::PlayerData, screen::{ScreenData, render_centered_string_at_ndc, widget::{widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties}}, texture_manager::texture::Texture, types::{BlockTexture, FontType, UITextures}};

pub struct Button {
    widget_properties: WidgetProperties,

    needs_resizing: bool,

    prefered_scale: [f32; 2],

    // Input
    left_click_events: Vec<Event>,
    right_click_events: Vec<Event>,

    // Appearance
    button_type: UITextures,
    apearence_pos: [f32; 4],
    text_ndc: [f32; 2],

    text: Option<String>,
    block_texture: Option<BlockTexture>,
    icon_type: Option<UITextures>,
}

impl Button {
    pub fn new() -> Button {

        let mut button = Button {
            widget_properties: WidgetProperties::new_blank(),

            needs_resizing: true,

            prefered_scale: [widget_calculations::get_button_scale(); 2],

            left_click_events: Vec::new(),
            right_click_events: Vec::new(),

            button_type: UITextures::ButtonCircle,
            apearence_pos: [0.0; 4],
            text_ndc: [0.0; 2],

            text: None,
            block_texture: None,
            icon_type: None,
        };

        button.size();
        button.widget_properties.prefered_scale = [widget_calculations::get_button_scale(); 2];

        return button;
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::Button(self)
    }

    pub fn size(&mut self) {
        self.widget_properties.scale_based_off_parent();

        let pos = self.widget_properties.pos;
        let scale = self.widget_properties.scale;

        self.apearence_pos = widget_calculations::buffer_pos(pos, [scale[0] / 5.0; 4]);
        self.text_ndc = [
            pos[0] + (scale[0] / 2.0),
            pos[1] - (widget_calculations::get_button_text_scale() / 2.0),
        ];

        self.needs_resizing = false;
    }

    pub fn set_text_scale(&mut self, size: widget_calculations::TextSize) {
        self.prefered_scale[1] = size.get_scale();
    }

    //=====================================
    // Events
    //=====================================

    pub fn clear_events(&mut self) {
        self.left_click_events.clear();
        self.right_click_events.clear();
    }

    pub fn add_left_click_event(&mut self, event: Event) {
        self.left_click_events.push(event);
    }

    pub fn add_right_click_event(&mut self, event: Event) {
        self.right_click_events.push(event);
    }

    //=====================================
    // Appearance
    //=====================================
    pub fn set_block(&mut self, block_texture: BlockTexture) {
        self.block_texture = Some(block_texture);
    }

    pub fn set_icon(&mut self, icon: UITextures) {
        self.icon_type = Some(icon);
    }

    pub fn add_texture(&mut self, texture: Texture) {
        
        match texture {
            Texture::BlockTexture(block_texture) => self.set_block(block_texture),
            Texture::UITexture(uitextures) => self.set_icon(uitextures),
            _ => {

            }
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = Some(text);
    }

    fn render_apearence(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        let bounds = self.widget_properties.bounds;

        if let Some(block_texture) = self.block_texture {
            texture_manager.render_block_with_pos(block_texture, self.apearence_pos);
        }

        if let Some(icon) = self.icon_type {
            if let Some(bounds) = bounds {
                texture_manager.render_texture_within_pos(icon.wrap_into_texture(), self.apearence_pos, bounds);
            } else {
                texture_manager.render_ui_element_with_pos(icon, self.apearence_pos);
            }
        }

        if let Some(text) = &self.text {
            if screen_data.mouse_on_ndc_pos(self.widget_properties.pos) {
                render_centered_string_at_ndc(
                    texture_manager,
                    text.clone(),
                    FontType::Basic,
                    widget_calculations::get_button_text_scale(),
                    self.text_ndc
                );
            }
        }
    }

    //=====================================
    // Special Rendering
    //=====================================
    pub fn set_ndc_pos(&mut self, ndc: [f32; 2]) {
        let scale = self.widget_properties.scale;
        self.widget_properties.pos[0] = ndc[0];
        self.widget_properties.pos[1] = ndc[1];
        self.widget_properties.pos[2] = ndc[0] + scale[0];
        self.widget_properties.pos[3] = ndc[1] + scale[1];
    }

}

impl Widget for Button {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.prefered_scale
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
        _player_data: &PlayerData,
    ) {
        if self.needs_resizing {
            self.size();
        }

        let bounds = self.widget_properties.bounds;
        let pos = self.widget_properties.pos;

        let mut button_texture = self.button_type;

        if screen_data.mouse_on_ndc_pos(pos) {
            button_texture = self.button_type.get_pressed_variant();

            if screen_data.was_left_released() {
                for event in &self.left_click_events {
                    game_event_manager.add_event(event.clone());
                }
            }

            if screen_data.was_right_pressed() {
                for event in &self.right_click_events {
                    game_event_manager.add_event(event.clone());
                }
            }
        }

        if let Some(bounds) = bounds {
            texture_manager.render_texture_within_pos(button_texture.wrap_into_texture(), pos, bounds);
        } else {
            texture_manager.render_ui_element_with_pos(button_texture, pos);
        }

        self.render_apearence(texture_manager, screen_data);
    }
}
