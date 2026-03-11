use crate::game_data::{TextureManager, game_event_manager::game_event_manager::GameEventManager, screen::{ScreenData, screen_data, widget::{bar_button::bar_button::BarButtonWidget, button::button::Button, panel::panel::Panel, text::header::Header}}, texture_manager};

pub trait Widget {
    fn get_pos(&self) -> [f32; 4];
    fn get_scale(&self) -> [f32; 2];
    fn get_prefered_scale(&self) -> [f32; 2];

    fn set_buffers(&mut self, pos: [f32; 4]);
    fn set_parent_pos(&mut self, pos: [f32; 4]);
    fn size(&mut self);

    fn render(
        &mut self, 
        texture_manager: &mut TextureManager, 
        screen_data: &ScreenData, 
        game_event_manager: &mut GameEventManager
    );
}

pub enum WidgetType {
    Panel(Panel),
    Button(Button),
    BarButton(BarButtonWidget),
    Header(Header),
}

impl Widget for WidgetType {
    fn get_pos(&self) -> [f32; 4] {
        match self {
            WidgetType::Panel(w) => w.get_pos(),
            WidgetType::Button(w) => w.get_pos(),
            WidgetType::BarButton(w) => w.get_pos(),
            WidgetType::Header(w) => w.get_pos(),
        }
    }

    fn get_scale(&self) -> [f32; 2] {
        match self {
            WidgetType::Panel(w) => w.get_scale(),
            WidgetType::Button(w) => w.get_scale(),
            WidgetType::BarButton(w) => w.get_scale(),
            WidgetType::Header(w) => w.get_scale(),
        }
    }

    fn get_prefered_scale(&self) -> [f32; 2] {
        match self {
            WidgetType::Panel(w) => w.get_prefered_scale(),
            WidgetType::Button(w) => w.get_prefered_scale(),
            WidgetType::BarButton(w) => w.get_prefered_scale(),
            WidgetType::Header(w) => w.get_prefered_scale(),
        }
    }

    //=====================================
    // Setters
    //=====================================

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        match self {
            WidgetType::Panel(w) => w.set_buffers(buffers),
            WidgetType::Button(w) => w.set_buffers(buffers),
            WidgetType::BarButton(w) => w.set_buffers(buffers),
            WidgetType::Header(w) => w.set_buffers(buffers),
        }
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        match self {
            WidgetType::Panel(w) => w.set_parent_pos(pos),
            WidgetType::Button(w) => w.set_parent_pos(pos),
            WidgetType::BarButton(w) => w.set_parent_pos(pos),
            WidgetType::Header(w) => w.set_parent_pos(pos),
        }
    }

    //=====================================
    // Rendering
    //=====================================

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut GameEventManager
    ) {
        match self {
            WidgetType::Panel(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::Button(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::BarButton(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::Header(w) => w.render(texture_manager, screen_data, game_event_manager),
        }
    }

    fn size(&mut self) {
        match self {
            WidgetType::Panel(w) => w.size(),
            WidgetType::Button(w) => w.size(),
            WidgetType::BarButton(w) => w.size(),
            WidgetType::Header(w) => w.size(),
        }
    }

}

impl WidgetType {

    pub fn new_panel(parent_pos: [f32; 4], buffers: [f32; 4]) -> Self {
        return WidgetType::Panel(Panel::new(parent_pos, buffers));
    }

}