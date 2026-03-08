use crate::game_data::{TextureManager, game_event_manager::{self, game_event_manager::{Event, GameEventManager}}, screen::{ScreenData, ui_elements::panel, widget::{self, button::button::Button, panel::{panel_color::PanelColor, panel_section::PanelSection}, widget::{Widget, WidgetType}}}, texture_manager, types::UITextures};

pub enum HorizontalAlignment {
    Left,
    Right,
    Center,
}

pub enum VerticalAlignment {
    Top,
    Bot,
    Center,
}

pub enum PanelType {
    Vertical(VerticalAlignment),
    Horizontal(HorizontalAlignment),
}

/*
###########
## Panel ##
###########
Panels are the main widget and act as a container for orginizing the
layout, rendering, and input handling for components contained within

*/
pub struct Panel {
    // Parent
    parent_scale: [f32; 2],

    // Panel
    external_buffers: [f32; 4],
    internal_buffers: [f32; 4],
    
    
    pos: [f32; 4],
    buffered_pos: [f32; 4],
    buffered_scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Panel Tile Rendering
    tile_ndc_scale: f32,
    tile_corner_pos: [[f32; 4]; 4],
    tile_side_pos: [[f32; 4]; 4],
    tile_center_pos: [f32; 4],

    // Aperence
    color: PanelColor,

    // Sections
    panel_type: PanelType,
    section_widgets: Vec<Option<WidgetType>>,
    sections: Vec<PanelSection>,

    widget_scale: f32,
    

}

//=====================================
// Panel
//=====================================
impl Panel {

    //=====================================
    // Constructor
    //=====================================
    pub fn new(panel_type: PanelType, parent_pos: [f32; 4], buffers: [f32; 4]) -> Self {
        
        let panel = Panel {
            // Parent Pos
            parent_scale: [parent_pos[2] - parent_pos[0], parent_pos[3] - parent_pos[1]],

            // Panel Pos
            external_buffers: buffers,
            internal_buffers: [0.01; 4],
            
            pos: parent_pos,
            buffered_pos: [0.0; 4],
            buffered_scale: [0.0; 2],
            prefered_scale: [0.1; 2],

            // Tile rendering data
            tile_ndc_scale: 0.005,
            tile_corner_pos: [[0.0; 4]; 4],
            tile_side_pos: [[0.0; 4]; 4],
            tile_center_pos: [0.0; 4],

            // Aperence
            color: PanelColor::Light,

            // Sections 
            panel_type: panel_type,
            section_widgets: Vec::new(),
            sections: Vec::new(),

            widget_scale: 0.0,
            
        };

        return panel;
    }

    pub fn resize(&mut self) {
        self.buffered_pos = [
            self.pos[0] + self.external_buffers[0],
            self.pos[1] + self.external_buffers[1],
            self.pos[2] - self.external_buffers[2],
            self.pos[3] - self.external_buffers[3],
        ];

        self.buffered_scale = [
            self.buffered_pos[2] - self.buffered_pos[0],
            self.buffered_pos[3] - self.buffered_pos[1],
        ];

        self.calculate_tile_pos();
        self.resize_widgets();
    }

    fn calculate_tile_pos(&mut self) {
        let s = self.tile_ndc_scale;
        let [x1, y1] = [self.buffered_pos[0], self.buffered_pos[1]];
        let x2 = x1 + self.buffered_scale[0];
        let y2 = y1 + self.buffered_scale[1];
        self.tile_corner_pos = [
            [x1,     y1,     x1 + s, y1 + s],  // top_left
            [x2 - s, y1,     x2,     y1 + s],  // top_right
            [x1,     y2 - s, x1 + s, y2    ],  // bot_left
            [x2 - s, y2 - s, x2,     y2    ],  // bot_right
        ];


        self.tile_side_pos = [
            [x1 + s, y1,     x2 - s, y1 + s],  // top
            [x1 + s, y2 - s, x2 - s, y2    ],  // bot
            [x1,     y1 + s, x1 + s, y2 - s],  // left
            [x2 - s, y1 + s, x2,     y2 - s],  // right
        ];

        self.tile_center_pos = [x1 + s, y1 + s, x2 - s, y2 - s];
    }


