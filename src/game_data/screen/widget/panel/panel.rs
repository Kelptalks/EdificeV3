use std::{cell::RefCell, rc::Rc};

use crate::game_data::{TextureManager, player_data::drone_programming::var::var_type::{Var, VarTypeKind}, game_event_manager::{event_manager, game_event_manager::{EventManager, GameEvent}, prelude::Event}, screen::{ScreenData, text, widget::{bar_button::bar_button::BarButtonWidget, button::button::Button, drone_programming::vars::{var_slot::VarSlot}, panel::{panel_background::{BackgroundType, PanelBackground}, panel_color::PanelColor, panel_section::PanelSection, panel_texture_manager::PanelTextureManager}, scroll_panel::{self, scroll_panel::ScrollPanel}, tab_panel::tab_panel::TabPanel, text::header::TextDisplay, toggle_button::toggle_button::ToggleButton, widget::{Widget, WidgetType}, widget_calculations, world_rendering::{play_world_view_config::PlayViewRendingConfig, play_world_view_render::PlayWorldViewRender}}}, types::UITextures};


#[derive(Clone, Copy)]
pub enum PanelAlignment {
    TopLeft,
    BotRight,
    Center,
    Fill
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
    external_buffers: [f32; 4],  
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    scale: [f32; 2],

    rendering_manager: PanelTextureManager,
    
    // Sections
    sections: Vec<PanelSection>,

    // Aprearence
    orientation: PanelOrientation,
    alignment: PanelAlignment,
    new_background: Option<PanelBackground>,

    // Control
    events: Vec<Event>,
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
            external_buffers: buffers, 
            internal_buffers: [0.012; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],
            
            rendering_manager: PanelTextureManager::new(),
            
            // Sections
            sections: Vec::new(),
            
            // Aprearence
            orientation: PanelOrientation::Horizontal,
            alignment: PanelAlignment::Center,
            new_background: None,

