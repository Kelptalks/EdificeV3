use miniquad::MouseButton;
use rand::rand_core::block;

use crate::game_data::{TextureManager, game_event_manager::{game_event_manager::GameEventManager, world_event_manager::world_event_manager::WorldEvent}, screen::{Button, ScreenData, play_view::{gui::location_manager_gui::LocationManagerGUI, play_view_data::{self, PlayViewData}}, render_centered_string_at_ndc, ui_elements::{block_selection::BlockSelection, panel::Panel}}, types::{BlockTexture, FontType, UITextures}};


enum BuildMode {
    SingleBlock,
    SelectionMode,
}

pub struct BuildingGUIManager {
    panel: Panel,
    panal_padding_ndc_scale: f32,
    gui_scale: [f32; 2],
    ndc_pos: [f32; 4],

    // Block selection
    button_block_select: Button,
    block_selection: BlockSelection,
    block_selection_visible: bool,

    button_toggle_build_mode: Button,
    create_location: Button,
}

impl BuildingGUIManager {
    pub fn new() -> BuildingGUIManager {
        BuildingGUIManager {
            panel: Panel::new_blank(),
            panal_padding_ndc_scale: 0.025,
            gui_scale: [0.0, 0.0],
            ndc_pos: [0.0, 0.0, 0.0, 0.0],

            // Block selection
            button_block_select: Button::new_blank(UITextures::ButtonCircle),
            block_selection: BlockSelection::new(),
            block_selection_visible: false,

            // Build mode
            button_toggle_build_mode: Button::new_blank(UITextures::ButtonCircle),
            create_location: Button::new_blank(UITextures::ButtonCircle),
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

    pub fn get_buttons_mut(&mut self) -> [&mut Button; 3] {
        [
            &mut self.button_block_select,
            &mut self.button_toggle_build_mode,
            &mut self.create_location,
            
        ]
    }


    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData, play_view_data: &PlayViewData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        

        // Panel
        let panel_padding_scale = play_view_data.get_panel_padding_scale();

        self.panel.scale_to_fill_screen_left(screen_data, panel_padding_scale, 0.10);
        self.panel.set_title("Building".to_string());
        self.panel.set_tile_ndc_scale(play_view_data.get_panel_tile_scale());

        let panel_center = self.panel.get_panel_ndc_center();
        let panel_start_cords = self.panel.get_text_ending_ndc();
        let panel_ndc_scale = self.panel.get_ndc_scale();

        // Buttons
        let button_scale = panel_ndc_scale[0] * 0.6;
        let button_spacing = button_scale * 0.2;
        let button_step = button_scale + button_spacing;
        let mut current_button_ndc = [panel_center[0] -(button_scale / 2.0), panel_start_cords[1] + button_spacing];
        

        self.button_block_select.set_scale(button_scale);
        self.button_block_select.set_ndc(current_button_ndc);
        self.button_block_select.set_text("Block Type".to_string());
        self.button_block_select.set_block(crate::game_data::types::BlockTexture::Debug);
        current_button_ndc[1] += button_step;

        self.button_toggle_build_mode.set_scale(button_scale);
        self.button_toggle_build_mode.set_ndc(current_button_ndc);
        self.button_toggle_build_mode.set_text("Toggle Build Mode".to_string());
        self.button_toggle_build_mode.set_block(crate::game_data::types::BlockTexture::Grass);
        current_button_ndc[1] += button_step;
        
        self.create_location.set_scale(button_scale);
        self.create_location.set_ndc(current_button_ndc);
        self.create_location.set_text("Create Location".to_string());
        self.create_location.set_block(crate::game_data::types::BlockTexture::Selector);
        current_button_ndc[1] += button_step;

        // Block selection Menu
        let block_selection_scale = [panel_ndc_scale[0], panel_ndc_scale[1]];
        self.block_selection.get_mut_panel().set_tile_ndc_scale(play_view_data.get_panel_tile_scale());
        self.block_selection.set_scale(block_selection_scale);
        self.block_selection.set_ndc(panel_start_cords);

        // Set GUI values
        self.gui_scale = [
            (self.panal_padding_ndc_scale * 2.0) + panel_ndc_scale[0],
            (self.panal_padding_ndc_scale * 2.0) + panel_ndc_scale[1],
        ];

        self.ndc_pos = [
            screen_start_ndc[0], 
            screen_start_ndc[1],
            screen_start_ndc[0] + self.gui_scale[0],
            screen_start_ndc[1] + self.gui_scale[1],
        ]
        
    }

    pub fn render(&mut self, 
        screen_data: &ScreenData, 
        texture_manager: &mut TextureManager,
    ) {
        // Render background
        self.panel.render(texture_manager);

        

        // Render block selection
        if self.block_selection_visible {
            self.block_selection.render(texture_manager, screen_data);
        }
        else {
            // Render buttons
            let buttons = self.get_buttons_mut();
            for button in buttons {
                button.render_button(texture_manager, screen_data);
            }
        }
    }


    //=====================================
    // Controls
    //=====================================
    pub fn mouse_button_down_event(&mut self, event_manager: &mut GameEventManager, play_view_data: &mut PlayViewData, screen_data: &ScreenData, button: MouseButton) {
        
        // Block selection updates
        if self.block_selection_visible {
            self.block_selection_visible = false;
            play_view_data.set_block_selected(self.block_selection.get_block_of_mouse());
            self.button_block_select.set_block(self.block_selection.get_block_of_mouse());
            self.block_selection_visible = !self.block_selection_visible; 
        }
        
        // If on GUI
        if screen_data.mouse_on_ndc_pos(self.ndc_pos) {
            if MouseButton::Left == button {
                if self.button_block_select.is_mouse_on_button() {
                    self.block_selection_visible = !self.block_selection_visible; 
                }
            }
        }
        // If on Render
        else {
            if button == MouseButton::Left {
                event_manager.add_world_event(WorldEvent::ModBlock(play_view_data.get_world_cords(), BlockTexture::Air));
            }
            else if button == MouseButton::Right {
                event_manager.add_world_event(WorldEvent::ModBlock(play_view_data.get_world_cords(), play_view_data.get_block_selected()));
            }
        }

    }

}