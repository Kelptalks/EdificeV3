use crate::game_data::screen::widget::widget_calculations;

pub struct WidgetProperties {
    // Parent rendering
    pub parent_pos: [f32; 4],
    pub parent_scale: [f32; 2],
    pub prefered_scale: [f32; 2],
    pub dirty: bool,

    // Self Rendering
    pub external_buffers: [f32; 4],  
    pub internal_buffers: [f32; 4],
    pub pos: [f32; 4],
    pub scale: [f32; 2],

    
}

impl WidgetProperties {

    pub fn new_blank() -> WidgetProperties {
        WidgetProperties { 
            // Parent
            parent_pos: [0.0; 4], 
            parent_scale: [0.0; 2], 
            prefered_scale: [0.0; 2], 
            dirty: false,

            // Self
            external_buffers: [0.0; 4], 
            internal_buffers: [0.0; 4], 
            pos: [0.0; 4], 
            scale: [0.0; 2]
        }
    }

    pub fn scale_based_off_parent(&mut self) {
        self.pos =   widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
    }
}