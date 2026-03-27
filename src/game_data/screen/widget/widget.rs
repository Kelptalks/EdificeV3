use crate::game_data::{TextureManager, game_event_manager::game_event_manager::EventManager, screen::{ScreenData, screen_data, widget::{bar_button::bar_button::BarButtonWidget, button::button::Button, drone_programming::vars::{var_source::VarSource, var_slot::VarSlot}, panel::panel::Panel, scroll_panel::scroll_panel::ScrollPanel, selection_panel::selection_panel::SelectionPanel, tab_panel::tab_panel::TabPanel, text::{header::TextDisplay, text_input::TextInput}, toggle_button::toggle_button::ToggleButton, world_rendering::play_world_view_render::PlayWorldViewRender}}, texture_manager};

pub trait Widget {
    fn get_pos(&self) -> [f32; 4];
    fn get_scale(&self) -> [f32; 2];
    fn get_preffered_scale(&self) -> [f32; 2];

    fn set_buffers(&mut self, pos: [f32; 4]);
    fn set_parent_pos(&mut self, pos: [f32; 4]);
    fn size(&mut self);

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager
    );
}

pub enum WidgetType {
    // Panels
    Panel(Panel),
    TabPanel(TabPanel),
    ScrollPanel(ScrollPanel),
    SelectionPanel(SelectionPanel),

    // Buttons
    Button(Button),
    BarButton(BarButtonWidget),
    ToggleButton(ToggleButton),

    // Text
    TextDisplay(TextDisplay),
    TextInput(TextInput),

    // World Rendering
    PlayWorldViewRender(PlayWorldViewRender),

    // Drone Programming
    VarSource(VarSource),
    VarSlot(VarSlot),
}

impl WidgetType {

    pub fn new_panel(parent_pos: [f32; 4], buffers: [f32; 4]) -> Self {
        return WidgetType::Panel(Panel::new(parent_pos, buffers));
    }

    pub fn new_text_display(text: String) -> Self {
        return WidgetType::TextDisplay(TextDisplay::new(text));
    }
}

macro_rules! widget_match {
    ($self:expr, $method:ident $(, $arg:expr)*) => {
        match $self {
            // Panels
            WidgetType::Panel(w)       => w.$method($($arg),*),
            WidgetType::TabPanel(w)    => w.$method($($arg),*),
            WidgetType::ScrollPanel(w) => w.$method($($arg),*),
            WidgetType::SelectionPanel(w) => w.$method($($arg), *),

            // Buttons
            WidgetType::Button(w)      => w.$method($($arg),*),
            WidgetType::BarButton(w)   => w.$method($($arg),*),
            WidgetType::ToggleButton(w)=> w.$method($($arg),*),
            // Text
            WidgetType::TextDisplay(w) => w.$method($($arg),*),
            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.$method($($arg),*),

            // Drone Programming
            WidgetType::VarSource(w)   => w.$method($($arg),*),
            WidgetType::VarSlot(w)        => w.$method($($arg),*),
            WidgetType::TextInput(w)      => w.$method($($arg),*),
        }
    };
}

impl Widget for WidgetType {
    fn get_pos(&self)            -> [f32; 4] { widget_match!(self, get_pos) }
    fn get_scale(&self)          -> [f32; 2] { widget_match!(self, get_scale) }
    fn get_preffered_scale(&self)-> [f32; 2] { widget_match!(self, get_preffered_scale) }
    fn set_buffers(&mut self, pos: [f32; 4]) { widget_match!(self, set_buffers, pos) }
    fn set_parent_pos(&mut self, pos: [f32; 4]) { widget_match!(self, set_parent_pos, pos) }
    fn size(&mut self)           { widget_match!(self, size) }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager
    ) {
        widget_match!(self, render, texture_manager, screen_data, game_event_manager)
    }
}