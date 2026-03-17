use crate::game_data::{TextureManager, game_event_manager::game_event_manager::GameEventManager, screen::{ScreenData, screen_data, widget::{bar_button::bar_button::BarButtonWidget, button::button::Button, drone_programming::vars::{draggable_var::DraggableVar, var_slot::VarSlot}, panel::panel::Panel, scroll_panel::scroll_panel::ScrollPanel, tab_panel::tab_panel::TabPanel, text::header::TextDisplay, toggle_button::toggle_button::ToggleButton, world_rendering::play_world_view_render::PlayWorldViewRender}}, texture_manager};

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
        game_event_manager: &mut GameEventManager
    );
}

pub enum WidgetType {
    // Panels
    Panel(Panel),
    TabPanel(TabPanel),
    ScrollPanel(ScrollPanel),

    // Buttons
    Button(Button),
    BarButton(BarButtonWidget),
    ToggleButton(ToggleButton),

    // Text
    TextDisplay(TextDisplay),

    // World Rendering
    PlayWorldViewRender(PlayWorldViewRender),

    // Drone Programming
    DraggableVar(DraggableVar),
    VarSlot(VarSlot),
}

#[derive(Copy, Clone)]
pub struct WidgetTypeProperties {
    pub name: &'static str,
    pub draggable: bool,
}

static WIDGET_TYPE_PROPERTIES: [WidgetTypeProperties; 10] = [
    WidgetTypeProperties { name: "Panel",                draggable: false },
    WidgetTypeProperties { name: "TabPanel",             draggable: false },
    WidgetTypeProperties { name: "ScrollPanel",          draggable: false },
    WidgetTypeProperties { name: "Button",               draggable: false },
    WidgetTypeProperties { name: "BarButton",            draggable: false },
    WidgetTypeProperties { name: "ToggleButton",         draggable: false },
    WidgetTypeProperties { name: "TextDisplay",          draggable: false },
    WidgetTypeProperties { name: "PlayWorldViewRender",  draggable: false },
    WidgetTypeProperties { name: "DraggableVar",         draggable: true  },
    WidgetTypeProperties { name: "VarSlot",              draggable: false },
];

impl WidgetType {
    fn variant_index(&self) -> usize {
        match self {
            WidgetType::Panel(_)               => 0,
            WidgetType::TabPanel(_)            => 1,
            WidgetType::ScrollPanel(_)         => 2,
            WidgetType::Button(_)              => 3,
            WidgetType::BarButton(_)           => 4,
            WidgetType::ToggleButton(_)        => 5,
            WidgetType::TextDisplay(_)         => 6,
            WidgetType::PlayWorldViewRender(_) => 7,
            WidgetType::DraggableVar(_)        => 8,
            WidgetType::VarSlot(_)             => 9,
        }
    }

    pub fn properties(&self) -> &'static WidgetTypeProperties {
        &WIDGET_TYPE_PROPERTIES[self.variant_index()]
    }

    pub fn is_draggable(&self) -> bool {
        WIDGET_TYPE_PROPERTIES[self.variant_index()].draggable
    }

    pub fn new_panel(parent_pos: [f32; 4], buffers: [f32; 4]) -> Self {
        return WidgetType::Panel(Panel::new(parent_pos, buffers));
    }
}

impl Widget for WidgetType {
    fn get_pos(&self) -> [f32; 4] {
        match self {
            // Panels
            WidgetType::Panel(w) => w.get_pos(),
            WidgetType::TabPanel(w) => w.get_pos(),
            WidgetType::ScrollPanel(w) => w.get_pos(),

            // Buttons
            WidgetType::Button(w) => w.get_pos(),
            WidgetType::BarButton(w) => w.get_pos(),
            WidgetType::ToggleButton(w) => w.get_pos(),

            // Text
            WidgetType::TextDisplay(w) => w.get_pos(),

            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.get_pos(),

            // Drone Programming
            WidgetType::DraggableVar(w) => w.get_pos(),
            WidgetType::VarSlot(w) => w.get_pos(),
        }
    }

