use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::render_event_manager::render_event_manager::RenderEvent, player_data::{drone_script::{action::{action::Action, action_type::ActionType}, function::function::Function, script_element, var::{game_vars::{game_var_type::{GameVarType, GameVarTypeKind}, primitive_var::{PrimitiveVarType, PrimitiveVarTypeKind}}, var_type::{VarType, VarTypeKind}}}, drones::drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, getter_actions::getter_actions::DroneGetterAction, prim_actions::{drone_invintory_actions::DroneInventoryAction, drone_prim_actions::DronePrimAction}}, player_data::PlayerData}, screen::{ScreenData, widget::{drone_programming::{action_slot::ActionSlot, function_slot::FunctionSlot, scripting_elements::scripting_panel, var_slot::var_slot::VarSlot}, panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, tab_panel::var_tab_panel::{self, VarTabPanel}, widget::{Widget, WidgetType}, widget_calculations::TextSize}}, types::{BlockTexture, drone_item::DroneItem}};




pub fn test_var_tab_panel() -> WidgetType {
    let var_ref = Rc::new(RefCell::new(PrimitiveVarType::Block(BlockTexture::Hive).wrap_into_var_type()));
    let var_tab_panel = VarTabPanel::new();



    return WidgetType::VarTabPanel(var_tab_panel);
    
}


pub fn test_script_panel() -> WidgetType {

    let mut panel = Panel::new_blank();

    let function = Function::new_blank();
    let function_slot = FunctionSlot::new_with_function(&function);
        
    panel.add_widget(function_slot.wrap_into_widget());
    

    return panel.wrap_into_widget()

}

pub fn get_menu(screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.set_color(PanelColor::Clear);
        panel.set_new_background(BackgroundType::Scrolling(crate::game_data::types::UITextures::MirrorBackground));

        let header = panel.add_text_display("Settings".to_string());
        header.set_text_scale(TextSize::ExtraLarge);


        // Back button
        let button = panel.add_button();
        button.add_left_click_event(RenderEvent::ChangeMenu(screen_data.get_current_menu()).wrap_into_event());
        button.set_icon(crate::game_data::types::UITextures::XIcon);
        button.set_text("Back".to_string());


        panel.add_widget(test_script_panel());
  

        panel.size();
    }

    return panel;

}