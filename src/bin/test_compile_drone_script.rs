use EdificeV3::game_data::player_data::{
    drone_programming::{function::function::Function, script_element::{self, ScriptElement}},
    drones::drone_actions::{
        advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, getter_actions::getter_actions::DroneGetterAction, prim_actions::{drone_invintory_actions::DroneInventoryAction, drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}
    },
};

fn main() {
    

    let mut function = Function::new_blank();
    let mut sub_function = Function::new_from_drone_action(DroneAction::GetterAction(DroneGetterAction::IsBusy));





    let return_values = sub_function.get_return();


    for (return_value, script_element) in return_values {
        if let ScriptElement::Function(script_element) = script_element{
            let test_function = Function::new_from_drone_action(DroneAction::PrimAction(DronePrimAction::DroneWorldAction(DroneWorldAction::MineBlock([0, 0, 1]))));
            script_element.borrow_mut().add_script_element(0, test_function.to_script_element());
        }
    }

    function.add_script_element(0, sub_function.to_script_element());

    function.add_script_element(0, ScriptElement::Action(DroneAction::AdvancedAction(DroneAdvancedAction::PathToLocation(None))));

    let mut index = 0;
    let elements = function.compile(0);
    for (indent, compiled_element) in elements {
        let indent = "-".repeat(indent);

        println!("{} | {}{}",index, indent, compiled_element.get_name());

        index += 1;
    }
}
