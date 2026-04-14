use crate::game_data::screen::widget::{drone_programming::scripting_elements::scripting_element_widgets::{control_flow_widget::ControlFlowWidget, function_widget::FunctionWidget}, panel::panel::Panel, widget::Widget};

pub enum ScriptingElementWidget {
    Action(),
    Function(FunctionWidget),
    ControlFlow(ControlFlowWidget),
}

impl ScriptingElementWidget {
    pub fn get_mut_panel(&mut self) -> &mut Panel{
        match self {
            ScriptingElementWidget::Action() => todo!(),
            ScriptingElementWidget::Function(w) => w.get_mut_panel(),
            ScriptingElementWidget::ControlFlow(w) => w.get_mut_panel(),
        }
    }

    pub fn get_panel(&self) -> &Panel{
        match self {
            ScriptingElementWidget::Action() => todo!(),
            ScriptingElementWidget::Function(w) => w.get_panel(),
            ScriptingElementWidget::ControlFlow(w) => w.get_panel(),
        }
    }

    pub fn handle_inputs(
        &mut self, 
        screen_data: &crate::game_data::screen::ScreenData, 
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        match self {
            ScriptingElementWidget::Action() => todo!(),
            ScriptingElementWidget::Function(function_widget) => function_widget.handle_inputs(screen_data, game_event_manager),
            ScriptingElementWidget::ControlFlow(control_flow_widget) => todo!(),
        }
    }
}




impl Widget for ScriptingElementWidget {
    fn get_pos(&self) -> [f32; 4] {
        self.get_panel().get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.get_panel().get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.get_panel().get_preffered_scale()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.get_mut_panel().set_buffers(pos)
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.get_mut_panel().set_parent_pos(pos)
    }

    fn size(&mut self) {
        self.get_mut_panel().size()
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        self.handle_inputs(screen_data, game_event_manager);
        self.get_mut_panel().render(texture_manager, screen_data, game_event_manager);
    }
}