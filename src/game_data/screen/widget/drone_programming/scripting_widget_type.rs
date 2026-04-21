use crate::game_data::screen::{ScreenData, screen_data, widget::drone_programming::{action_slot::ActionSlot, condition_slot::ConditionSlot, control_flow_slot::ControlFlowSlot, function_slot::FunctionSlot}};


pub enum ScriptingWidgetType<'a> {
    FunctionSlot(),
    ControlFlowSlot(&'a mut ControlFlowSlot),
    ConditionSlot(&'a mut ConditionSlot),
    ActionSlot(&'a mut ActionSlot),
}

impl<'a> ScriptingWidgetType<'a> {
    pub fn get_line(&self) -> usize {
        match self {
            ScriptingWidgetType::FunctionSlot() => 0,
            ScriptingWidgetType::ControlFlowSlot(w) => w.get_line_index(),
            ScriptingWidgetType::ConditionSlot(w) => w.get_line_index(),
            ScriptingWidgetType::ActionSlot(w) => w.get_line_index(),
        }
    }

    pub fn get_line_incert_index(&self, screen_data: &ScreenData) -> usize {
        match self {
            ScriptingWidgetType::FunctionSlot() => 0,
            ScriptingWidgetType::ControlFlowSlot(w) => w.get_line_incert_index(screen_data),
            ScriptingWidgetType::ConditionSlot(w) => w.get_line_incert_index(screen_data),
            ScriptingWidgetType::ActionSlot(w) => w.get_line_incert_index(screen_data),
        }
    }

    pub fn set_highlighted(&mut self) {
        match self {
            ScriptingWidgetType::FunctionSlot() => {},
            ScriptingWidgetType::ControlFlowSlot(w) => w.set_highlighted(),
            ScriptingWidgetType::ConditionSlot(w) => w.set_highlighted(),
            ScriptingWidgetType::ActionSlot(w) => w.set_highlighted(),
        }
    }

    pub fn has_body(&self) -> bool{
        match self {
            ScriptingWidgetType::FunctionSlot() => {
                true
            },
            ScriptingWidgetType::ControlFlowSlot(_) => {
                true
            }
            _ => false
        }
    }


}


pub trait ScriptingElementWidget {
    fn get_line_index(&self) -> usize;

    fn set_highlighted(&mut self);
    fn get_line_incert_index(&self, screen_data: &ScreenData) -> usize;
}


