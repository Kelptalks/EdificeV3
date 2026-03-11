use crate::game_data::{TextureManager, game_event_manager::game_event_manager::{Event, GameEventManager}, screen::{ScreenData, widget::{self, button::button::Button, panel::{panel_section::PanelSection, panel_texture_manager::PanelTextureManager}, widget::{Widget, WidgetType}, widget_calculations}}};


#[derive(Clone, Copy)]
pub enum PanelAlignment {
    TopLeft,
    BotRight,
    Center,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PanelOrientation {
    Vertical,
    Horizontal,
}

impl PanelOrientation {
    pub fn get_index_mods(&self) -> [usize; 4] {
        match self {
            PanelOrientation::Vertical => [1, 0, 3, 2],
            PanelOrientation::Horizontal => [0, 1, 2, 3],
        }
    }
}

pub struct Panel {
    // Parent rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Self Rendering
    needs_resizing: bool,
    external_buffers: [f32; 4],  
    internal_buffers: [f32; 4],

    pos: [f32; 4],
    scale: [f32; 2],

    rendering_manager: PanelTextureManager,
    

    // Sections
    sections: Vec<PanelSection>,
    orientation: PanelOrientation,
    alignment: PanelAlignment,


}

impl Panel {

    //=====================================
    // Init
    //=====================================

    pub fn new(parent_pos: [f32; 4], buffers: [f32; 4]) -> Panel {
        
        let panel = Panel {
            // Parent Rendering
            parent_pos: parent_pos,
            parent_scale: [0.0; 2],
            prefered_scale: [0.2; 2],

            // Self Rendering
            needs_resizing: true,
            external_buffers: buffers, 
            internal_buffers: [0.01; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],
            


            rendering_manager: PanelTextureManager::new(),
            
            sections: Vec::new(),
            orientation: PanelOrientation::Horizontal,
            alignment: PanelAlignment::Center,

        };

        return panel;
    }

    //=====================================
    // Positioning
    //=====================================

    pub fn set_orientation(&mut self, orientaiton: PanelOrientation, alignment: PanelAlignment) {
        self.orientation = orientaiton;
        self.alignment = alignment;

        for section in &mut self.sections {
            section.set_orientation(self.orientation, self.alignment);
        }
    }

    //=====================================
    // Sizing
    //=====================================

    pub fn size(&mut self) {
        // Parent Values
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);        

        // Orientation
        let indexing_mods = self.orientation.get_index_mods();

        // Scale Sections
        let mut stretch_space_used_scale = 0.0;
        for section in &mut self.sections {
            section.size(self.pos, stretch_space_used_scale, self.internal_buffers);

            let section_scale = section.get_section_scale();
            stretch_space_used_scale += section_scale[indexing_mods[0]];
        }

        // Calculate prefered scale
        let mut cross_prefered_scale = 0.0;
        let mut stretch_prefered_scale = 0.0;

        for section in &mut self.sections {
            let section_scale = section.get_section_scale();
            
            stretch_prefered_scale += section_scale[indexing_mods[0]];
            let widget_cross_scale = section_scale[indexing_mods[1]];
            if widget_cross_scale > cross_prefered_scale {
                cross_prefered_scale = widget_cross_scale;
            }
        }

        if self.orientation == PanelOrientation::Vertical {
            println!("stretch_prefered_scale: {}", stretch_prefered_scale);
            println!("cross_prefered_scale: {}", cross_prefered_scale);
        }

        self.prefered_scale[indexing_mods[0]] = stretch_prefered_scale;
        self.prefered_scale[indexing_mods[1]] = cross_prefered_scale;
        
        // Resize Texture
        self.rendering_manager.size(self.pos, self.scale);
        self.needs_resizing = false;
    }

    //=====================================
    // Constructors
    //=====================================

    pub fn add_section(&mut self, widget: WidgetType) {
        let section = PanelSection::new(widget, self.orientation, self.alignment);
        self.sections.push(section);
    }

    pub fn add_sub_panel(&mut self) -> &mut Panel {
        let panel = Self::new(self.pos, [0.0; 4]);
        self.add_section(WidgetType::Panel(panel));

        println!("added_panel");

        if let WidgetType::Panel(panel) = self.sections.last_mut().unwrap().get_mut_widget() {
            return panel;
        }
        else {
            panic!("Sub Panel was just inserted but could not be retrieved in Panel");
        }
    }

    pub fn add_button(&mut self, event: Event) -> &mut Button { 
        let button = Button::new(event, [0.0; 4]);
        self.add_section(WidgetType::Button(button));

        println!("added_button");

        if let WidgetType::Button(button) = self.sections.last_mut().unwrap().get_mut_widget() {
            return button;
        }
        else {
            panic!("Sub Panel was just inserted but could not be retrieved in Panel");
        }
    }


}


impl Widget for Panel {
    // Getters
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }
    fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    fn get_prefered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
    }

    fn size(&mut self) {
        self.size();
    }

    fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData, game_event_manager: &mut GameEventManager) {
        if self.needs_resizing {
            self.size();
        }
        
        // Render the panel
        self.rendering_manager.render(texture_manager);
        
        // testing

        // Render all the widgets
        for section in &mut self.sections {
            // section._test_render(texture_manager);
            section.get_mut_widget().render(texture_manager, screen_data, game_event_manager);
            
        }
    }
}