use crate::game_data::{
    TextureManager,
    game_event_manager::game_event_manager::EventManager,
    screen::{ScreenData,
        widget::{
            bar_button::bar_button::BarButtonWidget, button::button::Button, drone_programming::{
                action_slot::ActionSlot, condition_slot::ConditionSlot, control_flow_slot::ControlFlowSlot, function_slot::FunctionSlot, script_element_body_slot::ScriptElementBodySlot, scripting_elements::scripting_panel::ScriptingPanel, scripting_widget_type::{ScriptingElementWidget, ScriptingWidgetType}, var_slot::{var_prop_widgets::var_prop_widgets::VarPropVal, var_slot::VarSlot}}, panel::panel::Panel, scroll_panel::scroll_panel::ScrollPanel, selection_panel::selection_panel::SelectionPanel, tab_panel::{tab_panel::TabPanel, var_tab_panel::VarTabPanel}, text::{header::TextDisplay, text_input::TextInput}, toggle_button::toggle_button::ToggleButton, widget_properties::WidgetProperties, world_rendering::play_world_view_render::PlayWorldViewRender
        }
    }, texture_manager::rect::Pos};

pub trait Widget {
    fn get_widget_properties(&self) -> &WidgetProperties;
    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties;

    fn get_pos(&self) -> [f32; 4] { self.get_widget_properties().pos }
    fn get_scale(&self) -> [f32; 2] { self.get_widget_properties().scale }
    fn get_preffered_scale(&self) -> [f32; 2] { self.get_widget_properties().prefered_scale }

    fn set_buffers(&mut self, pos: [f32; 4]) { self.get_mut_widget_properties().external_buffers = pos; }
    fn set_parent_pos(&mut self, pos: [f32; 4]) { self.get_mut_widget_properties().parent_pos = pos; }

    fn mouse_on(&self, screen_data: &ScreenData) -> bool {
        self.get_widget_properties().mouse_on(screen_data)
    }

    fn size(&mut self);

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
    );
}

pub enum WidgetType {
    // Panels
    Panel(Panel),
    TabPanel(TabPanel),
    ScrollPanel(ScrollPanel),
    SelectionPanel(SelectionPanel),
    VarTabPanel(VarTabPanel),

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
    VarSlot(VarSlot),
    VarPropValWidget(VarPropVal),
    FunctionSlot(FunctionSlot),
    ActionSlot(ActionSlot),
    ControlFlowSlot(ControlFlowSlot),
    ConditionSlot(ConditionSlot),
    ScriptElementBodySlot(ScriptElementBodySlot),

    ScriptingPanel(ScriptingPanel),

}

impl WidgetType {

    pub fn new_panel(parent_pos: [f32; 4], buffers: [f32; 4]) -> Self {
        return WidgetType::Panel(Panel::new(parent_pos, buffers));
    }


    pub fn as_scripting_widget(&mut self) -> Option<ScriptingWidgetType> {
        match self {
            WidgetType::FunctionSlot(_) => Some(ScriptingWidgetType::FunctionSlot()),
            WidgetType::ActionSlot(action_slot) => Some(ScriptingWidgetType::ActionSlot(action_slot)),
            WidgetType::ControlFlowSlot(control_flow_slot) => Some(ScriptingWidgetType::ControlFlowSlot(control_flow_slot)),
            WidgetType::ConditionSlot(condition_slot) => Some(ScriptingWidgetType::ConditionSlot(condition_slot)),
            _ => {
                None
            }
        }
    }

    pub fn extract_body_widget(&mut self) -> Option<&mut ScriptElementBodySlot> {
        match self {
            WidgetType::ControlFlowSlot(control_flow) => {
                return control_flow.get_mut_body_widget()
            }
            _ => {
                None
            }
        }
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
            WidgetType::VarTabPanel(w)  => w.$method($($arg), *),

            // Buttons
            WidgetType::Button(w)      => w.$method($($arg),*),
            WidgetType::BarButton(w)   => w.$method($($arg),*),
            WidgetType::ToggleButton(w)=> w.$method($($arg),*),
            // Text
            WidgetType::TextDisplay(w) => w.$method($($arg),*),
            // World Rendering
            WidgetType::PlayWorldViewRender(w) => w.$method($($arg),*),

            // Drone Programming
            WidgetType::VarSlot(w)        => w.$method($($arg),*),
            WidgetType::TextInput(w)      => w.$method($($arg),*),
            WidgetType::FunctionSlot(w)   => w.$method($($arg),*),
            WidgetType::VarPropValWidget(w)   => w.$method($($arg),*),
            WidgetType::ActionSlot(w)   => w.$method($($arg),*),
            WidgetType::ControlFlowSlot(w)   => w.$method($($arg),*),
            WidgetType::ConditionSlot(w)   => w.$method($($arg),*),
            WidgetType::ScriptingPanel(w)   => w.$method($($arg),*),
            WidgetType::ScriptElementBodySlot(w) => w.$method($($arg),*),
            
        }
    };
}

impl Widget for WidgetType {
    fn get_widget_properties(&self) -> &WidgetProperties { widget_match!(self, get_widget_properties) }
    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties { widget_match!(self, get_mut_widget_properties) }

    fn size(&mut self) { widget_match!(self, size) }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
    ) {
        widget_match!(self, render, texture_manager, screen_data, game_event_manager)
    }
}
