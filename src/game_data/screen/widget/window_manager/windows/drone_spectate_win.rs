use std::fmt::format;

use crate::game_data::{player_data::{drones::{drone, drone_manager::DroneId}, game_entity::game_entity_manager::GameEntityId}, screen::widget::{drone_programming::var_slot::var_prop_widgets::{invintory_display_prop_widget::InvintoryDisplayPropWidget, text_display_prop_widget}, game_object_prop_displays::invintory_display::InvintoryDisplayWidget, panel::panel::Panel, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_properties::{WidgetId, WidgetProperties}, window_manager::windows::window_type::Window}};

pub struct DroneSpectateWindow {
    drone_id: DroneId,
    

    cords_widget_id: WidgetId,


    panel: Panel,


}

impl DroneSpectateWindow {
    pub fn new(id: DroneId) -> DroneSpectateWindow {
        let mut panel = Panel::new_blank();
        
        let cords = TextDisplay::new("cords".to_string());
        let cords_widget_id = cords.get_id();
        let invintory_widget = InvintoryDisplayWidget::new(panel.get_widget_properties(),GameEntityId::Drone(id));
        panel.add_widget(WidgetType::InvintoryDisplayWidget(invintory_widget));

        panel.add_widget(cords.wrap_into_widget());


        DroneSpectateWindow {
            drone_id: id,    
            
            cords_widget_id,
            
            panel
        }
    }
}


impl Window for DroneSpectateWindow {
    fn wrap_into_window_type(self) -> super::window_type::WindowType {
        super::window_type::WindowType::DroneSpectate(self)
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {


        let drone = player_data.get_drone_event_scheduler(self.drone_id);
        if let Some(drone) = drone {
            let drone = drone.get_drone();

            if let Some(cords_widget) = self.panel.find_widget_with_id(self.cords_widget_id) {
                if let WidgetType::TextDisplay(display) = cords_widget {
                    let text = format!("Cords: {:?}", drone.get_cords());
                    display.set_text(text);
                }
            }
        

        }

        
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }

    fn get_mut_panel(&mut self) -> &mut crate::game_data::screen::widget::panel::panel::Panel {
        &mut self.panel
    }

    fn get_panel(&self) -> &crate::game_data::screen::widget::panel::panel::Panel {
        &self.panel
    }
}