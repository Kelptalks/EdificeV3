use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{drone_script::function::function::Function, player_data::PlayerData}, screen::widget::{drone_programming::function_slot::FunctionSlot, panel::panel::{Panel, PanelAlignment, PanelOrientation}, scroll_panel::scroll_panel::ScrollPanel, widget::{Widget, WidgetType}, widget_properties::WidgetProperties}};

pub struct ScriptingPanel {
    widget_properties: WidgetProperties,
    panel: Panel,
    scroll_panel: ScrollPanel,
}

impl ScriptingPanel {
    pub fn new(function: &Rc<RefCell<Function>>) -> ScriptingPanel {
        let mut panel = Panel::new_blank();

        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display("Scripting Panel".to_string());



        let function_slot = FunctionSlot::new_with_function(function);

        let scroll_panel = ScrollPanel::new();
        panel.add_widget(function_slot.wrap_into_widget());


        let mut scripting_panel = ScriptingPanel {
            widget_properties: WidgetProperties::new_blank(),
            panel: panel,
            scroll_panel: scroll_panel,
        };

        scripting_panel.widget_properties.prefered_scale = [0.5; 2];

        scripting_panel
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        return WidgetType::ScriptingPanel(self)
    }

}

impl Widget for ScriptingPanel {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        let scroll_scale = self.scroll_panel.get_preffered_scale();
        let panel_scale = self.panel.get_preffered_scale();

        let x_scale = 0.5;
        let y_scale = scroll_scale[1] + panel_scale[1];

        [x_scale, y_scale]
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.widget_properties.external_buffers = pos;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
    }

    fn size(&mut self) {
        self.widget_properties.prefered_scale = self.get_preffered_scale();

        self.widget_properties.scale_based_off_parent();
        let pos   = self.widget_properties.pos;
        let scale = self.widget_properties.scale;

        self.panel.set_parent_pos(pos);
        self.scroll_panel.set_parent_pos(pos);

        // Size panel once to compute preferred height from content
        self.panel.set_buffers([0.0; 4]);
        self.panel.size();
        let panel_preferred_height = self.panel.get_preffered_scale()[1];

        // Constrain panel to its preferred height
        let bottom_buffer = (scale[1] - panel_preferred_height).max(0.0);
        self.panel.set_buffers([0.0, 0.0, 0.0, bottom_buffer]);
        self.panel.size();

        // Position scroll_panel immediately below the panel
        self.scroll_panel.set_buffers([0.0, panel_preferred_height, 0.0, 0.0]);
        self.scroll_panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &PlayerData,
    ) {
    
        self.size();

        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
        self.scroll_panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }
}
