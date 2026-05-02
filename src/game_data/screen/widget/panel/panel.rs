
use image::{error, flat};

use crate::game_data::{TextureManager, game_event_manager::{game_event_manager::EventManager, prelude::Event}, player_data::player_data::PlayerData, screen::{ScreenData, screen_data, widget::{bar_button::bar_button::BarButtonWidget, button::button::Button, panel::{panel_background::{BackgroundType, PanelBackground}, panel_color::PanelColor, panel_section::PanelSection, panel_texture_manager::PanelTextureManager}, scroll_panel::scroll_panel::ScrollPanel, text::header::TextDisplay, toggle_button::toggle_button::ToggleButton, widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties}}};


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
    widget_properties: WidgetProperties,
    panel_texture: PanelTextureManager,

    // Sections
    sections: Vec<PanelSection>,
    current_id: u32,

    // Appearance
    orientation: PanelOrientation,
    alignment: PanelAlignment,
    new_background: Option<PanelBackground>,

    // Control
    events: Vec<Event>,
    mouse_on: bool,
}

impl Panel {

    //=====================================
    // Init
    //=====================================

    pub fn new(parent_pos: [f32; 4], buffers: [f32; 4]) -> Panel {
        let mut wp = WidgetProperties::new_blank();
        wp.parent_pos = parent_pos;
        wp.external_buffers = buffers;
        wp.internal_buffers = [widget_calculations::get_panel_spacing_scale(); 4];
        wp.prefered_scale = [widget_calculations::get_button_scale(); 2];

        Panel {
            widget_properties: wp,

            panel_texture: PanelTextureManager::new(),
            sections: Vec::new(),
            current_id: 0,

            orientation: PanelOrientation::Horizontal,
            alignment: PanelAlignment::Center,
            new_background: None,

            events: Vec::new(),
            mouse_on: false,
        }
    }

    pub fn new_blank() -> Panel {
        return Panel::new([0.0; 4], [0.0; 4])
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::Panel(self)
    }

    //=====================================
    // Appearance
    //=====================================

    pub fn set_orientation(&mut self, orientaiton: PanelOrientation, alignment: PanelAlignment) {
        self.orientation = orientaiton;
        self.alignment = alignment;

        for section in &mut self.sections {
            section.set_orientation(self.orientation, self.alignment);
        }
    }

    pub fn set_color(&mut self, color: PanelColor) {
        self.panel_texture.set_color(color);
    }

    pub fn set_new_background(&mut self, background_type: BackgroundType) {
        self.new_background = Some(PanelBackground::new(background_type))
    }

    //=====================================
    // Sizing
    //=====================================

