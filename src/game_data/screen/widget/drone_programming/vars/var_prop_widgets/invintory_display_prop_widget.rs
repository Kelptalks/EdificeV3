use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::game_data::{player_data::drone_programming::var::{game_vars::{game_var_type::GameVarTypeKind, primitive_var::{PrimitiveVar, PrimitiveVarTypeKind}}, var_properties::{PropKey, PropValue, VarPropModRequest}, var_type::VarTypeKind}, screen::{ui_elements::panel, widget::{drone_programming::vars::{var_prop_widgets::var_prop_widgets::VarPropVal, var_slot}, panel::panel::{Panel, PanelAlignment, PanelOrientation}, scroll_panel::scroll_panel::ScrollPanel, selection_panel::selection_panel::SelectionPanel, text::header::TextDisplay, widget::{Widget, WidgetType}}}, types::drone_item::DroneItem};

pub struct InvintoryDisplayPropWidget {
    mutable: bool,

    key: PropKey,
    panel: Panel,
}

impl InvintoryDisplayPropWidget {
    pub fn new(key: PropKey, mutable: bool) -> InvintoryDisplayPropWidget {
        let panel = Panel::new_blank();

        

        InvintoryDisplayPropWidget {
            mutable: mutable,
            key: key,
            panel,
        }
    }


    fn get_item_panel(item: DroneItem) -> WidgetType {
        let mut panel = Panel::new_blank();

        // Add item var slot
        let var = Rc::new(RefCell::new(PrimitiveVar::DroneItem(item).wrap_into_var()));
        let var_slot = var_slot::VarSlot::new(&var);
        panel.add_widget(var_slot.wrap_into_widget());

        return panel.wrap_into_widget();
    }

    fn get_item_slots_panel(item: DroneItem, quantity: i32,) -> WidgetType {
        let mut panel = Panel::new_blank();

        // Add item var slot
        let var = Rc::new(RefCell::new(PrimitiveVar::DroneItem(item).wrap_into_var()));
        let mut var_slot = var_slot::VarSlot::new(&var);
        var_slot.set_dragging_properties(true, false, false);
        panel.add_widget(var_slot.wrap_into_widget());

        // add quantity text display
        let text_display = TextDisplay::new(quantity.to_string());
        panel.add_widget(text_display.wrap_into_widget());

        return panel.wrap_into_widget();

    }

    //pub fn wrap_into_widget() -> WidgetType {
        pub fn update_with_val(&mut self, val: PropValue) -> Vec<VarPropModRequest> {
        let mut prop_requests = Vec::new();
        
        // Create new panel
        self.panel = Panel::new_blank();
        self.panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        self.panel.add_text_display(self.key.to_name());
        

        let slots_per_row = 3;
        let mut current_row_pos = 0;
        let mut sub_panel = self.panel.add_sub_panel();

        if let PropValue::Inventory(slots) = val {
            for slot in slots {
                
                // Create next row
                if current_row_pos == slots_per_row {
                    sub_panel = self.panel.add_sub_panel();
                    current_row_pos = 0;
                }
                
                if let Some(item) = slot.get_item() {
                    sub_panel.add_widget(Self::get_item_slots_panel(item, slot.get_quantity()));
                }
                else {
                    sub_panel.add_widget(Self::get_item_slots_panel(DroneItem::Null, slot.get_quantity()));
                }
                
                current_row_pos += 1;

            }
        }
        else if let PropValue::ItemVec(items) = val {
            for item in items {
                self.panel.add_widget(Self::get_item_panel(item));
            }
        }
        self.panel.size();

        return prop_requests;
    }
    
    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::VarPropValWidget(VarPropVal::Invintory(self))
    }

    pub fn get_root_mut_panel(&mut self) -> &mut Panel {
        return &mut self.panel;
    }

    pub fn get_root_panel(&self) -> &Panel {
        return &self.panel;
    }




}


impl Widget for InvintoryDisplayPropWidget {
    fn get_pos(&self) -> [f32; 4] {
        self.panel.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.panel.get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.panel.get_preffered_scale()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.panel.set_buffers(pos)
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.panel.set_parent_pos(pos);
    }

    fn size(&mut self) {
        self.panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager);
    }
}