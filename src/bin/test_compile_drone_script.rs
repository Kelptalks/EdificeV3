use EdificeV3::game_data::player_data::{
    drone_script::{function::function::Function, script_element::{self, ScriptElement}},
    drones::drone_actions::{
        advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, getter_actions::getter_actions::DroneGetterAction, prim_actions::{drone_invintory_actions::DroneInventoryAction, drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}
    },
};

fn main() {
    

    let mut function = Function::new_blank();
    
    let mut sub_function = Function::new_from_drone_action(DroneAction::GetterAction(DroneGetterAction::IsBusy));
    
    function.add_script_element(0, sub_function.to_script_element());



    function.get_function_at_index(0);
}
