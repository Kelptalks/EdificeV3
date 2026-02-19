use crate::game_data::{TextureManager, screen::{Button, ScreenData, render_centered_string_at_ndc, ui_elements::panel::Panel}, types::{FontType, UITextures}};

pub struct BuildingGUIManager {
    panel: Panel,
    panal_padding_ndc_scale: f32,
    gui_scale: [f32; 2],
    ndc_pos: [f32; 4],

    button_block_select: Button,
}

impl BuildingGUIManager {
    pub fn new() -> BuildingGUIManager {
        BuildingGUIManager {
            panel: Panel::new_blank(),
            panal_padding_ndc_scale: 0.05,
            gui_scale: [0.0, 0.0],
            ndc_pos: [0.0, 0.0, 0.0, 0.0],

            button_block_select: Button::new_blank(UITextures::ButtonCircle)
            
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_gui_ndc_scale(&self) -> [f32; 2] {
        return self.gui_scale;
    }

    pub fn get_gui_pos(&self) -> [f32; 4] {
        return self.ndc_pos;
    }


    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        // Free Buttons
        

        // Panel
        let panel_x_scale = (screen_end_ndc[0] - screen_start_ndc[1]) / 3.0;
        let panel_y_scale = (screen_end_ndc[1] - screen_start_ndc[1]) * 0.9;

        let panel_x_ndc_cor = screen_start_ndc[0] + (self.panal_padding_ndc_scale);
        let panel_y_ndc_cor = screen_start_ndc[1] + (self.panal_padding_ndc_scale);

        self.panel.set_ndc([panel_x_ndc_cor, panel_y_ndc_cor]);
        self.panel.set_ndc_scale([panel_x_scale, panel_y_scale]);
        self.panel.set_tile_ndc_scale(0.025);
        

        // Buttons
        let button_scale = self.panel.get_tile_ndc_scale() * 3.0;
        let panel_center = self.panel.get_panel_ndc_center();
        let panel_start_cords = self.panel.get_ndc();
        
        self.button_block_select.set_scale(button_scale);
        self.button_block_select.set_ndc([panel_center[0] -(button_scale / 2.0), panel_start_cords[1] + (button_scale)]);
        self.button_block_select.set_text("Block Selection".to_string());
        self.button_block_select.set_block(crate::game_data::types::BlockType::selector);


        // Set GUI values
        self.gui_scale = [
            (self.panal_padding_ndc_scale * 2.0) + panel_x_scale,
            (self.panal_padding_ndc_scale * 2.0) + panel_y_scale,
        ];

        self.ndc_pos = [
            screen_start_ndc[0], 
            screen_start_ndc[1],
            screen_start_ndc[0] + self.gui_scale[0],
            screen_start_ndc[1] + self.gui_scale[1],
        ]
        
    }

    pub fn render_view(&mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager,
    ) {
        // Render background
        self.panel.render(texture_manager);
        
        // Render header text
        let text_scale = self.panel.get_tile_ndc_scale();
        let panel_center = self.panel.get_panel_ndc_center();
        let panel_start_cords = self.panel.get_ndc();
        render_centered_string_at_ndc(
            texture_manager, 
            "Build Menu".to_string(), 
            FontType::Basic, 
            text_scale, 
            [panel_center[0], panel_start_cords[1] + text_scale]
        );

        // Render buttons
        self.button_block_select.render_button(texture_manager);
    }
}