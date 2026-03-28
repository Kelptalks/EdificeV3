
use crate::game_data::{game_event_manager::{game_event_manager::GameEvent, input_event_manager::input_event_manager::InputEvent, player_data_event_manager::{location_event::LocationEvent, player_event_manager::PlayerDataEvent}, render_event_manager::render_event_manager::RenderEvent}, locations::world_area::WorldArea, player_data::{drone_programming::var::{game_vars::game_var_type::{GameVar, PrimitiveVar}, var_type::Var}, locations::location_manager, player_data::PlayerData}, screen::{ScreenData, menu_constructors::play_view_menu::control_panel, screen_data::CurrentMenu, widget::{panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, selection_panel::selection_panel::SelectionPanel, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_calculations::TextSize, world_rendering::play_world_view_config::PlayViewRendingConfig}}, types::{BlockTexture, UITextures, drone_item::DroneItem}};

//=====================================
// Selection Panel
//=====================================

pub fn get_block_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        // Header
        let text_display = panel.add_text_display("Blocks".to_string());
        text_display.set_text_scale(TextSize::Medium);

        let scroll_panel = panel.add_scroll_panel();
        

        
        let blocks_per_row = 5;
        for collumn_block_id in (0..BlockTexture::get_total_blocks()).step_by(blocks_per_row) {

            let mut block_selection_panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
            if let WidgetType::Panel(row_panel) = &mut block_selection_panel {
                row_panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
                row_panel.set_color(PanelColor::Clear);

                for row_block_id in 0..blocks_per_row {
                    
                    let block_type = BlockTexture::from_id(collumn_block_id as u16 + row_block_id as u16);
                    row_panel.add_draggable_var(Var::Game(GameVar::Primitive(PrimitiveVar::Block(block_type))));
                }
            }
            scroll_panel.add_widget(block_selection_panel);
        }

        scroll_panel.set_prefered_scale(0.8);
        panel.size();
    }

    return panel;
}

pub fn get_item_selection_panel() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Dark);

        // Header
        let text_display = panel.add_text_display("Items".to_string());
        text_display.set_text_scale(TextSize::Medium);

        let scroll_panel = panel.add_scroll_panel();
        
        let items_per_row = 5;
        for collumn_item_id in (0..DroneItem::get_total_items()).step_by(items_per_row) {

            let mut item_selection_panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
            if let WidgetType::Panel(row_panel) = &mut item_selection_panel {
                row_panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
                row_panel.set_color(PanelColor::Clear);

                for row_item_id in 0..items_per_row {
                    
                    let item_type = DroneItem::from_id(collumn_item_id as u32 + row_item_id as u32);
                    row_panel.add_draggable_var(Var::Game(GameVar::Primitive(PrimitiveVar::DroneItem(item_type))));
                }
            }
            scroll_panel.add_widget(item_selection_panel);
        }

        scroll_panel.set_prefered_scale(0.8);
        panel.size();
    }

    return panel;
}

pub fn get_location_selection_panel(player_data: &mut PlayerData) -> WidgetType {

    let mut selection_panel = SelectionPanel::new(player_data.get_mut_location_manager().get_panel_update_manager().clone());

    

    return selection_panel.wrap_into_widget();
}

pub fn get_drone_selection_panel(player_data: &mut PlayerData) -> WidgetType {
    let mut selection_panel = SelectionPanel::new(player_data.get_mut_drone_manager().get_panel_update_manager().clone());

    

    return selection_panel.wrap_into_widget();
}

pub fn add_selection_menu(panel: &mut Panel, screen_data: &ScreenData, player_data: &mut PlayerData) {
    let selection_sub_panel = panel.add_sub_panel();
    selection_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

    // Header
    let text_display = selection_sub_panel.add_text_display("Variables".to_string());
    text_display.set_text_scale(TextSize::Large);


    let selection_tab_panel = selection_sub_panel.add_tab_panel();

    // Add Block selection
    let button = selection_tab_panel.add_panel(get_block_selection_panel());
    button.set_text("Blocks".to_string());
    button.set_block(crate::game_data::types::BlockTexture::Grass);

    // Add Item selection
    let button = selection_tab_panel.add_panel(get_item_selection_panel());
    button.set_text("Items".to_string());
    button.set_icon(UITextures::ScallingIconMidCenter);

    // Add Location Selection
    let button = selection_tab_panel.add_panel(get_location_selection_panel(player_data));
    button.set_text("Locations".to_string());
    button.set_icon(UITextures::AreaIcon);

    // Add Drone Selection
    let button = selection_tab_panel.add_panel(get_drone_selection_panel(player_data));
    button.set_text("Drones".to_string());
    button.set_block(crate::game_data::types::BlockTexture::DroneBotRight);

}

//=====================================
// Menu Nav Panel
//=====================================


pub fn add_menu_nav_panel(panel: &mut Panel, screen_data: &ScreenData, player_data: &mut PlayerData) {
    let menu_nav_sub_panel = panel.add_sub_panel();
    menu_nav_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

    // Back button
    let button = menu_nav_sub_panel.add_button();
    button.add_event(RenderEvent::ChangeMenu(CurrentMenu::MapView).wrap_into_event());
    button.set_icon(crate::game_data::types::UITextures::MapIcon);
    button.set_text("Map".to_string());

    let button = menu_nav_sub_panel.add_button();
    button.add_event(RenderEvent::ChangeMenu(CurrentMenu::SettingsMenu).wrap_into_event());
    button.set_icon(crate::game_data::types::UITextures::SettingsIcon);
    button.set_text("Settings".to_string());

    let button = menu_nav_sub_panel.add_button();
    button.add_event(RenderEvent::ChangeMenu(CurrentMenu::MainMenu).wrap_into_event());
    button.set_icon(crate::game_data::types::UITextures::XIcon);
    button.set_text("Main Menu".to_string());
}

//=====================================
// Root panel
//=====================================

pub fn get_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::TopLeft);
        
        // panel.set_background(crate::game_data::types::UITextures::VoidBackground);
        panel.set_new_background(BackgroundType::Scrolling(UITextures::VoidBackground));
        panel.set_color(PanelColor::Clear);

        // Add panels
        self::add_menu_nav_panel(panel, screen_data, player_data);
        control_panel::control_panel::add_control_panel(panel, screen_data, player_data);
        self::add_selection_menu(panel, screen_data, player_data);

        panel.size();
    }

    return panel;

}