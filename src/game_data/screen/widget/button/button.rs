use rand::rand_core::block;

use crate::game_data::{TextureManager, game_event_manager::{self, game_event_manager::{Event, GameEventManager}}, screen::{ScreenData, widget::{self, widget::Widget}}, types::{BlockTexture, UITextures}};

pub struct Button {
    // Rendering
    pos: [f32; 4],
    scale: [f32; 2],

    // Input
    event: Event, // The event that will occer when the button is pressed

    // Apearence
    button_type: UITextures,
    apearence_pos: [f32; 4],
    
    block_texture: Option<BlockTexture>,
    icon_type: Option<UITextures>,

}

impl Button {
    pub fn new(parent_pos: [f32; 4], event: Event) -> Button {
        let scale = [
            parent_pos[2] - parent_pos[0],
            parent_pos[3] - parent_pos[1],
        ];
        
        // Calculate aperence pos based of scale
        let apearence_buffer = [
            scale[0] * 0.8,
            scale[1] * 0.8,
        ];
        let apearence_pos = [
            parent_pos[0] + apearence_buffer[0],
            parent_pos[1] + apearence_buffer[1],
            parent_pos[2] - apearence_buffer[0],
            parent_pos[3] - apearence_buffer[1],
        ];

        let button = Button {
            // Rendering
            pos: parent_pos,
            scale: scale,

            // Input 
            event: event, 

            // Apearence
            button_type: UITextures::ButtonCircle, 
            apearence_pos: apearence_pos,

            block_texture: None, 
            icon_type: None,
        };

        return button;
    }


    //=====================================
    // Apearence
    //=====================================
    pub fn set_block(&mut self, block_texture: BlockTexture) {
        self.block_texture = Some(block_texture);
    }

    pub fn set_icon(&mut self, icon: UITextures) {
        self.icon_type = Some(icon);
    }

    fn render_apearence(&self, texture_manager: &mut TextureManager ) {
        

        // Render block
        if let Some(block_texture) = self.block_texture {
            texture_manager.render_block_with_pos(block_texture, self.apearence_pos);
        }
        
        // Render icon
        if let Some(icon) = self.icon_type {
            texture_manager.render_ui_element_with_pos(icon, self.apearence_pos);
        }
    }

    
}

impl Widget for Button {
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }

    fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    fn render(
        &self, 
        texture_manager: &mut TextureManager, 
        screen_data: &ScreenData, 
        game_event_manager: &mut GameEventManager
    ) {
        let mut button_texture = self.button_type;
        // If mouse is on button
        if screen_data.mouse_on_ndc_pos(self.pos) {
            button_texture = self.button_type.get_pressed_variant(); // Update texture

            // If button was clicked
            if screen_data.was_left_pressed(){
                game_event_manager.add_event(self.event.clone());    
            }
        }

        // Render button 
        texture_manager.render_ui_element_with_pos(button_texture, self.pos);

        // Render aperence values
        self.render_apearence(texture_manager);
    }
}