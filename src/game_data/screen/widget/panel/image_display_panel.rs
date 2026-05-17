use crate::game_data::{
    TextureManager,
    game_event_manager::game_event_manager::EventManager,
    player_data::player_data::PlayerData,
    screen::{
        ScreenData,
        widget::{
            panel::panel::Panel,
            widget::{Widget, WidgetType},
            widget_properties::WidgetProperties,
        },
    },
    texture_manager::texture::Texture,
};

pub struct ImageDisplayPanel {
    widget_properties: WidgetProperties,
    panel: Panel,
    texture: Texture,
    preferred_scale: [f32; 2],
    needs_resizing: bool,
}

impl ImageDisplayPanel {
    pub fn new(texture: Texture, preferred_scale: [f32; 2]) -> ImageDisplayPanel {
        let mut wp = WidgetProperties::new_blank();
        wp.prefered_scale = preferred_scale;

        ImageDisplayPanel {
            widget_properties: wp,
            panel: Panel::new_blank(),
            texture,
            preferred_scale,
            needs_resizing: true,
        }
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
    }

    pub fn set_preferred_scale(&mut self, scale: [f32; 2]) {
        self.preferred_scale = scale;
        self.widget_properties.prefered_scale = scale;
        self.needs_resizing = true;
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::ImageDisplayPanel(self)
    }

    fn size_self(&mut self) {
        self.widget_properties.scale_based_off_parent();
        let pos = self.widget_properties.pos;
        self.panel.get_mut_widget_properties().parent_pos = pos;
        self.panel.size();
        self.widget_properties.prefered_scale = self.preferred_scale;
        self.needs_resizing = false;
    }
}

impl Widget for ImageDisplayPanel {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.preferred_scale
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
        self.size_self();
    }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {
        if self.needs_resizing {
            self.size_self();
        }

        let pos = self.widget_properties.pos;
        let bounds = self.widget_properties.bounds;

        self.panel.get_mut_widget_properties().bounds = bounds;
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);

        texture_manager.render_texture_within_pos_option(
            self.texture,
            pos,
            bounds,
        );
    }
}
