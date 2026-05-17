use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::widget_event_manager::prim_events::i32_event::I32Event, player_data::{drone_script::var::{game_vars::primitive_game_var::{PrimitiveGameVarType, PrimitiveGameVarTypeKind}, var::Var, var_properties::{PropKey, VarPropModRequest}, var_type::VarType}, player_data::PlayerData}, screen::widget::{drone_programming::var_slot::{var_prop_widgets::var_prop_widgets::VarPropVal, var_slot}, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::VarSlot, text::header::TextDisplay, widget::{Widget, WidgetType}}, types::drone_item::DroneItem};

pub struct InvintoryDisplayPropWidget {
    mutable: bool,

    item_to_mod_ref: Rc<RefCell<VarType>>,
    item_amount_to_mod: Rc<RefCell<i32>>,
    
    key: PropKey,
    panel: Panel,
}

impl InvintoryDisplayPropWidget {
    pub fn new(key: PropKey, mutable: bool) -> InvintoryDisplayPropWidget {
        let panel = Panel::new_blank();

        

        InvintoryDisplayPropWidget {
            mutable: mutable,

            item_to_mod_ref: Rc::new(RefCell::new(PrimitiveGameVarType::DroneItem(DroneItem::Ash).wrap_into_var_type())),
            item_amount_to_mod: Rc::new(RefCell::new(0)),
            

            key: key,
            panel,
        }
    }


    fn get_item_panel(item: DroneItem) -> WidgetType {
        let mut panel = Panel::new_blank();

        // Add item var slot
        let var_type = PrimitiveGameVarType::DroneItem(item).wrap_into_var_type();
        let var_slot = var_slot::VarSlot::new_with_type(var_type);
        panel.add_widget(var_slot.wrap_into_widget());

        return panel.wrap_into_widget();
    }

    fn get_item_slots_panel(item: DroneItem, quantity: i32,) -> WidgetType {
        let mut panel = Panel::new_blank();

        // Add item var slot
        let var_type = PrimitiveGameVarType::DroneItem(item).wrap_into_var_type();
        let mut var_slot = var_slot::VarSlot::new_with_type(var_type);
        var_slot.set_dragging_properties(true, false, false);
        panel.add_widget(var_slot.wrap_into_widget());

        // add quantity text display
        let text_display = TextDisplay::new(quantity.to_string());
        panel.add_widget(text_display.wrap_into_widget());

        return panel.wrap_into_widget();

    }


    pub fn update_with_var(&mut self, var: &Var) -> Vec<VarPropModRequest> {
        let mut prop_requests = Vec::new();
        
        // Create new panel
        self.panel = Panel::new_blank();
        self.panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        self.panel.add_text_display(self.key.to_name());
        

        let slots_per_row = 3;
        let mut current_row_pos = 0;
        
        if let Some(slots) = var.as_inventory() {


            // Item display
            let item_display_sub_panel = self.panel.add_sub_panel();
            item_display_sub_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
            let mut row_sub_panel = item_display_sub_panel.add_sub_panel();

            for slot in slots {
                // Create next row
                if current_row_pos == slots_per_row {
                    row_sub_panel = item_display_sub_panel.add_sub_panel();
                    current_row_pos = 0;
                }
                
                if let Some(item) = slot.get_item() {
                    row_sub_panel.add_widget(Self::get_item_slots_panel(item, slot.get_quantity()));
                }
                else {
                    row_sub_panel.add_widget(Self::get_item_slots_panel(DroneItem::Null, slot.get_quantity()));
                }

                current_row_pos += 1;
            }

            // Item Mod
            if self.mutable {

                
                let item_mod_sub_panel = self.panel.add_sub_panel();
                let mut var_slot = VarSlot::new_with_kind(PrimitiveGameVarTypeKind::DroneItem.wrap_into_var_kind());
                item_mod_sub_panel.add_widget(var_slot.wrap_into_widget());

                let mod_button = item_mod_sub_panel.add_button();
                mod_button.add_left_click_event(I32Event::mod_i32(self.item_amount_to_mod.clone(), 1).wrap_into_event());
                mod_button.add_right_click_event(I32Event::mod_i32(self.item_amount_to_mod.clone(), -1).wrap_into_event());
                mod_button.set_icon(crate::game_data::types::UITextures::ModIcon);

                
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
    fn get_widget_properties(&self) -> &crate::game_data::screen::widget::widget_properties::WidgetProperties {
        self.panel.get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut crate::game_data::screen::widget::widget_properties::WidgetProperties {
        self.panel.get_mut_widget_properties()
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
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &PlayerData,
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }
}