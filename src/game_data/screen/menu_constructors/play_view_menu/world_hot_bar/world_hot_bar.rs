#![allow(dead_code)]

use crate::game_data::{game_event_manager::{prelude::Event, widget_event_manager::play_view_events::PlayViewEvent}, player_data::drone_script::var::var::Var, screen::{menu_constructors::play_view_menu::new_play_view::{PlayViewMode, RefManager}, widget::{button::button::Button, panel::panel::{PanelAlignment, PanelOrientation}, prelude::{PanelColor, TabPanel, VarSlot, play_world_view_config::RenderMode}, widget::WidgetType, world_rendering::rendering_config::cursor_config::CursorMode}}, types::{BlockTexture, UITextures}};

//=====================================
// Helper
//=====================================

pub fn free_camera_event(ref_manager: &mut RefManager) -> Event {
    PlayViewEvent::SetCursorMode(
        ref_manager.play_view_rendering_config.clone(),
        CursorMode::Free()
    ).wrap_into_event()
}

pub fn lock_camera_event(ref_manager: &mut RefManager) -> Event {
    PlayViewEvent::SetCursorMode(
        ref_manager.play_view_rendering_config.clone(),
        CursorMode::LockedToVar(ref_manager.selected_var.get_var_type_ref().clone())
    ).wrap_into_event()
}


// Back To Main
pub fn back_to_main_menu_button(ref_manager: &mut RefManager) -> WidgetType {
    let mut back_button = Button::new();
    back_button.set_icon(UITextures::LeftArrowIcon);
    back_button.add_left_click_event(PlayViewMode::Main.to_tab_panel_event(&ref_manager.play_view_mode));
    back_button.set_text("Back".to_string());

    // Set the cameras mode to lock into the location
    back_button.add_left_click_event(
        free_camera_event(ref_manager)
    );

    return WidgetType::Button(back_button);
}


//=====================================
// Main Hotbar
//=====================================


/// Main HotBar
fn get_main_hotbar_panel(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        panel.set_color(PanelColor::DarkUI);


        // Toggle create location
        let create_location_button = panel.add_button();

        // Set the cameras mode to lock into the location
        create_location_button.add_left_click_event(
            PlayViewEvent::SetCursorMode(
                ref_manager.play_view_rendering_config.clone(),
                CursorMode::LockedToVar(ref_manager.selected_var.get_var_type_ref().clone())
            ).wrap_into_event()
        );

        create_location_button.add_left_click_event(PlayViewMode::Location.to_tab_panel_event(&ref_manager.play_view_mode));
        create_location_button.set_text("Create Location".to_string());
        create_location_button.set_icon(UITextures::LocationIcon);

        // Toggle show drones
        let show_drones = panel.add_toggle_button();
        show_drones.set_toggle_ref(&ref_manager.show_drones_toggle);
        show_drones.set_block(BlockTexture::DroneBotRight);
        show_drones.set_text("Show Drones".to_string());
        
        // Toggle Show Locations
        let show_locations = panel.add_toggle_button();
        show_locations.set_toggle_ref(&ref_manager.show_locations_toggle);
        show_locations.set_icon(UITextures::AreaIcon);
        show_locations.set_text("Show Locations".to_string());
    

        
    }
    return panel;
}


//=====================================
// Drone
//=====================================

fn get_drone_hotbar(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        panel.set_color(PanelColor::DarkUI);
        
        // Back To Main
        panel.add_widget(back_to_main_menu_button(ref_manager));


        // Lock on drone button
        let toggle_button = panel.add_toggle_button();
        toggle_button.add_toggle_off_event(lock_camera_event(ref_manager));
        toggle_button.add_toggle_on_event(free_camera_event(ref_manager));
        toggle_button.set_icon(UITextures::CameraIcon);

    }
    return panel;
}

//=====================================
// Location Hotbar
//=====================================

fn get_location_hotbar(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        panel.set_color(PanelColor::DarkUI);
        
        panel.add_widget(back_to_main_menu_button(ref_manager));


        // Var Source Slot
        
        let var = Var::new_with_var_type(ref_manager.selected_var.get_var_type_ref().borrow().clone());
        let mut var_slot = VarSlot::new_with_var(var);

        var_slot.set_dragging_properties(true, false, false);     
        panel.add_widget(var_slot.wrap_into_widget());
         

        // Toggle Render Only Location
        let render_only_location_toggle = panel.add_toggle_button();
        
        render_only_location_toggle.set_toggle_ref(&ref_manager.render_only_selected_location);
        render_only_location_toggle.set_icon(UITextures::MapIcon);
        render_only_location_toggle.set_text("Render Only Location".to_string());
        
        render_only_location_toggle.add_toggle_off_event(
            PlayViewEvent::SetRenderMode(
                ref_manager.play_view_rendering_config.clone(),
                RenderMode::All
            ).wrap_into_event()
        );

        render_only_location_toggle.add_toggle_on_event(
            PlayViewEvent::SetRenderMode(
                ref_manager.play_view_rendering_config.clone(),
                RenderMode::VarOnly(ref_manager.selected_var.clone())
            ).wrap_into_event()
        );

        

        // Enter BluePrintMode
        let enter_blue_print_mode = panel.add_button();
        enter_blue_print_mode.add_left_click_event(PlayViewMode::BluePrint.to_tab_panel_event(&ref_manager.play_view_mode));
        enter_blue_print_mode.set_icon(UITextures::BluePrintIcon);
        enter_blue_print_mode.set_text("Blue Print Mode".to_string());
        
        // Set Location Entrence
        let set_location_entrence = panel.add_button();
        set_location_entrence.set_block(BlockTexture::TranslucentGreen);
        set_location_entrence.set_text("Set Entrence".to_string());

        // Set Location Entrence
        let set_location_exit = panel.add_button();
        set_location_exit.set_block(BlockTexture::TranslucentRed);
        set_location_exit.set_text("Set Exit".to_string());




        
        
    }
    return panel;
}


//=====================================
// Blueprint
//=====================================

fn get_blue_print_hotbar(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        panel.set_color(PanelColor::DarkUI);

        // Back To Location
        let back_button = panel.add_button();
        back_button.set_icon(UITextures::LeftArrowIcon);
        back_button.add_left_click_event(PlayViewMode::Location.to_tab_panel_event(&ref_manager.play_view_mode));
        back_button.set_text("Back".to_string());
        

        

        
    }
    return panel;
}



//=====================================
// Tab Panel Constructor
//=====================================

pub fn get_widget(ref_manager: &mut RefManager) -> WidgetType {    
    let mut tab_panel = TabPanel::new(&ref_manager.play_view_mode);

    tab_panel.set_button_panel_visiblity(false);

    tab_panel.add_panel(get_main_hotbar_panel(ref_manager));
    tab_panel.add_panel(get_location_hotbar(ref_manager));
    tab_panel.add_panel(get_blue_print_hotbar(ref_manager));
    tab_panel.add_panel(get_drone_hotbar(ref_manager));

    return tab_panel.wrap_into_widget();
}
    
//=====================================
// Tab Panel Event constructor
//=====================================