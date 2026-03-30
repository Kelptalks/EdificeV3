use std::{cell::RefCell, rc::Rc};

use miniquad::KeyCode;

use crate::game_data::{game_event_manager::prelude::{Event, InputEvent, WidgetEvent}, player_data::{drone_programming::var::{game_vars::game_var_type::{GameVarTypeKind, PrimitiveVarTypeKind}, var_type::{VarTypeKind}}, player_data::PlayerData}, screen::{menu_constructors::play_view_menu::control_panel::building_panel::building_world_view, widget::{drone_programming::vars::var_slot::VarSlot, panel::panel::{PanelAlignment, PanelOrientation}, widget::WidgetType}}, types::BlockTexture};

pub fn get_block_hotbar_input_events(block_slot_refs: Vec<Rc<RefCell<BlockTexture>>>, block_selected_ref: Rc<RefCell<BlockTexture>>) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    let key_inputs = [
        KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, 
        KeyCode::Key4, KeyCode::Key5, KeyCode::Key6,
        KeyCode::Key7, KeyCode::Key8, KeyCode::Key9];

    for (i, block_slot_ref) in block_slot_refs.iter().enumerate() {
        let block_selected_event = WidgetEvent::SetBlockRef(block_selected_ref.clone(), block_slot_ref.clone()).wrap_into_event_vec();
        events.push(InputEvent::KeyDown(key_inputs[i], block_selected_event).wrap_into_event());


    }

    

    return events;
}

pub fn get_building_panel(player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);

        let block_selected_ref = Rc::new(RefCell::new(BlockTexture::Air));
        building_world_view::add_bulding_world_view_panel(panel, player_data, block_selected_ref.clone());

        let block_selection_panel = panel.add_sub_panel();
        block_selection_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        /* 
        let mut slot_refs = Vec::new();
        for i in 0..9 {
            let var_slot = VarSlot::new(VarTypeKind::Game(GameVarTypeKind::Primitive(PrimitiveVarTypeKind::Block)));

            // Get the ref from the slot
            if let VarRef::Game(game_var) = &*var_slot.get_var_ref().borrow() {
                if let GameVarRef::Primitive(PrimitiveVarRef::Block(block_ref)) = game_var {
                    slot_refs.push(block_ref.clone());
                }
            }

            block_selection_panel.add_widget(var_slot.wrap_into_widget());
        }
        

        
        
        let block_selection_input = &mut self::get_block_hotbar_input_events(slot_refs, block_selected_ref);
        panel.add_events(block_selection_input);
        */



        panel.size();
    }

    return panel;
}