use crate::game_data::{player_data::drone_programming::var::var_properties::PropKey, screen::widget::{panel::panel::Panel, widget::WidgetType}};

pub struct InvintoryDisplayPropWidget {
    mutable: bool,


    key: PropKey,
    panel: Panel,
}

impl InvintoryDisplayPropWidget {
    pub fn new(key: PropKey, mutable: bool) -> InvintoryDisplayPropWidget {
        let mut panel = Panel::new_blank();

        

        InvintoryDisplayPropWidget {
            mutable: mutable,
            key: key,
            panel: panel,
        }
    }


    //pub fn wrap_into_widget() -> WidgetType {

    




}