            // Control
            events: Vec::new(),
        };

        return panel;
    }

    //=====================================
    // Apearence
    //=====================================

    pub fn set_orientation(&mut self, orientaiton: PanelOrientation, alignment: PanelAlignment) {
        self.orientation = orientaiton;
        self.alignment = alignment;

        for section in &mut self.sections {
            section.set_orientation(self.orientation, self.alignment);
        }
    }

    pub fn set_color(&mut self, color: PanelColor) {
        self.rendering_manager.set_color(color);
    }
    
    pub fn set_new_background(&mut self, background_type: BackgroundType) {
        self.new_background = Some(PanelBackground::new(background_type))
    }

    //=====================================
    // Sizing
    //=====================================

    pub fn size(&mut self) {
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);

        let indexing_mods = self.orientation.get_index_mods();
        let [stretch, cross, _stretch_end, cross_end] = indexing_mods;

        // First pass: size sections to get preferred scales
        let mut stretch_space_used = 0.0;
        for section in &mut self.sections {
            let used = section.size(self.pos, stretch_space_used, self.internal_buffers);
            stretch_space_used += used;
        }

        // Compute preferred scale from content
        let mut cross_prefered_scale: f32 = 0.0;
        let mut stretch_prefered_scale: f32 = 0.0;

        for section in &mut self.sections {
            let widget_prefered = section.get_mut_widget().get_preffered_scale();
            stretch_prefered_scale += section.get_section_scale()[stretch];

            let widget_cross = widget_prefered[cross]
                + self.internal_buffers[cross]
                + self.internal_buffers[cross_end];

            cross_prefered_scale = cross_prefered_scale.max(widget_cross);
        }

        self.prefered_scale[stretch] = stretch_prefered_scale;
        self.prefered_scale[cross] = cross_prefered_scale;

        // Second pass: apply cross-axis alignment per section
        let available_cross = self.scale[cross];

        for section in &mut self.sections {
            let widget_prefered = section.get_mut_widget().get_preffered_scale();
            let needed_cross = widget_prefered[cross]
                + self.internal_buffers[cross]
                + self.internal_buffers[cross_end];

            let extra = (available_cross - needed_cross).max(0.0);

            let mut buffers = self.internal_buffers;
            match self.alignment {
                PanelAlignment::TopLeft => {
                    buffers[cross_end] += extra;
                }
                PanelAlignment::BotRight => {
                    buffers[cross] += extra;
                }
                PanelAlignment::Center => {
                    buffers[cross] += extra / 2.0;
                    buffers[cross_end] += extra / 2.0;
                }
                PanelAlignment::Fill => {
                }
            }

            section.get_mut_widget().set_buffers(buffers);
            section.get_mut_widget().size();
        }

        self.rendering_manager.size(self.pos, self.scale);
    }

    pub fn add_widget(&mut self, widget: WidgetType) {
        let section = PanelSection::new(widget, self.orientation, self.alignment);
        self.sections.push(section);
    }


    //=====================================
    // Panel Constructors
    //=====================================

    pub fn add_sub_panel(&mut self) -> &mut Panel {
        let panel = Self::new(self.pos, [0.0; 4]);
        self.add_widget(WidgetType::Panel(panel));
 
        if let WidgetType::Panel(panel) = self.sections.last_mut().unwrap().get_mut_widget() {
            return panel;
        }
        else {
            panic!("Sub Panel was just inserted but could not be retrieved in Panel");
        }
    }

    pub fn add_scroll_panel(&mut self) -> &mut ScrollPanel {
        let scroll_panel = ScrollPanel::new();
        self.add_widget(WidgetType::ScrollPanel(scroll_panel));
 
        if let WidgetType::ScrollPanel(scroll_panel) = self.sections.last_mut().unwrap().get_mut_widget() {
            return scroll_panel;
        }
        else {
            panic!("Scroll
             Panel was just inserted but could not be retrieved in Panel");
        }
    }

    //=====================================
    // Button Constructors
    //=====================================

    pub fn add_button(&mut self) -> &mut Button { 
        let button = Button::new([0.0; 4]);
        self.add_widget(WidgetType::Button(button));

        if let WidgetType::Button(button) = self.sections.last_mut().unwrap().get_mut_widget() {
            return button;
        }
        else {
            panic!("Button was just inserted but could not be retrieved in Panel");
        }
    }

    pub fn add_bar_button(&mut self, text: String) -> &mut BarButtonWidget {
        let bar_button = BarButtonWidget::new(text, [0.0; 4]);
        self.add_widget(WidgetType::BarButton(bar_button));

        if let WidgetType::BarButton(bar_button) = self.sections.last_mut().unwrap().get_mut_widget() {
            return bar_button;
        }
        else {
            panic!("BarButton was just inserted but could not be retrieved in Panel");
        }
    }

    pub fn add_toggle_button(&mut self) -> &mut ToggleButton {
        let toggle_button = ToggleButton::new();
        self.add_widget(WidgetType::ToggleButton(toggle_button));

        if let WidgetType::ToggleButton(toggle_button) = self.sections.last_mut().unwrap().get_mut_widget() {
            return toggle_button;
        }
        else {
            panic!("BarButton was just inserted but could not be retrieved in Panel");
        }
    }

    //=====================================
    // Text Constructors
    //=====================================

    pub fn add_text_display(&mut self, text: String) -> &mut TextDisplay {
        let header = TextDisplay::new(text);
        self.add_widget(WidgetType::TextDisplay(header));

        if let WidgetType::TextDisplay(header) = self.sections.last_mut().unwrap().get_mut_widget() {
            return header;
        }
        else {
            panic!("Header was just inserted but could not be retrieved in Panel");
        }
    }

    //=====================================
    // Input
    //=====================================

    pub fn add_event (&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn add_events(&mut self, events: &mut Vec<Event>) {
        self.events.append(events);
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

    fn get_preffered_scale(&self) -> [f32; 2] {
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

    fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData, game_event_manager: &mut EventManager) {        
        if let Some(background) = &mut self.new_background {
            background.render_background(texture_manager, self.pos);
        } 
        
        // Render the panel
        self.rendering_manager.render(texture_manager);

        // Render all the widgets
        for section in &mut self.sections {
            section.get_mut_widget().render(texture_manager, screen_data, game_event_manager);
        }

        if screen_data.mouse_on_ndc_pos(self.pos) {
            game_event_manager.add_events(&self.events);
        }
    }
}