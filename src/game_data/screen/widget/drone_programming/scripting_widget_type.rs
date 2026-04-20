use crate::game_data::screen::widget::drone_programming::{action_slot::ActionSlot, condition_slot::ConditionSlot, control_flow_slot::ControlFlowSlot, function_slot::FunctionSlot};


pub enum ScriptingWidget<'a> {
    FunctionSlot(),
    ControlFlowSlot(&'a mut ControlFlowSlot),
    ConditionSlot(&'a mut ConditionSlot),
    ActionSlot(&'a mut ActionSlot),
}

impl<'a> ScriptingWidget<'a> {
    pub fn get_line(&self) -> usize {
        match self {
            ScriptingWidget::FunctionSlot() => 0,
            ScriptingWidget::ControlFlowSlot(w) => w.get_line_index(),
            ScriptingWidget::ConditionSlot(w) => w.get_line_index(),
            ScriptingWidget::ActionSlot(w) => w.get_line_index(),
        }
    }

    pub fn set_highlighted(&mut self) {
        match self {
            ScriptingWidget::FunctionSlot() => {},
            ScriptingWidget::ControlFlowSlot(w) => w.set_highlighted(),
            ScriptingWidget::ConditionSlot(w) => w.set_highlighted(),
            ScriptingWidget::ActionSlot(w) => w.set_highlighted(),
        }
    }

}


pub trait ScriptingElementWidget {
    fn get_line_index(&self) -> usize;
    fn set_highlighted(&mut self);
}


