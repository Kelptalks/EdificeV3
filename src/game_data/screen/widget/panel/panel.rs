use crate::game_data::{TextureManager, game_event_manager::{self, game_event_manager::{Event, GameEventManager}}, screen::{ScreenData, widget::{button::button::Button, panel::panel_color::PanelColor, widget::{Widget, WidgetType}}}, texture_manager, types::UITextures};

/*
###########
## Panel ##
###########
Panels are the main widget and act as a container for orginizing the
layout, rendering, and input handling for components contained within

*/

pub struct Panel {
    // Panel
    external_buffers: [f32; 4],
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    ndc_scale: [f32; 2],
    

    // Panel Tile Rendering
    tile_ndc_scale: f32,
    tile_corner_pos: [[f32; 4]; 4],
    tile_side_pos: [[f32; 4]; 4],
    tile_center_pos: [f32; 4],

    // Aperence
    color: PanelColor,
    section_dimentions: [u32; 2],
    section_widgets: Vec<Option<WidgetType>>,
    
}

//=====================================
// Panel
//=====================================
impl Panel {

    //=====================================
    // Constructor
    //=====================================
    pub fn new(parent_pos: [f32; 4], side_buffers: [f32; 4]) -> Self {
        
        let pos = [
            parent_pos[0] + side_buffers[0],
            parent_pos[1] + side_buffers[1],
            parent_pos[2] - side_buffers[2],
            parent_pos[3] - side_buffers[3],
        ];

        let ndc_scale = [
            pos[2] - pos[0],
            pos[3] - pos[1],
        ];

        let mut panel = Panel {
            external_buffers: side_buffers,
            internal_buffers: [0.01; 4],
            pos: pos,
            ndc_scale: ndc_scale,

            // Tile rendering data
            tile_ndc_scale: 0.025,
            tile_corner_pos: [[0.0; 4]; 4],
            tile_side_pos: [[0.0; 4]; 4],
            tile_center_pos: [0.0; 4],

            // Aperence
            color: PanelColor::Light,

            // Sections 
            section_dimentions: [0, 0],
            section_widgets: Vec::new(),
        };

        panel.set_sections([1, 1]);
        panel.calculate_tile_pos();

        return panel;
    }

    fn calculate_tile_pos(&mut self) {
        let s = self.tile_ndc_scale;
        let [x1, y1] = [self.pos[0], self.pos[1]];
        let x2 = x1 + self.ndc_scale[0];
        let y2 = y1 + self.ndc_scale[1];
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
    // Sections
    //=====================================
    // The panel is split into even sections based off
    // Selection Dimentions. 

    fn get_section_ndc(&self, section: [u32; 2]) -> [f32; 4] {
        if section[0] >= self.section_dimentions[0] && section[1] >= self.section_dimentions[1] {
            panic!(
                "Cannot assign to section ({:?}), in panel of section dimentions ({:?})", 
                section, 
                self.section_dimentions
            );
        }
        
        
        let section_scale = [
            self.ndc_scale[0] / self.section_dimentions[0] as f32,
            self.ndc_scale[1] / self.section_dimentions[1] as f32,
        ];
        
        let section_ndc_offset = [
            self.pos[0] + (section[0] as f32 * section_scale[0]),
            self.pos[1] + (section[1] as f32 * section_scale[1]),
        ];

        let section_ndc = [
            section_ndc_offset[0],
            section_ndc_offset[1],
            section_ndc_offset[0] + section_scale[0],
            section_ndc_offset[1] + section_scale[1],

        ]; 
        return section_ndc;
    }
    
    fn get_section_buffered_ndc(&self, section: [u32; 2]) -> [f32; 4] {
        let section_ndc = self.get_section_ndc(section);

        let buffered_section_ndc = [
            section_ndc[0] + self.internal_buffers[0],
            section_ndc[1] + self.internal_buffers[1],
            section_ndc[2] - self.internal_buffers[2],
            section_ndc[3] - self.internal_buffers[3],
        ];

        return buffered_section_ndc;

    }

    fn get_section_index(&self, section: [u32; 2]) -> usize {
        let section_index = (section[0] + section[1]) as usize;
        if section_index < self.section_widgets.len() {
            return section_index;
        }
        else {
            panic!("Section Index out of bounds for section ({:?})", section);
        }
    }

    pub fn set_sections(&mut self, sections: [u32; 2]) {
        self.section_dimentions = sections;

        let total_sections = (sections[0] * sections[1]) as usize;

        // Clear and set section widget size
        self.section_widgets.clear();
        self.section_widgets.resize_with(total_sections, || None);


    }

    pub fn get_mut_section(&mut self, section: [u32; 2]) -> &mut Option<WidgetType> {
        let index = self.get_section_index(section);
        &mut self.section_widgets[index]
    }

    //=====================================
    // Widget Additions
    //=====================================
    // Tools for adding widgets to the panel
    // at specific panel cords

    pub fn add_sub_panel(&mut self, section: [u32; 2]) -> &mut Panel {
        let parent_pos = self.get_section_ndc(section);
        let section_index = self.get_section_index(section);

        let panel = Self::new(parent_pos, self.internal_buffers);
        self.section_widgets.insert(section_index, Some(WidgetType::Panel(panel)));

        if let Some(WidgetType::Panel(panel)) = self.section_widgets.get_mut(section_index).unwrap() {
            return panel;
        }
        else {
            panic!("Sub Panel was just inserted but could not be retrieved in Panel");
        }
    }

    pub fn add_button(&mut self, section: [u32; 2], event: Event) -> &mut Button {
        let parent_pos = self.get_section_buffered_ndc(section);
        let section_index = self.get_section_index(section);

        let button = Button::new(parent_pos, event);
        self.section_widgets.insert(section_index, Some(WidgetType::Button(button)));
        

        // Unwrap Option, then unpack the enum variant
        if let Some(WidgetType::Button(button)) = self.section_widgets.get_mut(section_index).unwrap() {
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
        return self.ndc_scale;
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