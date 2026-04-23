use std::{cell::RefCell, collections::binary_heap, rc::Rc};

use crate::game_data::{game_event_manager::{player_data_event_manager::var_event_manager::var_events::{DynamicVarEvent, LocationVarEvent, VarEvents}, prelude::{Event, InputEvent}}, locations::world_area_side::WorldAreaSide, screen::{menu_constructors::play_view_menu::new_play_view::RefManager, widget::{button::button::Button, panel::panel::{PanelAlignment, PanelOrientation}, prelude::{PanelColor, TabPanel}, text::text_input::TextInput, widget::WidgetType, widget_calculations::TextSize}}, types::UITextures};



//=====================================
// Location Scalling
//=====================================
fn get_expand_event(ref_manager: &mut RefManager, side: WorldAreaSide) -> Event {
    VarEvents::LocationVarEvent(
        ref_manager.selected_var.get_var_type_ref().clone(), LocationVarEvent::ExpandLocationSide(side)
    ).wrap_into_event()
}

fn get_shrink_event(ref_manager: &mut RefManager, side: WorldAreaSide) -> Event {
    VarEvents::LocationVarEvent(
        ref_manager.selected_var.get_var_type_ref().clone(), LocationVarEvent::ShrinkLocationSide(side)
    ).wrap_into_event()
}

fn construct_scaling_button(ref_manager: &mut RefManager, icon: UITextures, side: WorldAreaSide) -> WidgetType {
    let mut button = Button::new();

    button.add_left_click_event(get_expand_event(ref_manager, side.clone()));
    button.add_right_click_event(get_shrink_event(ref_manager, side));
    button.set_icon(icon);

    return WidgetType::Button(button);
}

fn get_location_scalling_mods(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display("scalling controls".to_string()).set_text_scale(TextSize::Small);

        let top_row = panel.add_sub_panel();
        top_row.set_color(PanelColor::Clear);

        top_row.add_widget(
            construct_scaling_button(
                ref_manager, 
                UITextures::ScallingIconTopLeft, 
                WorldAreaSide::XMinus
            )
        );

        top_row.add_widget(
            construct_scaling_button(
                ref_manager, 
                UITextures::ScallingIconTopMid, 
                WorldAreaSide::ZPlus
            )
        );

        top_row.add_widget(
            construct_scaling_button(
                ref_manager, 
                UITextures::ScallingIconTopRight, 
                WorldAreaSide::YMinus
            )
        );


        let bot_row = panel.add_sub_panel();
        bot_row.set_color(PanelColor::Clear);

        bot_row.add_widget(
            construct_scaling_button(
                ref_manager, 
                UITextures::ScallingIconBotLeft, 
                WorldAreaSide::YPlus
            )
        );

        bot_row.add_widget(
            construct_scaling_button(
                ref_manager, 
                UITextures::ScallingIconBotMid, 
                WorldAreaSide::ZMinus
            )
        );

        bot_row.add_widget(
            construct_scaling_button(
                ref_manager, 
                UITextures::ScallingIconBotRight, 
                WorldAreaSide::XPlus
            )
        );

        panel.size();
    }
    return panel;
}

//=====================================
// Location Naming
//=====================================

fn get_location_rename_panel(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);
        panel.set_color(PanelColor::DarkUI);

        let rename_ref = Rc::new(RefCell::new(ref_manager.selected_var.get_var_type_ref().borrow().get_name()));

        let text_input = TextInput::new_text_input(&rename_ref);
        panel.add_widget(WidgetType::TextInput(text_input));

        let button = panel.add_button();
        button.add_left_click_event(VarEvents::DynamicVarEvent(ref_manager.selected_var.get_var_type_ref().clone(), DynamicVarEvent::Rename(rename_ref)).wrap_into_event());

        panel.size();
    }
    return panel;
}

//=====================================
// Main Constructor
//=====================================
pub fn get_location_panel(ref_manager: &mut RefManager) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::DarkUI);

        let panel_title = panel.add_text_display("Location".to_string());
        panel_title.set_text_scale(TextSize::Large);
        

        let location_name = panel.add_text_display(ref_manager.selected_var.get_var_type_ref().borrow().get_name());
        location_name.set_text_scale(TextSize::Medium);


        // Location Modification buttons 
        panel.add_widget(self::get_location_scalling_mods(ref_manager));
        panel.add_widget(self::get_location_rename_panel(ref_manager));


        panel.size();
    }
    return panel;
}
