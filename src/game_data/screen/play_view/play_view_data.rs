use miniquad::{KeyCode, MouseButton};

use crate::game_data::{TextureManager, debuging::debug_data::DebugData, locations::world_area::WorldArea, screen::{Button, ScreenData, input_data::Input, ui_elements::{block_selection::BlockSelection, selection_menu}}, types::BlockTexture};


/*
###############
## Direction ##
###############
Enum for controlling the cameras direction
main use is to get direction modify values
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum ViewDirection {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl ViewDirection {
    pub fn rotation_matrix(&self) -> [[i32; 2]; 2] {
        match self {
            ViewDirection::North => [[ 1,  0], [ 0,  1]],
            ViewDirection::East  => [[ 0,  1], [-1,  0]],
            ViewDirection::South => [[-1,  0], [ 0, -1]],
            ViewDirection::West  => [[ 0, -1], [ 1,  0]],
        }
    }

    pub fn from_id(id: u8) -> Self {
        match id {
            0 => ViewDirection::North,
            1 => ViewDirection::South,
            2 => ViewDirection::East,
            3 => ViewDirection::West,
            _ => ViewDirection::North,
        }
    }

    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn to_string(&self) -> String {
        match self {
            ViewDirection::North => "North".to_string(),
            ViewDirection::South => "South".to_string(),
            ViewDirection::East => "East".to_string(),
            ViewDirection::West => "West".to_string(),
        }
    }
}


/*
##########
## Mode ##
##########

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayMode {
    BuildManager, 
    DroneManager,
    LocationManager,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMenuType {
    None,
    BlockSelection
}

/*
##################
## PlayViewData ##
##################
Manages information about the play view that needs to be shared
between UI elements, controls, and the renderer. 
*/
pub struct PlayViewData {
    // Controls
    play_mode: PlayMode,
    block_selected: BlockTexture,

    // UI rendering
    panel_padding_scale: f32,
    panel_tile_scale: f32,

    // Camera
    world_cords: [i32; 3],
    view_direction: ViewDirection,

    // World
    area_selected: WorldArea,

    // Selection Menus
    block_selection_menu: BlockSelection,
    button_selected: Option<Button>,
}