    fn get_scale(&self) -> [f32; 2] {
        match self {
            // Panels
            WidgetType::Panel(w) => w.get_scale(),
            WidgetType::TabPanel(w) => w.get_scale(),
            WidgetType::ScrollPanel(w) => w.get_scale(),

            // Buttons
            WidgetType::Button(w) => w.get_scale(),
            WidgetType::BarButton(w) => w.get_scale(),
            WidgetType::ToggleButton(w) => w.get_scale(),

            // Text
            WidgetType::TextDisplay(w) => w.get_scale(),

            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.get_scale(),

            // Drone Programming
            WidgetType::DraggableVar(w) => w.get_scale(),
            WidgetType::VarSlot(w) => w.get_scale(),
        }
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        match self {
            // Panels
            WidgetType::Panel(w) => w.get_preffered_scale(),
            WidgetType::TabPanel(w) => w.get_preffered_scale(),
            WidgetType::ScrollPanel(w) => w.get_preffered_scale(),

            // Buttons
            WidgetType::Button(w) => w.get_preffered_scale(),
            WidgetType::BarButton(w) => w.get_preffered_scale(),
            WidgetType::ToggleButton(w) => w.get_preffered_scale(),

            // Text
            WidgetType::TextDisplay(w) => w.get_preffered_scale(),

            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.get_preffered_scale(),

            // Drone Programming
            WidgetType::DraggableVar(w) => w.get_preffered_scale(),
            WidgetType::VarSlot(w) => w.get_preffered_scale(),
        }
    }

    //=====================================
    // Setters
    //=====================================

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        match self {
            // Panels
            WidgetType::Panel(w) => w.set_buffers(buffers),
            WidgetType::TabPanel(w) => w.set_buffers(buffers),
            WidgetType::ScrollPanel(w) => w.set_buffers(buffers),

            // Buttons
            WidgetType::Button(w) => w.set_buffers(buffers),
            WidgetType::ToggleButton(w) => w.set_buffers(buffers),
            WidgetType::BarButton(w) => w.set_buffers(buffers),

            // Text
            WidgetType::TextDisplay(w) => w.set_buffers(buffers),

            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.set_buffers(buffers),

            // Drone Programming
            WidgetType::DraggableVar(w) => w.set_buffers(buffers),
            WidgetType::VarSlot(w) => w.set_buffers(buffers),
        }
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        match self {
            // Panels
            WidgetType::Panel(w) => w.set_parent_pos(pos),
            WidgetType::TabPanel(w) => w.set_parent_pos(pos),
            WidgetType::ScrollPanel(w) => w.set_parent_pos(pos),

            // Buttons
            WidgetType::Button(w) => w.set_parent_pos(pos),
            WidgetType::BarButton(w) => w.set_parent_pos(pos),
            WidgetType::ToggleButton(w) => w.set_parent_pos(pos),

            // Text
            WidgetType::TextDisplay(w) => w.set_parent_pos(pos),

            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.set_parent_pos(pos),

            // Drone Programming
            WidgetType::DraggableVar(w) => w.set_parent_pos(pos),
            WidgetType::VarSlot(w) => w.set_parent_pos(pos),
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
            // Panels
            WidgetType::Panel(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::TabPanel(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::ScrollPanel(w) => w.render(texture_manager, screen_data, game_event_manager),

            // Button
            WidgetType::Button(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::BarButton(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::ToggleButton(w) => w.render(texture_manager, screen_data, game_event_manager),

            // Text
            WidgetType::TextDisplay(w) => w.render(texture_manager, screen_data, game_event_manager),

            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.render(texture_manager, screen_data, game_event_manager),

            // Drone Programming
            WidgetType::DraggableVar(w) => w.render(texture_manager, screen_data, game_event_manager),
            WidgetType::VarSlot(w) => w.render(texture_manager, screen_data, game_event_manager),
        }
    }

    fn size(&mut self) {
        match self {
            // Panels
            WidgetType::Panel(w) => w.size(),
            WidgetType::TabPanel(w) => w.size(),
            WidgetType::ScrollPanel(w) => w.size(),

            // Buttons
            WidgetType::Button(w) => w.size(),
            WidgetType::BarButton(w) => w.size(),
            WidgetType::ToggleButton(w) => w.size(),

            // Text
            WidgetType::TextDisplay(w) => w.size(),

            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.size(),

            // Drone Programming
            WidgetType::DraggableVar(w) => w.size(),
            WidgetType::VarSlot(w) => w.size(),
        }
    }
}
