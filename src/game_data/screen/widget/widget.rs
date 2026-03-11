use crate::game_data::{TextureManager, game_event_manager::game_event_manager::GameEventManager, screen::{ScreenData, screen_data, widget::{button::button::Button, panel::{panel::Panel}}}, texture_manager};

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
}

impl Widget for WidgetType {
    fn get_pos(&self) -> [f32; 4] {
        match self {
            WidgetType::Panel(w) => w.get_pos(),
            WidgetType::Button(w) => w.get_pos(),
        }
    }

    fn get_scale(&self) -> [f32; 2] {
        match self {
            WidgetType::Panel(w) => w.get_scale(),
            WidgetType::Button(w) => w.get_scale(),
        }
    }

    fn get_prefered_scale(&self) -> [f32; 2] {
        match self {
            WidgetType::Panel(w) => w.get_prefered_scale(),
            WidgetType::Button(w) => w.get_prefered_scale(),
        }
    }

    fn render(
        &mut self, 
        texture_manager: &mut TextureManager, 
        screen_data: &ScreenData, 
        game_event_manager: &mut GameEventManager
    ) {
        match self {
            WidgetType::Panel(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::Button(w) => w.render(texture_manager, screen_data, game_event_manager),
        }
    }
    
    fn set_buffers(&mut self, buffers: [f32; 4]) {
        match self {
            WidgetType::Panel(w) => w.set_buffers(buffers),
            WidgetType::Button(w) => w.set_buffers(buffers),
        }
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        match self {
            WidgetType::Panel(w) => w.set_parent_pos(pos),
            WidgetType::Button(w) => w.set_parent_pos(pos),
        }
    }

    fn size(&mut self) {
        match self {
            WidgetType::Panel(w) => w.size(),
            WidgetType::Button(w) => w.size(),
        }
    }

}

impl WidgetType {

    pub fn new_panel(parent_pos: [f32; 4], buffers: [f32; 4]) -> Self {
        return WidgetType::Panel(Panel::new(parent_pos, buffers));
    }

}