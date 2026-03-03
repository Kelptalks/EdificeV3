use crate::game_data::{TextureManager, screen::{Button, ScreenData, screen_data, ui_elements::{panel::Panel, selection_menu::SelectionMenu}}, types::BlockTexture};

pub struct BlockSelection {
    selection_menu: SelectionMenu,
}


impl BlockSelection {
    pub fn new() -> BlockSelection {
        let mut selection_menu = SelectionMenu::new();

        // Add buttons for each block
        for i in 0..BlockTexture::get_total_blocks() {
            let block = BlockTexture::from_id(i as u16);
            
            // Create button and set up it's aperence.
            let mut button = Button::new_blank();
            button.set_text(block.to_string().to_string());
            button.set_block(block);
            
            selection_menu.add_button(button);
        }


        BlockSelection {
            selection_menu: selection_menu,
        }
    }

    //=====================================
    // Updates
    //=====================================


    //=====================================
    // Getters / Setters
    //=====================================

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.selection_menu.set_ndc(ndc);
    }

    pub fn set_scale(&mut self, scale: [f32; 2]) {
        self.selection_menu.set_scale(scale);
    }

    pub fn get_ndc(&self) -> [f32; 2] {
        return self.selection_menu.get_ndc();
    }

    pub fn get_scale(&self) -> [f32; 2] {
        return self.selection_menu.get_scale();
    }

    pub fn get_ndc_end_cords(&self) -> [f32; 2] {
        return self.selection_menu.get_ending_ndc();
    }

    pub fn get_selected_button_clone(&mut self) -> Option<Button> {
        return self.selection_menu.get_selected_button_clone();
    }

    pub fn is_visible(&self) -> bool {
        return self.selection_menu.is_visible();
    }

    pub fn set_visible(&mut self, visible: bool) {
        return self.selection_menu.set_visible(visible);
    }

    //=====================================
    // Renderer
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {

        self.selection_menu.render(texture_manager, screen_data);
    }

}

