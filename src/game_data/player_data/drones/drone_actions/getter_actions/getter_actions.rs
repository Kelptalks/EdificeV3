use crate::game_data::{player_data::{drone_script::var::{game_vars::primitive_var::PrimitiveGameVarTypeKind, prim_vars::prim_var_type::{PrimitiveVarKind, PrimitiveVarType}, var::Var, var_type::{VarKind, VarType}}, drones::{drone::Drone, drone_actions::drone_actions::{DroneAction, DroneActionError}}}, texture_manager::texture::Texture, types::UITextures};

#[derive(Clone)]
pub enum DroneGetterAction {
    IsBusy,
    
    GetFuel,
    GetHealth,

    GetCords,
}

impl DroneGetterAction {
    pub fn wrap_into_action(self) -> DroneAction {
        DroneAction::GetterAction(self)
    }
    
    pub fn execute(&self, drone: &mut Drone) -> Var {
        match self {
            DroneGetterAction::IsBusy => DroneActionError::Ok.wrap_into_var_type().create_var(),
            DroneGetterAction::GetFuel => DroneActionError::Ok.wrap_into_var_type().create_var(),
            DroneGetterAction::GetHealth => DroneActionError::Ok.wrap_into_var_type().create_var(),
            DroneGetterAction::GetCords => DroneActionError::Ok.wrap_into_var_type().create_var(),
        }
    }

    pub fn get_all_actions() -> Vec<DroneAction> {
        let mut all_actions = Vec::new();

        all_actions.push(DroneGetterAction::IsBusy.wrap_into_action());
        all_actions.push(DroneGetterAction::GetFuel.wrap_into_action());
        all_actions.push(DroneGetterAction::GetHealth.wrap_into_action());
        all_actions.push(DroneGetterAction::GetCords.wrap_into_action());

        all_actions
    }

    //=====================================
    // Identity
    //=====================================

    pub fn get_name(&self) -> String {
        match self {
            DroneGetterAction::IsBusy => "IsBusy".to_string(),
            DroneGetterAction::GetFuel => "GetFuel".to_string(),
            DroneGetterAction::GetHealth => "GetHealth".to_string(),
            DroneGetterAction::GetCords => "GetCords".to_string(),
        }
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            DroneGetterAction::IsBusy => {
                UITextures::DroneActionIsBusyIcon.wrap_into_texture()
            },
            DroneGetterAction::GetFuel => {
                UITextures::DroneActionFuelIcon.wrap_into_texture()
            },
            DroneGetterAction::GetHealth => {
                UITextures::DroneActionHealthIcon.wrap_into_texture()
            },
            DroneGetterAction::GetCords => {
                UITextures::CordsIcon.wrap_into_texture()
            },
        }
    }

    //=====================================
    // Function Managment
    //=====================================

    pub fn get_return_var(&self) -> Var {
        match self {
            DroneGetterAction::IsBusy => {
                let mut var = Var::new_blank_with_kind(PrimitiveVarKind::Bool.wrap_into_var_kind());
                var.set_name(self.get_name());
                var
            }
            DroneGetterAction::GetFuel => {
                let mut var = Var::new_blank_with_kind(PrimitiveVarKind::Num.wrap_into_var_kind());
                var.set_name(self.get_name());
                var
            },
            DroneGetterAction::GetHealth => {
                let mut var = Var::new_blank_with_kind(PrimitiveVarKind::Num.wrap_into_var_kind());
                var.set_name(self.get_name());
                var
            },
            DroneGetterAction::GetCords => {
                let mut var = Var::new_blank_with_kind(PrimitiveGameVarTypeKind::Cords.wrap_into_var_kind());
                var.set_name(self.get_name());
                var
            }
        }
        

    }

    

    
}