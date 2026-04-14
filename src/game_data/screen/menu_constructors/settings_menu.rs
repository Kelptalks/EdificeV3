use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::render_event_manager::render_event_manager::RenderEvent, player_data::{drone_programming::{control_flow::control_flow::ControlFlow, function::function::Function, script_element, var::{game_vars::{game_var_type::{GameVar, GameVarTypeKind}, primitive_var::{PrimitiveVar, PrimitiveVarTypeKind}}, var_type::{Var, VarTypeKind}}}, drones::drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, getter_actions::getter_actions::DroneGetterAction}, player_data::PlayerData}, screen::{ScreenData, widget::{drone_programming::{scripting_elements::scripting_panel, vars::var_slot::VarSlot}, panel::{panel::{Panel, PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, tab_panel::var_tab_panel::{self, VarTabPanel}, widget::{Widget, WidgetType}, widget_calculations::TextSize}}, types::{BlockTexture, drone_item::DroneItem}};



pub fn test_var_slots() -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Blocks
        panel.add_text_display("Block Slots".to_string());
        let block_sub_panel = panel.add_sub_panel();
        let blocks = [BlockTexture::Air, BlockTexture::Stone, BlockTexture::Granite, BlockTexture::Grass, BlockTexture::BlueGrass];
        for i in 0..blocks.len() {
            let var_instance = Rc::new(RefCell::new(Var::Game(GameVar::Primitive(PrimitiveVar::Block(blocks[i])))));
            let mut var_slot = VarSlot::new(&var_instance);

            var_slot.set_allowed_type(VarTypeKind::Game(GameVarTypeKind::Primitive(PrimitiveVarTypeKind::Block)));
            var_slot.set_dragging_properties(true, false, false);

            block_sub_panel.add_widget(var_slot.wrap_into_widget());
        }

        // Items
        panel.add_text_display("Item Slots".to_string());
        
        let item_sub_panel = panel.add_sub_panel();
        let items = [DroneItem::Ash, DroneItem::Dirt, DroneItem::PlantMatter, DroneItem::IronBattery, DroneItem::DroneChassis];
        for i in 0..items.len() {
            let var_instance = Rc::new(RefCell::new(Var::Game(GameVar::Primitive(PrimitiveVar::DroneItem(items[i])))));
            let mut var_slot = VarSlot::new(&var_instance);

            var_slot.set_allowed_type(VarTypeKind::Game(GameVarTypeKind::Primitive(PrimitiveVarTypeKind::DroneItem)));
            var_slot.set_dragging_properties(true, false, false);

            item_sub_panel.add_widget(var_slot.wrap_into_widget());
        }



        // Slots Types
        panel.add_text_display("Block Selectors".to_string());
        let slot_sub_panel = panel.add_sub_panel();
        
        
        let var_instance = Rc::new(RefCell::new(Var::Game(GameVar::Primitive(PrimitiveVar::DroneItem(DroneItem::Ash)))));
        let mut var_slot = VarSlot::new(&var_instance);


        // var_slot.set_allowed_type(VarTypeKind::Game(GameVarTypeKind::Primitive(PrimitiveVarTypeKind::DroneItem)));
        var_slot.set_dragging_properties(true, true, true);

        slot_sub_panel.add_widget(var_slot.wrap_into_widget());
    }

    return panel;
}

pub fn test_var_tab_panel() -> WidgetType {
    let var_ref = Rc::new(RefCell::new(PrimitiveVar::Block(BlockTexture::Hive).wrap_into_var()));
    let var_tab_panel = VarTabPanel::new(&var_ref);



    return WidgetType::VarTabPanel(var_tab_panel);
    
}


pub fn test_script_panel() -> WidgetType {

    let mut panel = Panel::new_blank();

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