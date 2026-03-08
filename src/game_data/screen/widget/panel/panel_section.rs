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


    presentage_of_panel: f32,
    widget: WidgetType,
}

impl PanelSection {
    pub fn new(widget: WidgetType) -> PanelSection {
        PanelSection {
            pos: [0.0; 4],
            presentage_of_panel: 0.0, 
            widget: widget 
        }
    }

    pub fn resize(&mut self, presentage_of_panel: f32, pos: [f32; 4]) {
        self.pos = pos;
        self.presentage_of_panel = presentage_of_panel;
    }

    pub fn get_presentage_of_panel(&self) -> f32 {
        return self.presentage_of_panel;
    }




    pub fn get_section_prefered_size(&self) -> [f32; 2] {
        return self.widget.get_prefered_scale();
    }

    pub fn get_widget(&self) -> &WidgetType {
        return &self.widget;
    } 
    pub fn get_mut_widget(&mut self) -> &mut WidgetType {
        return &mut self.widget;
    } 




}