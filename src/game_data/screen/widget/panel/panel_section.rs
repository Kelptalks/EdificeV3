use std::fmt::Alignment;

use crate::game_data::{TextureManager, screen::widget::{self, panel::panel::{PanelAlignment, PanelOrientation}, widget::{Widget, WidgetType}, widget_calculations}, texture_manager};


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

    pub fn size(&mut self, parent_pos: [f32; 4], starting_stretch: f32, external_buffers: [f32; 4]) -> f32 {
        self.widget.size();

        let indexing_mods = self.orientation.get_index_mods();

        let stretch_scale = 
            self.widget.get_prefered_scale()[indexing_mods[0]]
            + external_buffers[indexing_mods[0]]
            + external_buffers[indexing_mods[2]];
        
        let cross_scale = 
            self.widget.get_prefered_scale()[indexing_mods[1]]
            + external_buffers[indexing_mods[1]]
            + external_buffers[indexing_mods[3]];

        // Calculate location
        self.pos = [
            parent_pos[0],
            parent_pos[1],
            parent_pos[0],
            parent_pos[3],
        ];

        self.pos[indexing_mods[0]] = parent_pos[indexing_mods[0]];
        self.pos[indexing_mods[1]] = parent_pos[indexing_mods[1]];
        self.pos[indexing_mods[2]] = parent_pos[indexing_mods[0]];
        self.pos[indexing_mods[3]] = parent_pos[indexing_mods[1]] + cross_scale;

        self.pos[indexing_mods[0]] += starting_stretch;
        self.pos[indexing_mods[2]] += starting_stretch + stretch_scale;

        self.widget.set_parent_pos(self.pos);

        self.scale = widget_calculations::pos_to_scale(self.pos);

        // Base widget is an H Panel base scale y off section scale
        
        let mut cross_total_buffer_scale = self.scale[indexing_mods[1]] - cross_scale;
        if let WidgetType::Panel(_panel) = &mut self.widget {
            cross_total_buffer_scale = 0.0; 
        }
        
        let mut widget_buffers= external_buffers;
        match self.alignment {
            PanelAlignment::TopLeft => {
                widget_buffers[indexing_mods[3]] += cross_total_buffer_scale;
            
            },
            PanelAlignment::BotRight => {
                widget_buffers[indexing_mods[1]] += cross_total_buffer_scale;

            },
            PanelAlignment::Center => {

                widget_buffers[indexing_mods[1]] += cross_total_buffer_scale / 2.0;
                widget_buffers[indexing_mods[3]] += cross_total_buffer_scale / 2.0;

            },
        }

        self.widget.set_buffers(widget_buffers);
        self.widget.size();




        return stretch_scale;
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

    pub fn _test_render(&self, texture_manager: &mut TextureManager) {
        texture_manager.render_ui_element_with_pos(
            crate::game_data::types::UITextures::ScallingIconMidCenter, 
            self.pos
        );

    }


}