
use crate::game_data::{player_data::player_data::PlayerData, screen::{ScreenData, menu_constructors::play_view_menu::new_play_view::PlayViewConstructionManager, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::{Widget, WidgetType}}}, types::UITextures};

/*
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
                    //row_panel.add_draggable_var(Var::Game(GameVar::Primitive(PrimitiveVar::Block(block_type))));
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
                    //row_panel.add_draggable_var(Var::Game(GameVar::Primitive(PrimitiveVar::DroneItem(item_type))));
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

    let location_manager = player_data.get_mut_location_manager();
    location_manager.add_all_location_to_selection_manager();
    let selection_panel = SelectionPanel::new(location_manager.get_panel_update_manager().clone());

    

    return selection_panel.wrap_into_widget();
}

pub fn get_drone_selection_panel(player_data: &mut PlayerData) -> WidgetType {
    
    let drone_manager = player_data.get_mut_drone_manager();
    drone_manager.add_all_drones_to_selection_manager();
    let selection_panel = SelectionPanel::new(drone_manager.get_panel_update_manager().clone());

    

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
*/
//=====================================
// Root panel
//=====================================

pub fn get_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    
    return get_new_menu(screen_data, player_data);

    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::TopLeft);
        
        // panel.set_background(crate::game_data::types::UITextures::VoidBackground);
        panel.set_new_background(BackgroundType::Scrolling(UITextures::VoidBackground));
        panel.set_color(PanelColor::Clear);

        // Add panels
        // self::add_menu_nav_panel(panel, screen_data, player_data);
        // control_panel::control_panel::add_control_panel(panel, screen_data, player_data);
        // self::add_selection_menu(panel, screen_data, player_data);

        panel.size();
    }

    return panel;

}


pub fn var_ref_bar() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {

        for i in 0..10 {
            //panel.add_var_slot(VarTypeKind::Any);
        }

    }
    return panel;
}

pub fn get_new_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    let mut play_view_menu_construct = PlayViewConstructionManager::new(player_data);

    return play_view_menu_construct.build_panel(screen_data, player_data);
}