use crate::game_data::{TextureManager, screen::widget::{self, panel::h_panel::HPanelAlignment, widget::{Widget, WidgetType}, widget_calculations}, texture_manager};


/*
##################
## PanelSection ##
##################
Panel sections are to orginize scaling
of widgets within a panel

*/
pub struct PanelSection {
    // Parent
    parent_pos: [f32; 4],
    parent_scale: [f32; 4],
    alignment_type: HPanelAlignment,

    // Self Rendering
    external_buffers: [f32; 4],
    internal_buffers: [f32; 4],

    pos: [f32; 4],
    scale: [f32; 2],

    presentage_of_panel: f32,
    widget: WidgetType,
}

impl PanelSection {
    pub fn new(widget: WidgetType) -> PanelSection {
        PanelSection {
            // Parent
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 4],
            alignment_type: HPanelAlignment::Bot,

            // Self Rendering
            external_buffers: [0.0; 4],
            internal_buffers: [0.0; 4],

            pos: [0.0; 4],
            scale: [0.0; 2], 

            presentage_of_panel: 0.0, 
            widget: widget
        }
    }

    //=====================================
    // Section Calculations
    //=====================================

    pub fn size(&mut self) {
        //Self
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
        
        let widget_prefered_y = self.widget.get_prefered_scale()[1];
        let y_total_buffer_scale = self.scale[1] - widget_prefered_y; 

        match self.alignment_type {
            HPanelAlignment::Top => {
                self.internal_buffers = [
                    0.0,
                    0.0,
                    0.0,
                    y_total_buffer_scale,
                ];
            },
            HPanelAlignment::Bot => {
                self.internal_buffers = [
                    0.0,
                    y_total_buffer_scale,
                    0.0,
                    0.0,
                ];
            },
            HPanelAlignment::Center => {
                self.internal_buffers = [
                    0.0,
                    y_total_buffer_scale / 2.0,
                    0.0,
                    y_total_buffer_scale / 2.0,
                ];
            },
        }
        


        self.widget.set_parent_pos(self.pos);
        self.widget.set_buffers(self.internal_buffers);
        
    }

    //=====================================
    // Parrent Setters
    //=====================================

    pub fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
        self.size();
    }

    pub fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.external_buffers = buffers;
        self.size();
    }

    //=====================================
    // Section Getters
    //=====================================

    pub fn get_section_prefered_width(&self) -> f32 {
        return self.widget.get_prefered_scale()[0];
    }

    pub fn get_section_width(&self) -> f32 {
        return self.scale[0];
    }

    pub fn get_section_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    //=====================================
    // Widget Getters
    //=====================================

    pub fn get_widget(&self) -> &WidgetType {
        return &self.widget;
    } 
    pub fn get_mut_widget(&mut self) -> &mut WidgetType {
        return &mut self.widget;
    } 


    pub fn test_render(&self, texture_manager: &mut TextureManager) {
        texture_manager.render_ui_element_with_pos(
            crate::game_data::types::UITextures::VoidBackground, 
            self.widget.get_pos()
        );

    }


}