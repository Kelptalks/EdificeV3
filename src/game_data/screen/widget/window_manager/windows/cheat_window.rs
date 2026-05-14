use crate::game_data::{game_event_manager::player_data_event_manager::player_event_manager::PlayerDataEvent, player_data::game_entity::{block_entity_manager::{block_entity_manager::BlockEntity, natural::flungle::BlockEntityFlungle, player_created::{battery::BlockEntityBattery, radar::BlockEntityRadar}}, dynamic_entity_manager::{dynamic_entity_manager::DynamicEntity, natural::puff::DynamicEntityPuff}, game_entity_manager::GameEntity}, screen::widget::{panel::panel::Panel, widget::{Widget, WidgetType}, widget_properties::WidgetId, window_manager::windows::window_type::{Window, WindowType}}, types::BlockTexture};

pub struct CheatWindow {
    panel: Panel,

    spawn_radar_button_id: WidgetId,
    spawn_battery_button_id: WidgetId, 
    spawn_flungle_button_id: WidgetId,

    spawn_puff_button_id: WidgetId,
    
}

impl CheatWindow {
    pub fn new() -> CheatWindow {
        let mut panel = Panel::new_blank();

        let spawn_radar_button = panel.add_button();
        spawn_radar_button.add_texture(BlockTexture::LBM.wrap_into_texture());
        let spawn_radar_button_id = spawn_radar_button.get_id();


        let spawn_battery_button = panel.add_button();
        spawn_battery_button.add_texture(BlockTexture::Battery1.wrap_into_texture());
        let spawn_battery_button_id = spawn_battery_button.get_id();

        let spawn_fungle_button = panel.add_button();
        spawn_fungle_button.add_texture(BlockTexture::Flungle.wrap_into_texture());
        let spawn_flungle_button_id = spawn_fungle_button.get_id();

        let spawn_puff_button = panel.add_button();
        spawn_puff_button.add_texture(BlockTexture::PinkCloud.wrap_into_texture());
        let spawn_puff_button_id = spawn_puff_button.get_id();

        CheatWindow {
            panel,

            spawn_radar_button_id,
            spawn_battery_button_id,
            spawn_flungle_button_id,
            spawn_puff_button_id,
        }
    }
}

impl Window for CheatWindow {
    fn wrap_into_window_type(self) -> WindowType {
        WindowType::CheatWindow(self)
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {
        self.panel.render(texture_manager, screen_data, event_manager, player_data);


        if screen_data.was_left_pressed() {
            if let Some(WidgetType::Button(button)) = self.panel.find_widget_with_id(self.spawn_radar_button_id) {
                if button.mouse_on(screen_data) {
                    let cords = player_data.get_cursor().get_cords();
                    let radar = BlockEntityRadar::new(cords, event_manager);
                    event_manager.add_player_data_event(PlayerDataEvent::NewGameEntity(radar.wrap_into_game_entity()));
                }
            }

            if let Some(WidgetType::Button(button)) = self.panel.find_widget_with_id(self.spawn_battery_button_id) {
                if button.mouse_on(screen_data) {
                    let cords = player_data.get_cursor().get_cords();
                    let battery = BlockEntityBattery::new(cords, event_manager);
                    event_manager.add_player_data_event(PlayerDataEvent::NewGameEntity(battery.wrap_into_game_entity()));
                }
            }

            if let Some(WidgetType::Button(button)) = self.panel.find_widget_with_id(self.spawn_flungle_button_id) {
                if button.mouse_on(screen_data) {
                    let cords = player_data.get_cursor().get_cords();
                    let flungle = BlockEntityFlungle::new(cords, event_manager);
                    event_manager.add_player_data_event(PlayerDataEvent::NewGameEntity(flungle.wrap_into_game_entity()));
                }
            }


            if let Some(WidgetType::Button(button)) = self.panel.find_widget_with_id(self.spawn_puff_button_id) {
                if button.mouse_on(screen_data) {
                    let cords = player_data.get_cursor().get_cords();
                    let puff = DynamicEntityPuff::new(cords);
                    event_manager.add_player_data_event(PlayerDataEvent::NewGameEntity(puff.wrap_into_game_entity()));
                }
            }

        }
        


    }

    fn get_mut_panel(&mut self) -> &mut crate::game_data::screen::widget::panel::panel::Panel {
        &mut self.panel
    }

    fn get_panel(&self) -> &crate::game_data::screen::widget::panel::panel::Panel {
        &self.panel
    }
}