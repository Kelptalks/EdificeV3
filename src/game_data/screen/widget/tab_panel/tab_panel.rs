use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::widget_event_manager::widget_event_manager::WidgetEvent, player_data::player_data::PlayerData, screen::{text::render_string_at_ndc, widget::{button::button::Button, panel::panel::Panel, widget::{Widget, WidgetType}, widget_calculations, widget_properties::{WidgetId, WidgetProperties}}}};

pub struct TabPanel {
    widget_properties: WidgetProperties,

    // Sub Panel Management
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

        let mut wp = WidgetProperties::new_blank();
        wp.internal_buffers = [0.0; 4];

        TabPanel {
            widget_properties: wp,

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
        let button = self.button_panel.add_button();

        button.add_left_click_event(
            WidgetEvent::SetUsizeEvent(self.current_panel_index.clone(), self.sub_panels.len() - 1).wrap_into_event());

        return button;
    }

    pub fn find_widget_with_id(&mut self, id: WidgetId) -> Option<&mut WidgetType> {
        if let Some(found) = self.button_panel.find_widget_with_id(id) {
            return Some(found);
        }
        for panel in &mut self.sub_panels {
            if let Some(found) = panel.find_with_id(id) {
                return Some(found);
            }
        }
        None
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
        let internal_buffers = self.widget_properties.internal_buffers;
        let mut sub_panel_buffer = internal_buffers;
        sub_panel_buffer[1] += top_offset;

        let pos = self.widget_properties.pos;

        for sub_panel in &mut self.sub_panels {
            let current_width = sub_panel.get_preffered_scale()[0];
            let x_center_offset = ((largest_width - current_width) / 2.0).max(0.0);

            let mut individual_buffer = sub_panel_buffer;
            individual_buffer[0] += x_center_offset;
            individual_buffer[2] += x_center_offset;

            sub_panel.set_parent_pos(pos);
            sub_panel.set_buffers(individual_buffer);
            sub_panel.size();
        }
    }

    fn size_with_buttons(&mut self) {
        let btn_preferred = self.button_panel.get_preffered_scale();
        let sub_preferred = self.largest_sub_panel_prefered_scale();

        self.widget_properties.prefered_scale = [
            sub_preferred[0].max(btn_preferred[0]),
            btn_preferred[1] + sub_preferred[1],
        ];

        let internal_buffers = self.widget_properties.internal_buffers;
        let pos = self.widget_properties.pos;
        let scale = self.widget_properties.scale;

        let button_buffer = [
            internal_buffers[0],
            internal_buffers[1],
            internal_buffers[2],
            internal_buffers[3] + (scale[1] - btn_preferred[1]),
        ];
        self.button_panel.set_parent_pos(pos);
        self.button_panel.set_buffers(button_buffer);
        self.button_panel.size();

        self.size_sub_panels(self.button_panel.get_scale()[1]);
    }

    fn size_without_buttons(&mut self) {
        let sub_preferred = self.largest_sub_panel_prefered_scale();
        self.widget_properties.prefered_scale = sub_preferred;

        self.size_sub_panels(0.0);
    }

}


impl Widget for TabPanel {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.widget_properties.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
    }

    fn size(&mut self) {
        self.widget_properties.scale_based_off_parent();

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
        game_event_manager: &mut crate::game_data::game_event_manager::event_manager::EventManager,
        player_data: &PlayerData,
    ) {
        let bounds = self.widget_properties.bounds;

        let current_index = *self.current_panel_index.borrow();
        if current_index >= self.sub_panels.len() {
            render_string_at_ndc(
                texture_manager,
                "ERROR TAB PANEL USIZE OUT OF RANGE OF SUBPANELS".to_string(),
                crate::game_data::types::FontType::Basic,
                widget_calculations::get_button_text_scale(),
                [self.widget_properties.pos[0], self.widget_properties.pos[1]],
            );
            return;
        }
        if self.sub_panels.len() > 0 {
            self.sub_panels[current_index].get_mut_widget_properties().bounds = bounds;
            self.sub_panels[current_index].render(texture_manager, screen_data, game_event_manager, player_data);
        }

        self.button_panel.get_mut_widget_properties().bounds = bounds;
        self.button_panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }
}