    pub fn size(&mut self) {
        self.widget_properties.scale_based_off_parent();

        let pos   = self.widget_properties.pos;
        let scale = self.widget_properties.scale;
        let internal_buffers = self.widget_properties.internal_buffers;

        let indexing_mods = self.orientation.get_index_mods();
        let [stretch, cross, _stretch_end, cross_end] = indexing_mods;

        let mut stretch_space_used = 0.0;
        for section in &mut self.sections {
            let used = section.size(pos, stretch_space_used, internal_buffers);
            stretch_space_used += used;
        }

        let mut cross_prefered_scale: f32 = 0.0;
        let mut stretch_prefered_scale: f32 = 0.0;

        for section in &mut self.sections {
            let widget_prefered = section.get_mut_widget().get_preffered_scale();
            stretch_prefered_scale += section.get_section_scale()[stretch];
            stretch_prefered_scale += self.widget_properties.internal_buffers[1];

            let widget_cross = widget_prefered[cross]
                + internal_buffers[cross]
                + internal_buffers[cross_end];

            cross_prefered_scale = cross_prefered_scale.max(widget_cross);
        }

        self.widget_properties.prefered_scale[stretch] = stretch_prefered_scale;
        self.widget_properties.prefered_scale[cross] = cross_prefered_scale;

        let available_cross = scale[cross];

        for section in &mut self.sections {
            let widget_prefered = section.get_mut_widget().get_preffered_scale();
            let needed_cross = widget_prefered[cross]
                + internal_buffers[cross]
                + internal_buffers[cross_end];

            let extra = (available_cross - needed_cross).max(0.0);

            let mut buffers = internal_buffers;
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

        self.panel_texture.size(pos, scale);
    }

    pub fn add_widget(&mut self, mut widget: WidgetType) -> u32 {
        let id = self.current_id;
        widget.set_parent_pos(self.widget_properties.pos);
        widget.size();

        let section = PanelSection::new(widget, self.orientation, self.alignment, id);
        self.current_id += 1;

        self.sections.push(section);
        id
        
    }

    //=====================================
    // Panel Constructors
    //=====================================

    pub fn add_sub_panel(&mut self) -> &mut Panel {
        let panel = Self::new(self.widget_properties.pos, [0.0; 4]);
        self.add_widget(WidgetType::Panel(panel));

        if let WidgetType::Panel(panel) = self.sections.last_mut().unwrap().get_mut_widget() {
            return panel;
        } else {
            panic!("Sub Panel was just inserted but could not be retrieved in Panel");
        }
    }

    pub fn add_scroll_panel(&mut self) -> &mut ScrollPanel {
        let scroll_panel = ScrollPanel::new();
        self.add_widget(WidgetType::ScrollPanel(scroll_panel));

        if let WidgetType::ScrollPanel(scroll_panel) = self.sections.last_mut().unwrap().get_mut_widget() {
            return scroll_panel;
        } else {
            panic!("Scroll Panel was just inserted but could not be retrieved in Panel");
        }
    }

    //=====================================
    // Button Constructors
    //=====================================

    pub fn add_button(&mut self) -> &mut Button {
        let button = Button::new();
        self.add_widget(WidgetType::Button(button));

        if let WidgetType::Button(button) = self.sections.last_mut().unwrap().get_mut_widget() {
            return button;
        } else {
            panic!("Button was just inserted but could not be retrieved in Panel");
        }
    }

    pub fn add_bar_button(&mut self, text: String) -> &mut BarButtonWidget {
        let bar_button = BarButtonWidget::new(text, [0.0; 4]);
        self.add_widget(WidgetType::BarButton(bar_button));

        if let WidgetType::BarButton(bar_button) = self.sections.last_mut().unwrap().get_mut_widget() {
            return bar_button;
        } else {
            panic!("BarButton was just inserted but could not be retrieved in Panel");
        }
    }

    pub fn add_toggle_button(&mut self) -> &mut ToggleButton {
        let toggle_button = ToggleButton::new();
        self.add_widget(WidgetType::ToggleButton(toggle_button));

        if let WidgetType::ToggleButton(toggle_button) = self.sections.last_mut().unwrap().get_mut_widget() {
            return toggle_button;
        } else {
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
        } else {
            panic!("Header was just inserted but could not be retrieved in Panel");
        }
    }

    //=====================================
    // Input
    //=====================================

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn add_events(&mut self, events: &mut Vec<Event>) {
        self.events.append(events);
    }

    pub fn is_mouse_on(&self) -> bool {
        self.mouse_on
    }

    fn handle_inputs(&mut self, screen_data: &ScreenData, game_event_manager: &mut EventManager) {
        if screen_data.mouse_on_ndc_pos(self.widget_properties.pos) {
            game_event_manager.add_events(&self.events);
            self.mouse_on = true;
        } else {
            self.mouse_on = false;
        }
    }

    //=====================================
    // Sub Widget Getters
    //=====================================

    pub fn get_mut_sub_widget_mouse_on(&mut self, screen_data: &ScreenData) -> Option<&mut WidgetType> {
        for section in &mut self.sections {
            let widget_type = section.get_mut_widget();
            if widget_type.mouse_on(screen_data) {
                return Some(widget_type);
            }
        }
        return None;
    }

    pub fn get_sub_widget_mouse_on(&self, screen_data: &ScreenData) -> Option<&WidgetType> {
        for section in &self.sections {
            let widget_type = section.get_widget();
            if widget_type.mouse_on(screen_data) {
                return Some(widget_type);
            }
        }
        return None;
    }

    pub fn get_mut_sub_widgets(&mut self) -> Vec<&mut WidgetType> {
        let mut widgets = Vec::new();
        for section in &mut self.sections {
            widgets.push(section.get_mut_widget());
        }
        widgets
    }

    pub fn get_mut_widget_with_id(&mut self, id: u32) -> Option<&mut WidgetType> {
        if let Some(section) = self.sections.get_mut(id as usize) {
            Some(section.get_mut_widget())
        }
        else {
            None
        }
    }
}


impl Widget for Panel {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.widget_properties.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
    }

    fn size(&mut self) {
        self.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager,
        player_data: &PlayerData,
    ) {
        let bounds = self.widget_properties.bounds;

        if let Some(background) = &mut self.new_background {
            background.render_background(texture_manager, self.widget_properties.pos, bounds);
        }

        self.panel_texture.render(texture_manager, bounds);

        for section in &mut self.sections {
            let widget = section.get_mut_widget();
            widget.get_mut_widget_properties().bounds = bounds;
            widget.render(texture_manager, screen_data, game_event_manager, player_data);
        }
        self.handle_inputs(screen_data, game_event_manager);
    }
}
