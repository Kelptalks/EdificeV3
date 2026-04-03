use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{drone_programming::var::game_vars::primitive_var::PrimitiveVar, player_data::PlayerData}, screen::{ScreenData, menu_constructors::play_view_menu::{new_play_view::RefManager, selection_panel::var_creation_panel}, widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, TabPanel, VarSlot}, selection_panel::selection_panel::SelectionPanel, widget::WidgetType, widget_calculations::TextSize}}, types::{BlockTexture, UITextures, drone_item::DroneItem}};

fn get_block_selection_panel() -> WidgetType {
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
                    let var_ref = PrimitiveVar::construct_block_var_ref(block_type);
                    let var_slot = VarSlot::new(&var_ref);
                    row_panel.add_widget(var_slot.wrap_into_widget());
                    
                }
            }
            scroll_panel.add_widget(block_selection_panel);
        }

        scroll_panel.set_prefered_scale(0.5);
        panel.size();
    }

    return panel;
}

fn get_item_selection_panel() -> WidgetType {
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
                    let var_ref = PrimitiveVar::construct_item_var_ref(item_type);
                    let var_slot = VarSlot::new(&var_ref);
                    row_panel.add_widget(var_slot.wrap_into_widget());
                }
            }
            scroll_panel.add_widget(item_selection_panel);
        }

        scroll_panel.set_prefered_scale(0.5);
        panel.size();
    }

    return panel;
}



pub fn get_var_managment_panel_widget(ref_manger: &mut RefManager) -> WidgetType {
    
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);
    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        let current_panel_index = Rc::new(RefCell::new(0));
        let mut selection_tab_panel = TabPanel::new(&current_panel_index);

        // Add Block selection
        let button = selection_tab_panel.add_panel(get_block_selection_panel());
        button.set_text("Blocks".to_string());
        button.set_block(crate::game_data::types::BlockTexture::Grass);

        // Add Item selection
        let button = selection_tab_panel.add_panel(get_item_selection_panel());
        button.set_text("Items".to_string());
        button.set_icon(UITextures::ScallingIconMidCenter);


        panel.add_widget(selection_tab_panel.wrap_into_widget());

        panel.add_widget(var_creation_panel::get_var_create_widget());

        panel.size();

    }

    return panel;
}