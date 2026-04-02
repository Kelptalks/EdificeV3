use std::{cell::RefCell, rc::Rc};


use crate::game_data::{TextureManager, game_event_manager::{event_manager::EventManager, game_event_manager::game_event_manager::GameEvent, prelude::BoolEvent, widget_event_manager::widget_event_manager::WidgetEvent}, screen::{ScreenData, widget::{button::button::Button, widget::Widget, widget_calculations}}, types::{BlockTexture, UITextures}};

pub struct ToggleButton {
    // Input handling
    is_toggled: Rc<RefCell<bool>>, 

    // Rendering
    button: Button,
    links: Vec<GameEvent>

}

impl ToggleButton {
    pub fn new() -> ToggleButton {

        let is_toggled = Rc::new(RefCell::new(false));

        let mut button = Button::new();
        button.add_left_click_event(WidgetEvent::ToggleBoolEvent(is_toggled.clone()).wrap_into_event());


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
    pub fn set_toggle_ref(&mut self, new_ref: &Rc<RefCell<bool>>) {
        self.is_toggled = new_ref.clone();
        
        self.button.clear_events();
        self.button.add_left_click_event(BoolEvent::ToggleBool(new_ref.clone()).wrap_into_event());
    }
    
    pub fn get_toggle_ref(&self) -> Rc<RefCell<bool>> {
        return self.is_toggled.clone();
    }
    pub fn set_toggle(&mut self, toggle: bool) {
        *self.is_toggled.borrow_mut() = toggle;
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

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.button.get_preffered_scale()
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
        game_event_manager: &mut EventManager
    ) { 
        // Add links
        for event in &self.links.pop() {
            game_event_manager.add_game_event(event.clone());
        }

        self.button.render(texture_manager, screen_data, game_event_manager);

        if *self.is_toggled.borrow() {
            texture_manager.render_ui_element_with_pos(UITextures::XIcon, self.button.get_pos());
        }

    }

}