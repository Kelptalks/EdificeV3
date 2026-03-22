use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::game_event_manager::GameEvent, widget_event_manager::widget_event_manager::WidgetEvent}, screen::widget::{button::{self, button::Button}, panel::panel::Panel, widget::{Widget, WidgetType}, widget_calculations}, types::UITextures};

pub struct TabPanel {
    // Parent 
    prefered_scale: [f32; 2],
    parent_pos: [f32; 4],
    external_buffers: [f32; 4],

    // Self
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    scale: [f32; 2],

    // Sub Panel Managment
    current_panel_index: Rc<RefCell<usize>>,
    button_panel: Panel,
    sub_panels: Vec<WidgetType>,
}



impl TabPanel {
    pub fn new() -> TabPanel {
        let mut button_panel = Panel::new([0.0; 4], [0.0; 4]);
        button_panel.set_orientation(
            crate::game_data::screen::widget::panel::panel::PanelOrientation::Horizontal, 
            crate::game_data::screen::widget::panel::panel::PanelAlignment::Center
        );
        TabPanel {
            // Parent
            prefered_scale: [0.0; 2],
            parent_pos: [0.0; 4],
            external_buffers: [0.0; 4],

            // Self
            pos: [0.0; 4],
            internal_buffers: [0.0; 4],
            scale: [0.0; 2],

            // Sub Panel Management
            current_panel_index: Rc::new(RefCell::new(0)),
            button_panel: button_panel,
            sub_panels: Vec::new(),
        }
    }


    pub fn add_panel(&mut self, panel: WidgetType) -> &mut Button { 
        self.sub_panels.push(panel);
        let button =  self.button_panel.add_button();
        
        // Add Index modifyer event to button
        button.add_event(
            WidgetEvent::SetUsizeEvent(self.current_panel_index.clone(), self.sub_panels.len() - 1).wrap_into_event());

        return button;

    }

}


impl Widget for TabPanel {
    fn get_pos(&self) -> [f32; 4] {
        self.pos
    }

    fn get_scale(&self) -> [f32; 2] {
        self.scale
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
    }

    fn size(&mut self) {
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);

        // Size button first
        let button_prefred_scale = self.button_panel.get_preffered_scale();
        let mut largest_prefered_scale = [0.0; 4];
        for sub_panel in &mut self.sub_panels {
            let current_panel_prefered_scale = sub_panel.get_preffered_scale();
            // X needs to check button and sub panel prefered scale
            if largest_prefered_scale[0] < current_panel_prefered_scale[0] {
                largest_prefered_scale[0] = current_panel_prefered_scale[0];
            }
            if largest_prefered_scale[0] < button_prefred_scale[0] {
                largest_prefered_scale[0] = button_prefred_scale[0];
            }

            // Y just needs largest
            if largest_prefered_scale[1] < current_panel_prefered_scale[1] {
                largest_prefered_scale[1] = current_panel_prefered_scale[1];
            }
        }
        
        self.prefered_scale = [
            largest_prefered_scale[0],
            button_prefred_scale[1] + largest_prefered_scale[1],
        ];


        let button_buffer = [
            self.internal_buffers[0],
            self.internal_buffers[1],
            self.internal_buffers[2],
            self.internal_buffers[3] + (self.scale[1] - button_prefred_scale[1]),
        ];
        self.button_panel.set_parent_pos(self.pos);
        self.button_panel.set_buffers(button_buffer);
        self.button_panel.size();


        let sub_panel_buffer = [
            self.internal_buffers[0],
            self.internal_buffers[1] + self.button_panel.get_scale()[1],
            self.internal_buffers[2],
            self.internal_buffers[3],
        ];
        for sub_panel in &mut self.sub_panels {
            sub_panel.set_parent_pos(self.pos);            
            sub_panel.set_buffers(sub_panel_buffer);
            sub_panel.size();
        }
    }

    fn render(
        &mut self, 
        texture_manager: &mut crate::game_data::TextureManager, 
        screen_data: &crate::game_data::screen::ScreenData, 
        game_event_manager: &mut crate::game_data::game_event_manager::event_manager::EventManager
    ) {
        
        let current_index = *self.current_panel_index.borrow();
        if self.sub_panels.len() > 0 {
            self.sub_panels[current_index].render(texture_manager, screen_data, game_event_manager);
        }


        
        self.button_panel.render(texture_manager, screen_data, game_event_manager);
        // texture_manager.render_ui_element_with_pos(UITextures::ScallingIconMidCenter, self.pos);
    }
}