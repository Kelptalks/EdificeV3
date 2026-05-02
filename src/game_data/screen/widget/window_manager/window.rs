use crate::game_data::screen::widget::{button::button::Button, panel::panel::Panel, widget::{Widget, WidgetType}, widget_properties::WidgetProperties, window_manager::widget_window_manager::WidgetWindowId};



pub struct WidgetWindow {
    widget_props: WidgetProperties,

    panel: Panel,


    top_bar_id: u32,
    min_button_id: u32,

    minimized: bool,
}


impl WidgetWindow {
    pub fn new(mut panel: Panel) -> WidgetWindow {
        

        let mut top_bar = Panel::new_blank();
        top_bar.add_text_display("UNKNOWN WINDOW".to_string());
        
        
        let button = Button::new();
        let minimize_button_id = top_bar.add_widget(button.wrap_into_widget());



        let top_bar_id = panel.add_widget(top_bar.wrap_into_widget());


        WidgetWindow {
            widget_props: WidgetProperties::new_blank(),
            panel: panel,

            top_bar_id: top_bar_id,
            min_button_id: minimize_button_id,

            minimized: false
        }
    }

    pub fn minimized(&self) -> bool {
        self.minimized
    }
}

impl Widget for WidgetWindow {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_props
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_props
    }

    fn size(&mut self) {
        self.panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        if let Some(WidgetType::Panel(top_bar)) = self.panel.get_mut_widget_with_id(self.top_bar_id) {
            if let Some(min_button) = top_bar.get_mut_widget_with_id(self.min_button_id) {
                if min_button.mouse_on(screen_data) && screen_data.was_left_pressed() {
                    self.minimized = true;
                }
            }
        } 
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }
}