use miniquad::MouseButton;
use rand::rand_core::block;

use crate::game_data::{TextureManager, game_event_manager::{game_event_manager::GameEventManager, world_event_manager::world_event_manager::WorldEvent}, screen::{Button, ScreenData, play_view::{gui::area_selection_gui::AreaSelectionGUI, play_view_data::{self, PlayViewData}}, render_centered_string_at_ndc, ui_elements::{block_selection::BlockSelection, panel::Panel}}, types::{BlockTexture, FontType, UITextures}};


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
    button_select_area_mode: Button,

    // Area Selection
    area_selection_gui: AreaSelectionGUI,

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
            button_select_area_mode: Button::new_blank(UITextures::ButtonCircle),

            // Area Secltion
            area_selection_gui: AreaSelectionGUI::new(),
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
            &mut self.button_select_area_mode,
            
        ]
    }


    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        // Free Buttons
        

        // Panel
        self.panel.scale_to_fill_screen_left(screen_data, 0.025, 0.15);
        self.panel.set_title("Building".to_string());
        let panel_center = self.panel.get_panel_ndc_center();
        let panel_start_cords = self.panel.get_ndc();
        let panel_ndc_scale = self.panel.get_ndc_scale();


        

        // Buttons
        let button_scale = self.panel.get_tile_ndc_scale() * 3.0;
        let mut current_button_ndc = [panel_center[0] -(button_scale / 2.0), panel_start_cords[1] + (button_scale)];
        let button_spacing = button_scale + (button_scale * 0.5);

        self.button_block_select.set_scale(button_scale);
        self.button_block_select.set_ndc(current_button_ndc);
        self.button_block_select.set_text("Block Type".to_string());
        self.button_block_select.set_block(crate::game_data::types::BlockTexture::Debug);
        current_button_ndc[1] += button_spacing;


        self.button_toggle_build_mode.set_scale(button_scale);
        self.button_toggle_build_mode.set_ndc(current_button_ndc);
        self.button_toggle_build_mode.set_text("Toggle Build Mode".to_string());
        self.button_toggle_build_mode.set_block(crate::game_data::types::BlockTexture::Grass);
        current_button_ndc[1] += button_spacing;

        
        self.button_select_area_mode.set_scale(button_scale);
        self.button_select_area_mode.set_ndc(current_button_ndc);
        self.button_select_area_mode.set_text("Single Block Mode".to_string());
        self.button_select_area_mode.set_block(crate::game_data::types::BlockTexture::selector);
        current_button_ndc[1] += button_spacing;



        // Block selection Menu
        let block_selection_scale = [panel_ndc_scale[0], panel_ndc_scale[1]];
        self.block_selection.set_scale(block_selection_scale);
        self.block_selection.set_ndc(panel_start_cords);

        // Area selection GUI
        self.area_selection_gui.set_ndc([panel_center[0] - (button_scale), current_button_ndc[1]]);
        self.area_selection_gui.set_x_scale(button_scale * 2.0);

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

        // Render buttons
        let buttons = self.get_buttons_mut();
        for button in buttons {
            button.render_button(texture_manager, screen_data);
        }

        // Render block selection
        if self.block_selection_visible {
            self.block_selection.render(texture_manager, screen_data);
        }

        self.area_selection_gui.render_view(screen_data, texture_manager);
    }

    //=====================================
    // Controls
    //=====================================

    pub fn mouse_button_down_event(&mut self, event_manager: &mut GameEventManager, play_view_data: &mut PlayViewData, screen_data: &ScreenData, button: MouseButton) {
        // If on GUI
        if screen_data.mouse_on_ndc_pos(self.ndc_pos) {
            if MouseButton::Left == button {
                if self.button_block_select.is_mouse_on_button() {
                    self.block_selection_visible = !self.block_selection_visible; 
                }
                else if self.block_selection_visible {
                    self.block_selection_visible = false;
                    play_view_data.set_block_selected(self.block_selection.get_block_of_mouse());
                    self.button_block_select.set_block(self.block_selection.get_block_of_mouse());
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