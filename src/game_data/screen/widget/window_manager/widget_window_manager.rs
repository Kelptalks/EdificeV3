use std::collections::HashMap;

use crate::game_data::{screen::{ScreenData, screen_data, widget::{panel::panel::Panel, prelude::PlayWorldViewRender, widget::{Widget, WidgetType}, widget_properties::WidgetProperties, window_manager::{widget_window_manager, window::WidgetWindow}}}, types::UITextures};


#[derive(Hash, PartialEq, Eq)]
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


    next_window_id: u32, 

    play_view: PlayWorldViewRender,
}

impl WidgetWindowManager {
    pub fn new(screen_data: &ScreenData) -> WidgetWindowManager {
        let mut widget_props = WidgetProperties::new_blank();
        
        // Force to take up entire screen
        widget_props.pos = screen_data.get_viewport_uv();
        widget_props.scale = screen_data.get_screen_rez();
        widget_props.parent_scale = screen_data.get_screen_rez();

        WidgetWindowManager {
            widget_props: widget_props,

            focused_window: None,
            open_windows: Vec::new(),
            minimized_windows: Vec::new(),
            
            windows: HashMap::new(),

            next_window_id: 0,
            play_view: PlayWorldViewRender::new(),
        }
    }

    fn get_next_window_id(&mut self) -> WidgetWindowId {
        let new_id = WidgetWindowId::new(self.next_window_id);
        self.next_window_id+=1;
        new_id
    }




    fn new_window(&mut self, mut panel: Panel) {
        let id = self.get_next_window_id();
        

        panel.set_parent_pos(self.widget_props.pos);
        panel.set_buffers([0.2; 4]);
        panel.size();

        let window = WidgetWindow::new(panel);
        self.windows.insert(id, window);
        println!("test");
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
        for (id, window) in &mut self.windows {
            window.set_parent_pos(parent_pos);
        }

    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        // render play view / Background
        // self.play_view.render(texture_manager, screen_data, game_event_manager, player_data);
        let temp_background = UITextures::FaceBackground.wrap_into_texture();
        texture_manager.render_texture(temp_background, self.widget_props.pos);


        // render the windows
        for (key, window) in &mut self.windows {
            if !window.minimized() {
                window.render(texture_manager, screen_data, game_event_manager, player_data);
            }
        
        }

        // Handle input
        if screen_data.get_input_manager().was_key_code_pressed(miniquad::KeyCode::C) {
            let mut panel = Panel::new_blank();
            panel.add_text_display("test".to_string());    
            
            
            self.new_window(panel);
        }

        




        

    }
}