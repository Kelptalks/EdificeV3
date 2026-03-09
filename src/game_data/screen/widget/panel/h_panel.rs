use crate::game_data::{TextureManager, game_event_manager::game_event_manager::{Event, GameEventManager}, screen::{ScreenData, widget::{self, button::button::Button, panel::{panel_section::PanelSection, panel_texture_manager::PanelTextureManager}, widget::{Widget, WidgetType}, widget_calculations}}};

pub enum HPanelAlignment {
    Top,
    Bot,
    Center,
}


pub struct VPanel {
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
    section_space_occupied: f64,
    sections: Vec<PanelSection>,
    section_alignment: HPanelAlignment,


}

impl VPanel {

    //=====================================
    // Init
    //=====================================

    pub fn new(parent_pos: [f32; 4], buffers: [f32; 4]) -> VPanel {
        
        let mut panel = VPanel {
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
            
            section_space_occupied: 0.0,
            sections: Vec::new(),
            section_alignment: HPanelAlignment::Top,

        };

        panel.size();

        return panel;
    }


    //=====================================
    // Sizing
    //=====================================

    pub fn size(&mut self) {
        // Parent Rendering
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);

        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);

        // println!("Panel Sizing :");
        // println!("  - Sizing : external buffers {:?}", self.external_buffers);
        // println!("  - Sized : pos({:?}), scale({:?})", self.pos, self.scale);

        self.rendering_manager.size(self.pos, self.scale);


        // Update section pos
        let mut y_prefered_scale = 0.0;
        let mut x_prefered_scale = 0.0;
        for section in &mut self.sections {
            section.set_parent_pos(self.pos);

            
            let section_scale = section.get_section_scale();
            
            x_prefered_scale += section_scale[0];
            let widget_y_scale = section_scale[1];
            if widget_y_scale > y_prefered_scale {
                y_prefered_scale = widget_y_scale;
            }
        }

        if x_prefered_scale > 0.08 {
            self.prefered_scale[0] = x_prefered_scale;
        }
        if y_prefered_scale > 0.08 {
            self.prefered_scale[1] = y_prefered_scale;
        }

        println!("Prefered_Scale: {:?}", self.prefered_scale);

        self.needs_resizing = false;
    }


    //=====================================
    // sections
    //=====================================

    pub fn add_section(&mut self, widget: WidgetType) {
        let mut section = PanelSection::new(widget);

        section.set_internal_buffers(self.internal_buffers);

        let x_scale = section.get_section_prefered_width();
        let space_requested = x_scale / self.scale[0];
        

        let section_x_scale = self.scale[0] * space_requested;

        let x_left_buffer = (self.scale[0] * self.section_space_occupied as f32);
        let x_right_buffer = (self.scale[0] - (x_left_buffer + section_x_scale));

        let section_buffers = [
            x_left_buffer + self.internal_buffers[0],
            self.internal_buffers[1],

            x_right_buffer + self.internal_buffers[2],
            self.internal_buffers[3],
        ];

        section.set_buffers(section_buffers);
        section.set_parent_pos(self.pos);

        self.sections.push(section);

        // Update space ocupied
        self.section_space_occupied += space_requested as f64;
        if self.section_space_occupied > 1.0 {
            todo!("need to resize all components to make space");
        }

        self.size();

    }


    //=====================================
    // Constructors
    //=====================================

    pub fn add_sub_panel(&mut self) -> &mut VPanel {
        let panel = Self::new(self.pos, [0.0; 4]);
        self.add_section(WidgetType::VPanel(panel));

        println!("added_panel");

        if let WidgetType::VPanel(panel) = self.sections.last_mut().unwrap().get_mut_widget() {
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


impl Widget for VPanel {
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
        self.size();
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
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
            section.test_render(texture_manager);
            section.get_mut_widget().render(texture_manager, screen_data, game_event_manager);
            
        }
    }
}