use std::collections::HashMap;

use miniquad::window;
use mlua::Debug;

use crate::game_data::{game_event_manager::debug_data::window_debug_data::WindowDebugData, player_data, screen::{ScreenData, menu_constructors::play_view_menu::new_play_view::PlayViewConstructionManager, screen_data, widget::{panel::panel::Panel, prelude::PlayWorldViewRender, widget::{Widget, WidgetType}, widget_calculations, widget_properties::WidgetProperties, window_manager::{widget_window_manager, window::WidgetWindow, windows::{debug_win::DebugWin, drone_spectate_win::DroneSpectateWindow, window_type::{Window, WindowType}}}, world_rendering::{tile_map_manager::TileMapManager, view_mode::ViewMode}}}, types::UITextures};


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

        for (id, window) in &mut self.windows {
            
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


        // render the windows
        self.windows.retain(|_, window| !window.close());

        for window in self.windows.values_mut() {
            if !window.minimized() {
                window.render(texture_manager, screen_data, game_event_manager, player_data);
            }
        }

        // Menu 
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::F3){
            
            if let Some(debug) = self.debug {
                if let Some(window) = self.windows.get_mut(&debug) {
                    window.focus()
                }
                else {
                    let debug = game_event_manager.get_mut_debug_data();
                    let window = DebugWin::new(debug).wrap_into_window_type();
                    self.debug = Some(self.new_window(window, "Debug"));
                }
            }
            else {
                let debug = game_event_manager.get_mut_debug_data();
                let window = DebugWin::new(debug).wrap_into_window_type();
                self.debug = Some(self.new_window(window, "Debug"));
            }


        }
        else if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::F2) {
           
           
           
            match player_data.get_view_mode() {
                Some(ViewMode::Drone(id)) => {
                    let window = DroneSpectateWindow::new(id).wrap_into_window_type();
                    self.new_window(window, "Drone");
                    println!("Opening Drone window");
                }
                _ => {

                }
            }
            
            
        }


        let debug_data = game_event_manager.get_mut_debug_data();
        debug_data.clear_window_data();

        let windows_open = format!("Windows Open: {}", self.windows.len());
        debug_data.add_window_data(windows_open);

        

    }
}