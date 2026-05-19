#![allow(dead_code)]
use std::collections::HashMap;


use crate::game_data::screen::{ScreenData, widget::{prelude::PlayWorldViewRender, widget::Widget, widget_calculations, widget_properties::WidgetProperties, window_manager::{window::WidgetWindow, windows::{block_select_win::BlockSelectWindow, cheat_window::CheatWindow, debug_win::DebugWin, debug_world_win::DebugWorldWindow, settings_window::SettingsWindow, window_type::{Window, WindowType}}}, world_rendering::tile_map_manager::TileMapManager}};


#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct WidgetWindowId {
    id: u32,
}

impl WidgetWindowId {
    pub fn new(id: u32) -> WidgetWindowId { 
        WidgetWindowId { id }
    }
    
    pub fn as_u32(&self) -> u32 {
        self.id
    }
}


pub struct WidgetWindowManager {
    widget_props: WidgetProperties,

    focused_window: Option<WidgetWindowId>,
    open_windows: Vec<WidgetWindowId>,
    minimized_windows: Vec<WidgetWindowId>,

    windows: HashMap<WidgetWindowId, WidgetWindow>,
    
    debug: Option<WidgetWindowId>,
    debug_world: Option<WidgetWindowId>,
    block_select: Option<WidgetWindowId>,
    settings: Option<WidgetWindowId>,


    next_window_id: u32, 
    play_view: PlayWorldViewRender,
}

impl WidgetWindowManager {
    pub fn new(screen_data: &ScreenData) -> WidgetWindowManager {
        let mut widget_props = WidgetProperties::new_blank();
        
        // Force to take up entire screen
        widget_props.pos = screen_data.get_viewport_uv();
        widget_props.scale = widget_calculations::pos_to_scale(widget_props.pos);
        widget_props.parent_scale = widget_calculations::pos_to_scale(widget_props.pos);


        let mut play_view = PlayWorldViewRender::new(&widget_props);
        play_view.set_prefered_size(1.0);

        WidgetWindowManager {
            widget_props: widget_props,

            focused_window: None,
            open_windows: Vec::new(),
            minimized_windows: Vec::new(),
            
            windows: HashMap::new(),
            debug: None,
            debug_world: None,
            block_select: None,
            settings: None,

            next_window_id: 0,
            play_view: play_view,
        }
    }

    fn get_next_window_id(&mut self) -> WidgetWindowId {
        let new_id = WidgetWindowId::new(self.next_window_id);
        self.next_window_id+=1;
        new_id
    }

    pub fn get_tile_map_manager(&mut self) -> &mut TileMapManager {
        self.play_view.get_tile_map_manager()
    }

    pub fn new_window(&mut self, mut window: WindowType, name: &str) -> WidgetWindowId {
        let id = self.get_next_window_id();
        
        window.set_parent_pos(self.widget_props.pos);
        window.set_buffers([0.2; 4]);
        window.size();

        let window = WidgetWindow::new(window, &self.widget_props, name);
        self.windows.insert(id, window);
        self.open_windows.push(id);

        id
    }
}

impl Widget for WidgetWindowManager {
    fn get_widget_properties(&self) -> &crate::game_data::screen::widget::widget_properties::WidgetProperties {
        &self.widget_props
    }

    fn get_mut_widget_properties(&mut self) -> &mut crate::game_data::screen::widget::widget_properties::WidgetProperties {
        &mut self.widget_props
    }

    fn size(&mut self) {
        let parent_pos = self.get_pos();
        let screen_scale = self.get_scale();

        for (_id, window) in &mut self.windows {
            
            let prefered_scale = window.get_preffered_scale();
            
 
            let x_buffer = (screen_scale[0] - prefered_scale[0]) / 2.0;
            let y_buffer = (screen_scale[1] - prefered_scale[1]) / 2.0;

            let mut buffer = [
                x_buffer,
                y_buffer,
                x_buffer,
                y_buffer,
            ];

            let offset = window.get_offset();
            widget_calculations::offset_buffer(&mut buffer, offset);
 
            
            
            window.set_buffers(buffer);
            window.set_parent_pos(parent_pos);
            window.size();
        }

    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        self.size();

        // render play view / Background
        self.play_view.render(texture_manager, screen_data, game_event_manager, player_data);


        // Remove closed windows
        self.windows.retain(|_, window| !window.close());
        self.open_windows.retain(|id| self.windows.contains_key(id));

        // Render in z-order: last entry is on top
        let ids: Vec<WidgetWindowId> = self.open_windows.clone();
        let mut newly_focused: Option<WidgetWindowId> = None;

        for id in &ids {
            if let Some(window) = self.windows.get_mut(id) {
                if !window.minimized() {
                    window.render(texture_manager, screen_data, game_event_manager, player_data);
                    if window.was_clicked() {
                        newly_focused = Some(*id);
                    }
                }
            }
        }

        // Move the topmost clicked window to the front of the z-order
        if let Some(id) = newly_focused {
            self.open_windows.retain(|w| *w != id);
            self.open_windows.push(id);
            self.focused_window = Some(id);
        }

        // Menu 
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::F3){
            
            if let Some(debug) = self.debug {
                if let Some(window) = self.windows.get_mut(&debug) {
                    window.focus()
                }
                else {
                    let window = DebugWin::new().wrap_into_window_type();
                    self.debug = Some(self.new_window(window, "Debug"));
                }
            }
            else {
                let window = DebugWin::new().wrap_into_window_type();
                self.debug = Some(self.new_window(window, "Debug"));
            }

        }
        else if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::F2) {
            let window = CheatWindow::new().wrap_into_window_type();
            self.new_window(window, "Cheat Window");
        }
        else if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::F4) {
            if let Some(id) = self.debug_world {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.focus();
                } else {
                    let window = DebugWorldWindow::new().wrap_into_window_type();
                    self.debug_world = Some(self.new_window(window, "Debug World"));
                }
            } else {
                let window = DebugWorldWindow::new().wrap_into_window_type();
                self.debug_world = Some(self.new_window(window, "Debug World"));
            }
        }
        else if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::F5) {
            if let Some(id) = self.block_select {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.focus();
                } else {
                    let window = BlockSelectWindow::new().wrap_into_window_type();
                    self.block_select = Some(self.new_window(window, "Block Select"));
                }
            } else {
                let window = BlockSelectWindow::new().wrap_into_window_type();
                self.block_select = Some(self.new_window(window, "Block Select"));
            }
        }
        else if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::F12) {
            if let Some(id) = self.settings {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.focus();
                } else {
                    let window = SettingsWindow::new().wrap_into_window_type();
                    self.settings = Some(self.new_window(window, "Settings"));
                }
            } else {
                let window = SettingsWindow::new().wrap_into_window_type();
                self.settings = Some(self.new_window(window, "Settings"));
            }
        }



        let debug_data = game_event_manager.get_mut_debug_data();
        debug_data.clear("Window");
        debug_data.record("Window", format!("Windows Open: {}", self.windows.len()));

        

    }
}