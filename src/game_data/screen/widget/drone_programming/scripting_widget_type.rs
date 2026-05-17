use crate::game_data::screen::{ScreenData, widget::drone_programming::{action_slot::ActionSlot, condition_slot::ConditionSlot, control_flow_slot::ControlFlowSlot}};


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

    pub fn set_highlighted(&mut self, color: [u8; 3]) {
        match self {
            ScriptingWidgetType::FunctionSlot() => {},
            ScriptingWidgetType::ControlFlowSlot(w) => w.highlight(color),
            ScriptingWidgetType::ConditionSlot(w) => w.highlight(color),
            ScriptingWidgetType::ActionSlot(w) => w.highlight(color),
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

    fn highlight(&mut self, color: [u8; 3]);
    fn get_line_incert_index(&self, screen_data: &ScreenData) -> usize;
}


