use crate::game_data::screen::{ScreenData, widget::{widget::{Widget, WidgetType}, widget_calculations}};



use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq)]
pub struct WidgetId {
    id: u32,
}

impl WidgetId {
    pub fn get_next_id() -> WidgetId {
        WidgetId {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}

pub struct WidgetProperties {
    id: WidgetId,

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

    // Clipping bounds (set by parent before render)
    pub bounds: Option<[f32; 4]>,
}

impl WidgetProperties {

    pub fn new_with_parent_props(parent_props: &WidgetProperties) -> WidgetProperties {
        WidgetProperties {
            id: WidgetId::get_next_id(),
            
            parent_pos: parent_props.pos,
            parent_scale: parent_props.scale,
            prefered_scale: [0.0; 2],
            dirty: false,
            external_buffers: [0.0; 4],
            internal_buffers: [0.0; 4],
            pos: [0.0; 4],
            scale: [0.0; 2],
            bounds: None,
        }
    }

    pub fn new_blank() -> WidgetProperties {
        WidgetProperties {
            id: WidgetId::get_next_id(),
            
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],
            prefered_scale: [0.0; 2],
            dirty: false,
            external_buffers: [0.0; 4],
            internal_buffers: [0.0; 4],
            pos: [0.0; 4],
            scale: [0.0; 2],
            bounds: None,
        }
    }

    pub fn get_id(&self) -> WidgetId {
        self.id
    }

    pub fn mouse_on(&self, screen_data: &ScreenData) -> bool {
        screen_data.mouse_on_ndc_pos(self.pos)
    }

    pub fn scale_based_off_parent(&mut self) {
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);
        self.pos   = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
    }

    pub fn calculate_prefered_scale_from_widget_list(&mut self, widgets: &Vec<WidgetType>) {
        let mut largest_x_scale = 0.0;
        let mut total_y_scale = 0.0;


        for widget in widgets {
            let scale = widget.get_scale();
            if scale[0] > largest_x_scale {
                largest_x_scale = scale[0]
            }
            total_y_scale += scale[1];
        }

        self.prefered_scale = [largest_x_scale, total_y_scale];
    }
}
