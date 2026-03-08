use crate::game_data::screen::widget::widget::{Widget, WidgetType};


/*
##################
## PanelSection ##
##################
Panel sections are to orginize scaling
of widgets within a panel

*/
pub struct PanelSection {
    pos: [f32; 4],
    scale: [f32; 2],


    presentage_of_panel: f32,
    widget: WidgetType,
}

impl PanelSection {
    pub fn new(widget: WidgetType) -> PanelSection {
        PanelSection {
            pos: [0.0; 4],
            scale: [0.0; 2],
            presentage_of_panel: 0.0, 
            widget: widget 
        }
    }

    pub fn get_presentage_of_panel(&self) -> f32 {
        return self.presentage_of_panel;
    }




}