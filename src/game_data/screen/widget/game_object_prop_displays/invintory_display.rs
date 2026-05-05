use miniquad::StencilOp;

use crate::game_data::{player_data::{drone_script::var::{game_vars::game_var_type::{PrimitiveGameVarType, PrimitiveGameVarTypeKind}, var::Var, var_type::VarKind}, game_object::GameObjectType}, screen::widget::{drone_programming::var_slot::var_slot::VarSlot, panel::panel::Panel, widget::Widget, widget_properties::WidgetProperties}};

pub struct InvintoryDisplayWidget {
    panel: Panel,

    game_object: GameObjectType

}

impl InvintoryDisplayWidget {
    pub fn new(parents_props: &WidgetProperties, object_id: GameObjectType) -> InvintoryDisplayWidget {
        let mut panel = Panel::new_with_parent_props(parents_props);
        
        InvintoryDisplayWidget {
            panel,

            game_object: object_id
        }
    }
}

impl Widget for InvintoryDisplayWidget {
    fn get_widget_properties(&self) -> &WidgetProperties {
        self.panel.get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        self.panel.get_mut_widget_properties()
    }

    fn size(&mut self) {
        self.panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        player_data: &crate::game_data::player_data::player_data::PlayerData,
    ) {

        self.panel = Panel::new_blank();

        // Extract the invintory from the game object
        let mut inventory = None;
        match self.game_object {
            GameObjectType::Drone(drone_id) => {
                let drone = player_data.get_drone_manager().clone_drone_with_id(drone_id);
                if let Some(drone) = drone {
                    inventory = Some(drone.get_inventory().clone());
                }
            },
            _ => {

            }
        }


        if let Some(inventory) = inventory {
            for slot in inventory.get_slots() {
                let item = slot.get_item();
                let quantity = slot.get_quantity();

                if let Some(item) = item {

                    let var = Var::new_with_var_type(PrimitiveGameVarType::DroneItem(item).wrap_into_var_type()); 
                    let var_slot = VarSlot::new_with_var(var);
                
                    self.panel.add_widget(var_slot.wrap_into_widget());
                }
            }
        }
        
        self.panel.size();
        self.panel.render(texture_manager, screen_data, game_event_manager, player_data);
    }
}