impl PlayViewData {
    pub fn new() -> PlayViewData {
        PlayViewData {
            // Controls
            play_mode: PlayMode::BuildManager,
            block_selected: BlockTexture::Air,

            // UI rendering
            panel_padding_scale: 0.025,
            panel_tile_scale: 0.01,

            // Camera
            world_cords: [0, 0, 0],
            view_direction: ViewDirection::North,

            // World
            area_selected: WorldArea::new_blank(),

            // Selection menus
            block_selection_menu: BlockSelection::new(),

            button_selected: None,
        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn window_resize_update(&mut self, screen_data: &ScreenData) {
        let screen_end_ndc = screen_data.get_viewport_ending_ndc();
        let screen_start_ndc = screen_data.get_viewport_starting_ndc();

        // Block selection Menu
        let block_selection_scale = [
            0.3,
            (screen_end_ndc[1] - screen_start_ndc[1]) - (self.panel_padding_scale * 2.0),
        ];
        self.block_selection_menu.set_scale(block_selection_scale);

        let block_selection_ndc = [
            screen_end_ndc[0] - (block_selection_scale[0] + self.panel_padding_scale),
            screen_start_ndc[1] + self.panel_padding_scale,
        ];
        self.block_selection_menu.set_ndc(block_selection_ndc);
    }

    pub fn render(&mut self,
        screen_data: &ScreenData,
        texture_manager: &mut TextureManager,
    ) {
        self.block_selection_menu.render(texture_manager, screen_data);

        if let Some(button) = &mut self.button_selected {
            // Center button on mouse
            let mouse_cords = screen_data.get_mouse_ndc();
            let button_half_scale = button.get_scale() / 2.0;
            let button_cords = [
                mouse_cords[0] - button_half_scale,
                mouse_cords[1] - button_half_scale
            ];
            button.set_ndc(button_cords);
            button.render_button(texture_manager, screen_data);
        }

        // Handle inputs
        let inputs = screen_data.get_inputs();
        for input in inputs {
            match input {
                Input::MouseButtonDown(mouse_button) => {
                    if *mouse_button == MouseButton::Left{
                        if self.block_selection_menu.is_visible() {
                            self.button_selected = self.block_selection_menu.get_selected_button_clone();
                        }
                    }
                },
                Input::MouseButtonUp(mouse_button) => {
                    if *mouse_button == MouseButton::Left{
                        self.button_selected = None;
                    }
                },
                Input::KeyDown(key_code) => {
                    if *key_code == KeyCode::B {
                        self.open_selection_menu(SelectionMenuType::BlockSelection);
                    }
                }
                _ => {
                    
                }
            }
        }
    }

    //=====================================
    // Selection Menus
    //=====================================

    pub fn open_selection_menu(&mut self, selection_menu: SelectionMenuType) {
        match selection_menu {
            SelectionMenuType::None => {
                self.block_selection_menu.set_visible(false);
            },
            SelectionMenuType::BlockSelection => {
                self.block_selection_menu.set_visible(true);
            },
        }
    }

    pub fn get_button_selected(&mut self) -> &mut Option<Button> {
        return &mut self.button_selected;
    }

    pub fn get_button_selected_clone(&mut self) -> Option<Button> {
        return self.button_selected.clone();
    }

    //=====================================
    // Controls
    //=====================================

    pub fn get_play_mode(&self) -> PlayMode {
        return self.play_mode;
    }

    pub fn set_play_mode(&mut self, play_mode: PlayMode) {
        self.play_mode = play_mode;
    }

    pub fn get_block_selected(&self) -> BlockTexture{
        return self.block_selected;
    }

    pub fn set_block_selected(&mut self, new_block: BlockTexture) {
        self.block_selected = new_block;
    }

    //=====================================
    // UI rendering
    //=====================================

    pub fn get_panel_padding_scale(&self) -> f32 {
        return self.panel_padding_scale;
    }

    pub fn get_panel_tile_scale(&self) -> f32 {
        return self.panel_tile_scale;
    }

    //=====================================
    // Camera 
    //=====================================

    // Add / Subtract cords 
    pub fn get_world_cords(&self) -> [i32; 3] {
        return self.world_cords;
    }

    pub fn get_rotation_matrix(&self) -> [[i32; 2]; 2] {
        return self.view_direction.rotation_matrix();
    }

    pub fn mod_world_cords(&mut self, cord_mods: [i32; 3]) {
        let rot = self.view_direction.rotation_matrix();
        self.world_cords[0] += rot[0][0] * cord_mods[0] + rot[0][1] * cord_mods[1];
        self.world_cords[1] += rot[1][0] * cord_mods[0] + rot[1][1] * cord_mods[1];
        self.world_cords[2] += cord_mods[2];
    }

    pub fn set_world_cords(&mut self, cords: [i32; 3]) {
        self.world_cords = cords;
    }

    // Camera Direction
    pub fn rotate_left(&mut self) {
        let new_direction_id = self.view_direction.id() + 1;
        if new_direction_id < 4 {
            self.view_direction = ViewDirection::from_id(new_direction_id);
        }
        else {
            self.view_direction = ViewDirection::from_id(0);
        }
    }
    pub fn rotate_right(&mut self) {
        let current_direction_id = self.view_direction.id();

        if current_direction_id > 0 {
            self.view_direction = ViewDirection::from_id(current_direction_id - 1);
        }
        else {
            self.view_direction = ViewDirection::from_id(3);
        }
    }

    //=====================================
    // World
    //=====================================


    pub fn add_area(&mut self, area: WorldArea) {
        self.area_selected = area;
    }

    pub fn get_area_selected(&self) -> WorldArea {
        return self.area_selected;
    }

    //=====================================
    // Debug
    //=====================================

    pub fn collect_debug_data(&self, debug_data: &mut DebugData) {
        debug_data.set_camera_cords(self.world_cords);
        debug_data.set_direction(self.view_direction.to_string());
    }
}