use crate::game_data::{TextureManager, screen::widget::{self, panel::panel::PanelAlignment, widget::{Widget, WidgetType}, widget_calculations}, texture_manager};


/*
##################
## PanelSection ##
##################
Panel sections are to orginize scaling
of widgets within a panel

*/
pub struct PanelSection {
    // Parent
    alignment_type: PanelAlignment,

    pos: [f32; 4],
    scale: [f32; 2],

    widget: WidgetType,
}

impl PanelSection {
    pub fn new(widget: WidgetType) -> PanelSection {
        PanelSection {
            alignment_type: PanelAlignment::Center,


            pos: [0.0; 4],
            scale: [0.0; 2], 

            widget: widget
        }
    }

    //=====================================
    // Section Calculations
    //=====================================

    pub fn size(&mut self, parent_pos: [f32; 4], starting_x: f32, external_buffers: [f32; 4]) -> f32 {
        self.widget.size();

        let x_scale = self.widget.get_prefered_scale()[0] + external_buffers[0]  + external_buffers[2];
        let y_scale = self.widget.get_prefered_scale()[1] + external_buffers[1]  + external_buffers[3];

        // Calculate location
        self.pos = [
            parent_pos[0] + starting_x,
            parent_pos[1],
            parent_pos[0] + starting_x + x_scale,
            parent_pos[3],
        ];
        self.widget.set_parent_pos(self.pos);

        self.scale = widget_calculations::pos_to_scale(self.pos);

        // Base widget is an H Panel base scale y off section scale
        
        let mut y_total_buffer_scale = self.scale[1] - y_scale;
        if let WidgetType::Panel(_panel) = &mut self.widget {
            y_total_buffer_scale = 0.0; 
        }
        
        let widget_buffers;
        match self.alignment_type {
            PanelAlignment::Top => {
                widget_buffers = [
                    external_buffers[0],
                    external_buffers[1],
                    external_buffers[2],
                    external_buffers[3] + y_total_buffer_scale,
                ];
            },
            PanelAlignment::Bot => {
                widget_buffers = [
                    external_buffers[0],
                    external_buffers[1] + y_total_buffer_scale,
                    external_buffers[2],
                    external_buffers[3],
                ];
            },
            PanelAlignment::Center => {
                widget_buffers = [
                    external_buffers[0],
                    external_buffers[1] + y_total_buffer_scale / 2.0,
                    external_buffers[2],
                    external_buffers[3] + y_total_buffer_scale / 2.0,
                ];
            },
        }

        self.widget.set_buffers(widget_buffers);
        self.widget.size();

        return x_scale;
    }

    //=====================================
    // Section Getters
    //=====================================
    
    pub fn get_section_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    //=====================================
    // Widget Getters
    //=====================================

    pub fn get_mut_widget(&mut self) -> &mut WidgetType {
        return &mut self.widget;
    } 

    pub fn _test_render(&self, texture_manager: &mut TextureManager) {
        texture_manager.render_ui_element_with_pos(
            crate::game_data::types::UITextures::FaceBackground, 
            self.pos
        );

    }


}