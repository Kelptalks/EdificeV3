
use crate::game_data::{TextureManager, screen::widget::{panel::panel::{PanelAlignment, PanelOrientation}, widget::{Widget, WidgetType}, widget_calculations}};


/*
##################
## PanelSection ##
##################
Panel sections are to orginize scaling
of widgets within a panel

*/
pub struct PanelSection {
    // Parent
    orientation: PanelOrientation,
    alignment: PanelAlignment,
    
    // Self 
    pos: [f32; 4],
    scale: [f32; 2],

    // Children
    widget: WidgetType,
}

impl PanelSection {
    pub fn new(widget: WidgetType, orientation: PanelOrientation, alignment: PanelAlignment) -> PanelSection {
        PanelSection {
            // Parent
            orientation: orientation,
            alignment: alignment,

            // Self
            pos: [0.0; 4],
            scale: [0.0; 2], 

            // Children
            widget: widget
        }
    }

    //=====================================
    // Section Calculations
    //=====================================

    pub fn size(&mut self, parent_pos: [f32; 4], starting_stretch: f32, internal_buffers: [f32; 4]) -> f32 {
        self.widget.size();

        let [stretch, cross, stretch_end, cross_end] = self.orientation.get_index_mods();

        let stretch_scale =
            self.widget.get_preffered_scale()[stretch]
            + internal_buffers[stretch]
            + internal_buffers[stretch_end];

        // Section spans full parent cross, stretch is content-sized
        self.pos[stretch] = parent_pos[stretch] + starting_stretch;
        self.pos[cross] = parent_pos[cross];
        self.pos[stretch_end] = parent_pos[stretch] + starting_stretch + stretch_scale;
        self.pos[cross_end] = parent_pos[cross_end];

        self.widget.set_parent_pos(self.pos);
        self.scale = widget_calculations::pos_to_scale(self.pos);

        stretch_scale
    }

    //=====================================
    // Orientation
    //=====================================

    pub fn set_orientation(&mut self, orientation: PanelOrientation, alignment: PanelAlignment) {
        self.orientation = orientation;
        self.alignment = alignment;
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

    pub fn get_widget(&self) -> &WidgetType {
        return &self.widget;
    } 

    pub fn _test_render(&self, texture_manager: &mut TextureManager) {
        texture_manager.render_ui_element_with_pos(
            crate::game_data::types::UITextures::ScallingIconMidCenter, 
            self.pos
        );

    }


}