    //=====================================
    // Widget Resizing
    //=====================================
    fn get_widget_starting_ndc(&mut self) -> [f32; 2] {
        match &self.panel_type {
            PanelType::Vertical(alignment) => {
                match alignment {
                    VerticalAlignment::Top => {
                        return [
                            self.buffered_pos[0],
                            self.buffered_pos[1],
                        ];
                    },
                    VerticalAlignment::Bot => {
                        return [
                            self.buffered_pos[0],
                            self.buffered_pos[3] - self.widget_scale,
                        ];
                    },
                    VerticalAlignment::Center => {
                        let widget_padding = (self.buffered_scale[1] - self.widget_scale) / 2.0;
                        return [
                            self.buffered_pos[0],
                            self.buffered_pos[1] + widget_padding,
                        ];
                    },
                }
            },
            PanelType::Horizontal(alignment) => {
                match alignment {
                    HorizontalAlignment::Left => {
                        return [
                            self.buffered_pos[0],
                            self.buffered_pos[1],
                        ];
                    },
                    HorizontalAlignment::Right => {
                        return [
                            self.buffered_pos[2] - self.widget_scale,
                            self.buffered_pos[1],
                        ];
                    },
                    HorizontalAlignment::Center => {
                        let widget_padding = (self.buffered_scale[0] - self.widget_scale) / 2.0;
                        println!("widget_padding: {}", widget_padding);
                        println!("buffered_pos: {:?}", self.buffered_pos);
                        return [
                            (self.buffered_pos[0] + widget_padding),
                            self.buffered_pos[1],
                        ];
                    },
                }
            },
        }
    }
    
    fn calculate_widget_scale(&mut self) {
        let mut widget_scale = 0.0;
        match &self.panel_type {
            PanelType::Vertical(alignment) => {
                let mut largest_widget_x_scale = 0.0;
                for widget in &self.section_widgets {
                    if let Some(widget) = widget {
                        let widget_prefered_scale = widget.get_prefered_scale();

                        widget_scale += widget_prefered_scale[1];

                        if widget_prefered_scale[0] > largest_widget_x_scale {
                            largest_widget_x_scale = widget_prefered_scale[0];
                        }
                    }
                }
                self.prefered_scale = [
                    largest_widget_x_scale + self.internal_buffers[0] * 2.0,
                    self.parent_scale[1],
                ];
            },
            PanelType::Horizontal(alignment) => {
                let mut largest_widget_y_scale = 0.0;
                
                for widget in &self.section_widgets {
                    if let Some(widget) = widget {
                        let widget_prefered_scale = widget.get_prefered_scale();
                        widget_scale += widget_prefered_scale[0];

                        if widget_prefered_scale[1] > largest_widget_y_scale {
                            largest_widget_y_scale = widget_prefered_scale[1];
                        }
                    }
                }

                self.prefered_scale = [
                    self.parent_scale[0],
                    largest_widget_y_scale + self.internal_buffers[1] * 2.0,
                ];
            },
        }
        self.widget_scale = widget_scale;
    }

    fn resize_widgets(&mut self) {
        // widget scale
        self.calculate_widget_scale();
        let mut widget_ndc = self.get_widget_starting_ndc();


        match &self.panel_type {
            PanelType::Vertical(vertical_alignment) => {
                for widget in &mut self.section_widgets {
                    if let Some(widget) = widget {
                        let widget_scale = widget.get_prefered_scale();

                        let widget_pos = [
                            widget_ndc[0],
                            widget_ndc[1],
                            widget_ndc[0] + widget_scale[0],
                            widget_ndc[1] + widget_scale[1],
                        ];

                        widget.set_pos(widget_pos);

                        println!("Pos: {:?}", widget_pos);

                        widget_ndc[1] += widget_scale[1]; // Add to y axis for Vertical
                    }
                }
            },
            
            PanelType::Horizontal(_horizontal_alignment) => {
                for widget in &mut self.section_widgets {
                    if let Some(widget) = widget {
                        let widget_scale = widget.get_prefered_scale();

                        let widget_pos = [
                            widget_ndc[0],
                            widget_ndc[1],
                            widget_ndc[0] + widget_scale[0],
                            widget_ndc[1] + widget_scale[1],
                        ];

                        widget.set_pos(widget_pos);

                        widget_ndc[0] += widget_scale[0]; // Add to x axis for Horizontal
                    }
                }     
            },
        }

    }

