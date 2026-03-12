use std::{cell::RefCell, rc::Rc};

use rand::rand_core::block;

use crate::game_data::{TextureManager, game_event_manager::{self, game_event_manager::{Event, GameEventManager}, widget_event_manager::widget_event_manager::WidgetEvent}, screen::{ScreenData, render_centered_string_at_ndc, screen_data, screen_mananager, widget::{self, button::button::Button, widget::Widget, widget_calculations}}, types::{BlockTexture, FontType, UITextures}};

pub struct ToggleButton {
    // Input handling
    is_toggled: Rc<RefCell<bool>>, 

    // Rendering
    button: Button,
    links: Vec<Event>

}

impl ToggleButton {
    pub fn new() -> ToggleButton {

        let is_toggled = Rc::new(RefCell::new(false));

        let mut button = Button::new([0.0; 4]);
        button.add_event(Event::WidgetEvent(WidgetEvent::ToggleBoolEvent(is_toggled.clone())));


        let mut toggle_button = ToggleButton {
            // Input handling
            is_toggled: is_toggled.clone(), 
            
            // Parent Rendering
            button: button,
            links: Vec::new(),
        };

        toggle_button.size();

        return toggle_button;
    }

    pub fn set_text_scale(&mut self, size: widget_calculations::TextSize) {
        self.button.set_text_scale(size);
    }

    //=====================================
    // Values
    //=====================================
    pub fn get_toggle_ref(&self) -> Rc<RefCell<bool>> {
        return self.is_toggled.clone();
    }
    pub fn set_toggle(&mut self, toggle: bool) {
        *self.is_toggled.borrow_mut() = toggle;
    }

    //=====================================
    // Events
    //=====================================

    pub fn add_value_link(&mut self, event: Event) {
        self.links.push(event);
    } 

    //=====================================
    // Apearence
    //=====================================
    pub fn set_block(&mut self, block_texture: BlockTexture) {
        self.button.set_block(block_texture);
    }

    pub fn set_icon(&mut self, icon: UITextures) {
        self.button.set_icon(icon);
    }

    pub fn set_text(&mut self, text: String) {
        self.button.set_text(text);
    }
    
}

impl Widget for ToggleButton {
    fn get_pos(&self) -> [f32; 4] {
        self.button.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.button.get_scale()
    }

    fn get_prefered_scale(&self) -> [f32; 2] {
        self.button.get_prefered_scale()
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.button.set_buffers(buffers);
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.button.set_parent_pos(pos);
    }
    
    fn size(&mut self) {
        self.button.size();
    }

    fn render(
        &mut self, 
        texture_manager: &mut TextureManager, 
        screen_data: &ScreenData, 
        game_event_manager: &mut GameEventManager
    ) { 
        // Add links
        for event in &self.links.pop() {
            game_event_manager.add_event(event.clone());
        }

        self.button.render(texture_manager, screen_data, game_event_manager);

        if *self.is_toggled.borrow() {
            texture_manager.render_ui_element_with_pos(UITextures::XIcon, self.button.get_pos());
        }

    }

}