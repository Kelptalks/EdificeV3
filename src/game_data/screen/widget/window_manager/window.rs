use crate::game_data::screen::widget::{button::button::Button, panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::{Widget, WidgetType}, widget_properties::{self, WidgetId, WidgetProperties}, window_manager::{widget_window_manager::WidgetWindowId, windows::window_type::WindowType}};



pub struct WidgetWindow {
    panel: Panel,

    top_bar_id: WidgetId,
    close_button_id: WidgetId,
    minimize_button_id: WidgetId,

    window_content_id: WidgetId,

    held: bool,
    offset: [f32; 2],

    minimized: bool,
    close: bool,
}


impl WidgetWindow {
    
    
    pub fn new(window: WindowType, parent_props: &WidgetProperties, name: &str) -> WidgetWindow {

        let mut window_panel = Panel::new_with_parent_props(parent_props);
        window_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
        let top_bar = window_panel.add_sub_panel();
        let top_bar_id = top_bar.get_id();

        top_bar.set_orientation(PanelOrientation::Horizontal, PanelAlignment::TopLeft);
        top_bar.set_color(crate::game_data::screen::widget::prelude::PanelColor::BrightGreen);

        let text_panel = top_bar.add_sub_panel();
        text_panel.add_text_display(name.to_string());

        let button_panel = top_bar.add_sub_panel();
        button_panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::BotRight);


        let minimize = button_panel.add_button();
        minimize.set_text("minimize".to_string());
        let minimize_id = minimize.get_id();

        let close = button_panel.add_button();
        close.set_text("close".to_string());
        close.set_icon(crate::game_data::types::UITextures::XIcon);
        let close_id = close.get_id();

        


        let content_id = window.get_id();
        window_panel.add_widget(window.wrap_into_widget());
        

        let mut widget_window = WidgetWindow {
            panel: window_panel,


            // Tab bar
            top_bar_id: top_bar_id,
            close_button_id: close_id,
            minimize_button_id: minimize_id,


            window_content_id: content_id,

            held: false,
            offset: [0.0; 2],

            minimized: false,
            close: false,
        };
        widget_window.size();

        widget_window
    }

    pub fn get_offset(&self) -> [f32; 2] {
        self.offset
    }

    pub fn minimized(&self) -> bool {
        self.minimized
    }

    pub fn close(&self) -> bool {
        self.close
    }
}

impl Widget for WidgetWindow {
    fn get_widget_properties(&self) -> &WidgetProperties {
        self.panel.get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        self.panel.get_mut_widget_properties()
    }

    fn size(&mut self) {
        self.panel.size();
        self.panel.set_parent_pos(self.get_pos());
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        if screen_data.was_left_pressed() {
            if let Some(min_button) = self.panel.find_widget_with_id(self.minimize_button_id) {
                if min_button.mouse_on(screen_data) {
                    self.minimized = true;
                }
            }

            if let Some(min_button) = self.panel.find_widget_with_id(self.close_button_id) {
                if min_button.mouse_on(screen_data) {
                    self.close = true;
                }
            }
        }

        if let Some(top_bar) = self.panel.find_widget_with_id(self.top_bar_id) {
            if top_bar.mouse_on(screen_data) {
                if screen_data.get_input_manager().get_mouse_input_data().was_left_clicked() {
                    self.held = true;
                }
                else if !screen_data.is_left_mouse_held() {
                    self.held = false;
                }

                if self.held {
                    let mouse_change = screen_data.get_change_in_mouse_ndc();
                    self.offset[0] -= mouse_change[0];
                    self.offset[1] -= mouse_change[1];
                }
            }


        }



        
        
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }
}