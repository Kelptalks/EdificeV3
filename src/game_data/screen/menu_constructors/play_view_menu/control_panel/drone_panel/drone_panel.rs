use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, LocationEvent, PlayerDataEvent}, locations::world_area::WorldArea, player_data::{drone_programming::var::{self, game_vars::game_var_type::{DynamicVarTypeKind, GameVarTypeKind}, var_type::{VarTypeKind}}, locations::location::WorldLocation, player_data::PlayerData}, screen::{menu_constructors::play_view_menu::control_panel::drone_panel::drone_world_view, widget::{drone_programming::vars::var_slot::VarSlot, panel::panel::{PanelAlignment, PanelOrientation}, text::header::TextDisplay, widget::WidgetType, widget_calculations::TextSize}}};

/*
pub fn lock_camera_to_drone_event(location_ref: &Rc<RefCell<WorldLocation>>, var_ref: &Rc<RefCell<VarRef>>) -> Vec<Event> {
    let mut events = Vec::new();

  
    events.push(PlayerDataEvent::LocationEvent(location_ref.clone(), LocationEvent::SetAreaWithVarRef(var_ref.clone())).wrap_into_event());



    return events;
}
*/

pub fn get_drone_panel(player_data: &mut PlayerData) -> WidgetType {
    let mut panel = WidgetType::new_panel([0.0; 4], [0.0; 4]);

    if let WidgetType::Panel(panel) = &mut panel {
        panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);

        // Header
        let scroll_panel = panel.add_scroll_panel();
        scroll_panel.set_prefered_scale(0.8);

        let mut text_display = TextDisplay::new("Controls".to_string());
        text_display.set_text_scale(TextSize::Small);
        scroll_panel.add_widget(WidgetType::TextDisplay(text_display));


        scroll_panel.set_prefered_scale(0.5);

        /*
        let var_slot = VarSlot::new(VarTypeKind::Game(GameVarTypeKind::Dynamic(DynamicVarTypeKind::Drone)));
        let dynamic_var_ref = var_slot.get_var_ref();
        
         
        


        let location_ref = 
            Rc::new(RefCell::new(WorldLocation::new("Drone_Spectating".to_string(), WorldArea::new_blank(), 404)));


        panel.add_events(&mut lock_camera_to_drone_event(&location_ref, dynamic_var_ref));   

        panel.add_widget(var_slot.wrap_into_widget());
        drone_world_view::add_drone_world_view_panel(panel, player_data, &location_ref);
        */

        panel.size();
    }

    return panel;
}