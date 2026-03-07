use crate::game_data::{TextureManager, screen::{ScreenData, widget::widget_trait::Widget}, texture_manager, types::UITextures};


#[derive(Clone)]
pub enum PanelColor {
    Light,
    Dark,
}

impl PanelColor {
    pub fn get_panel_corner_textures(&self) -> [UITextures; 4] {
        match self {
            PanelColor::Light => [
                UITextures::PanelTopLeftLight,   // top_left
                UITextures::PanelTopRightLight,  // top_right
                UITextures::PanelBotLeftLight,   // bot_left
                UITextures::PanelBotRightLight,  // bot_right
            ],
            PanelColor::Dark => [
                UITextures::PanelTopLeftDark,    // top_left
                UITextures::PanelTopRightDark,   // top_right
                UITextures::PanelBotLeftDark,    // bot_left
                UITextures::PanelBotRightDark,   // bot_right
            ],
        }
    }

    pub fn get_panel_side_textures(&self) -> [UITextures; 4] {
        match self {
            PanelColor::Light => [
                UITextures::PanelTopCenterLight,  // top
                UITextures::PanelBotCenterLight,  // bot
                UITextures::PanelMidLeftLight,    // left
                UITextures::PanelMidRightLight,   // right
            ],
            PanelColor::Dark => [
                UITextures::PanelTopCenterDark,   // top
                UITextures::PanelBotCenterDark,   // bot
                UITextures::PanelMidLeftDark,     // left
                UITextures::PanelMidRightDark,    // right
            ],
        }
    }

    pub fn get_panel_center_texture(&self) -> UITextures {
        match self {
            PanelColor::Light => UITextures::PanelMidCenterLight,
            PanelColor::Dark  => UITextures::PanelMidCenterDark,
        }
    }
}



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
    section_widgets: Vec<Option<Box<dyn Widget>>>,
    
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

    pub fn get_mut_section(&mut self, section: [u32; 2]) -> &mut Option<Box<dyn Widget>> {
        let index: usize = self.get_section_index(section);
        &mut self.section_widgets[index]  // panics if out of bounds
    }

    pub fn add_sub_panel(&mut self, section: [u32; 2]) {
        let parent_pos = self.get_section_ndc(section);

        let panel = Box::new(Self::new(parent_pos, self.internal_buffers));
        self.section_widgets.push(Some(panel));
    }




}


//=====================================
// Widget trait
//=====================================
impl Widget for Panel {
    //=====================================
    // Setters / Getters
    //=====================================

    // Getters
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }
    fn get_scale(&self) -> [f32; 2] {
        return self.ndc_scale;
    }

    fn set_internal_buffers(&mut self, buffer: [f32; 4]) {
        self.internal_buffers = buffer;
    }

    fn render(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
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


        // Render all the widgets
        for widget in &self.section_widgets {
            if let Some(_widget) = widget {
                _widget.render(texture_manager, screen_data);
            }
        }
    }
}