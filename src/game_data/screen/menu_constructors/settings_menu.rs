use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    game_event_manager::render_event_manager::render_event_manager::RenderEvent, 
    player_data::{drone_script::{action::{action::Action, action_type::ActionType}, 
    script_element, var::{game_vars::{game_var_type::{GameVarKind, GameVarType}, primitive_game_var::{PrimitiveGameVarType, PrimitiveGameVarTypeKind}}, var_type::{VarKind, VarType}}}, drones::drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, getter_actions::getter_actions::DroneGetterAction, prim_actions::{drone_invintory_actions::DroneInventoryAction, drone_prim_actions::DronePrimAction}}, player_data::PlayerData}, screen::{ScreenData, widget::{drone_programming::{action_slot::ActionSlot, function_slot::FunctionSlot, scripting_elements::scripting_panel, var_slot::var_slot::VarSlot}, panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, tab_panel::var_tab_panel::{self, VarTabPanel}, text::{header::TextDisplay, text_input::TextInput}, widget::{Widget, WidgetType}, widget_calculations::TextSize, window_manager::widget_window_manager::{self, WidgetWindowManager}}}, types::{BlockTexture, drone_item::DroneItem}};



pub fn get_menu(screen_data: &ScreenData) -> WidgetType {
    
    let panel = Panel::new_blank();

    panel.wrap_into_widget()


}