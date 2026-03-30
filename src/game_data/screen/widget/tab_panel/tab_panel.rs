use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::game_event_manager::GameEvent, widget_event_manager::widget_event_manager::WidgetEvent}, screen::{render_string, text::render_string_at_ndc, widget::{button::{self, button::Button}, panel::panel::Panel, widget::{Widget, WidgetType}, widget_calculations}}, types::UITextures};

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
    
    show_button_panel: bool,
}



impl TabPanel {
    pub fn new(current_panel_index_ref: &Rc<RefCell<usize>>) -> TabPanel {
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
            current_panel_index: current_panel_index_ref.clone(),
            button_panel: button_panel,
            sub_panels: Vec::new(),

            show_button_panel: true,
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::TabPanel(self)
    }

    pub fn add_panel(&mut self, panel: WidgetType) -> &mut Button { 
        self.sub_panels.push(panel);
        let button =  self.button_panel.add_button();
        
        // Add Index modifyer event to button
        button.add_event(
            WidgetEvent::SetUsizeEvent(self.current_panel_index.clone(), self.sub_panels.len() - 1).wrap_into_event());

        return button;

    }

    pub fn set_button_panel_visiblity(&mut self, visible: bool) {
        self.show_button_panel = visible;
    }

    pub fn get_panel_ref(&self) -> &Rc<RefCell<usize>> {
        return &self.current_panel_index;
    }

    // =================================================
    // Sizing
    // =================================================
 
    fn largest_sub_panel_prefered_scale(&self) -> [f32; 2] {
        let mut largest = [0.0f32; 2];
        for sub_panel in &self.sub_panels {
            let scale = sub_panel.get_preffered_scale();
            if scale[0] > largest[0] { largest[0] = scale[0]; }
            if scale[1] > largest[1] { largest[1] = scale[1]; }
        }
        largest
    }
 
    fn size_sub_panels(&mut self, top_offset: f32) {
        let largest_width = self.largest_sub_panel_prefered_scale()[0];
        let mut sub_panel_buffer = self.internal_buffers;
        sub_panel_buffer[1] += top_offset;

        for sub_panel in &mut self.sub_panels {
            let current_width = sub_panel.get_preffered_scale()[0];
            let x_center_offset = ((largest_width - current_width) / 2.0).max(0.0);

            let mut individual_buffer = sub_panel_buffer;
            individual_buffer[0] += x_center_offset; // left
            individual_buffer[2] += x_center_offset; // right

            sub_panel.set_parent_pos(self.pos);
            sub_panel.set_buffers(individual_buffer);
            sub_panel.size();
        }
    }
 
    /// Sizing when the button panel is visible.
    /// Layout (top → bottom): [button row] [sub panel]
    fn size_with_buttons(&mut self) {
        let btn_preferred = self.button_panel.get_preffered_scale();
        let sub_preferred = self.largest_sub_panel_prefered_scale();
 
        self.prefered_scale = [
            sub_preferred[0].max(btn_preferred[0]),
            btn_preferred[1] + sub_preferred[1],
        ];
 
        // Button panel is pinned to the top; push its bottom buffer so it
        // occupies only its preferred height.
        let button_buffer = [
            self.internal_buffers[0],
            self.internal_buffers[1],
            self.internal_buffers[2],
            self.internal_buffers[3] + (self.scale[1] - btn_preferred[1]),
        ];
        self.button_panel.set_parent_pos(self.pos);
        self.button_panel.set_buffers(button_buffer);
        self.button_panel.size();
 
        // Sub panels start below the button row.
        self.size_sub_panels(self.button_panel.get_scale()[1]);
    }
 
    /// Sizing when the button panel is hidden.
    /// Sub panels fill the entire TabPanel area.
    fn size_without_buttons(&mut self) {
        let sub_preferred = self.largest_sub_panel_prefered_scale();
        self.prefered_scale = sub_preferred;
 
        self.size_sub_panels(0.0);
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
 
        if self.show_button_panel {
            self.size_with_buttons();
        } else {
            self.size_without_buttons();
        }
    }

    fn render(
        &mut self, 
        texture_manager: &mut crate::game_data::TextureManager, 
        screen_data: &crate::game_data::screen::ScreenData, 
        game_event_manager: &mut crate::game_data::game_event_manager::event_manager::EventManager
    ) {
        
        let current_index = *self.current_panel_index.borrow();
        if current_index >= self.sub_panels.len() {
            render_string_at_ndc(
                texture_manager, 
                "ERROR TAB PANEL USIZE OUT OF RANGE OF SUBPANELS".to_string(), 
                crate::game_data::types::FontType::Basic,
                widget_calculations::get_button_text_scale(), 
                [self.pos[0], self.pos[1]], 
            );
            return;
        }
        if self.sub_panels.len() > 0 {
            self.sub_panels[current_index].render(texture_manager, screen_data, game_event_manager);
        }


        
        self.button_panel.render(texture_manager, screen_data, game_event_manager);
        // texture_manager.render_ui_element_with_pos(UITextures::ScallingIconMidCenter, self.pos);
    }
}