    //=====================================
    // Widget Additions
    //=====================================
    // Tools for adding widgets to the panel
    // at specific panel cords

    pub fn add_sub_panel(&mut self, panel_type: PanelType) -> &mut Panel {
        let panel = Self::new(panel_type, self.buffered_pos, self.internal_buffers);
        self.section_widgets.push(Some(WidgetType::Panel(panel)));
        //self.sections.push(PanelSection::new());

        self.resize_widgets();

        if let Some(WidgetType::Panel(panel)) = self.section_widgets.last_mut().unwrap() {
            return panel;
        }
        else {
            panic!("Sub Panel was just inserted but could not be retrieved in Panel");
        }

    }

    pub fn add_button(&mut self, event: Event) -> &mut Button {
        let button = Button::new(event, self.internal_buffers);
        self.section_widgets.push(Some(WidgetType::Button(button)));
        
        //self.sections.push(PanelSection::new(WidgetType::Button(button)));
        self.resize_widgets();

        // Unwrap Option, then unpack the enum variant
        if let Some(WidgetType::Button(button)) = self.section_widgets.last_mut().unwrap() {
            return button;
        }
        else {
            panic!("Button was just inserted but could not be retrieved in Panel");
        }
    }


    //=====================================
    // Rendering 
    //=====================================
    // rendering of the panel texture using  
    // the propper tile textures

    fn set_internal_buffers(&mut self, buffer: [f32; 4]) {
        self.internal_buffers = buffer;
    }

    fn render_panel(&self, texture_manager: &mut TextureManager) {
        if self.color != PanelColor::Clear {
            let corners = self.tile_corner_pos;
            let sides = self.tile_side_pos;
            let center = self.tile_center_pos;

            let corner_textures = self.color.get_panel_corner_textures();
            let side_textures = self.color.get_panel_side_textures();
            let center_texture = self.color.get_panel_center_texture();

            // Corners
            texture_manager.render_ui_element_with_pos(corner_textures[0], corners[0]);
            texture_manager.render_ui_element_with_pos(corner_textures[1], corners[1]);
            texture_manager.render_ui_element_with_pos(corner_textures[2], corners[2]);
            texture_manager.render_ui_element_with_pos(corner_textures[3], corners[3]);

            // Sides
            texture_manager.render_ui_element_with_pos(side_textures[0], sides[0]);
            texture_manager.render_ui_element_with_pos(side_textures[1], sides[1]);
            texture_manager.render_ui_element_with_pos(side_textures[2], sides[2]);
            texture_manager.render_ui_element_with_pos(side_textures[3], sides[3]);

            // Center
            texture_manager.render_ui_element_with_pos(center_texture, center);
        }
    }



}


//=====================================
// Widget trait
//=====================================
impl Widget for Panel {
    // Getters
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }
    fn get_scale(&self) -> [f32; 2] {
        return self.buffered_scale;
    }

    fn set_pos(&mut self, pos: [f32; 4]) {
        self.pos = pos;
        self.resize();
    }

    fn has_prefered_scale(&self) -> bool {
        return true;
    }

    fn get_prefered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;
    }

    fn render(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData, game_event_manager: &mut GameEventManager) {
        // Render the panel
        self.render_panel(texture_manager);

        // Render all the widgets
        for widget in &self.section_widgets {
            if let Some(widget) = widget {
                widget.render(texture_manager, screen_data, game_event_manager);
            }
        }
    }
}