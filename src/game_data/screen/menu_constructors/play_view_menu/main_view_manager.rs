use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{locations::location::WorldLocation, player_data::PlayerData}, screen::widget::prelude::PlayViewRendingConfig};

pub struct MainPlayViewManager {
    render_all_locations_toggle: Rc<RefCell<bool>>,
    all_locations: Rc<RefCell<Vec<Rc<RefCell<WorldLocation>>>>>,
}

impl MainPlayViewManager {
    pub fn new(player_data: &mut PlayerData) -> MainPlayViewManager {
        let main_play_config = MainPlayViewManager {
            render_all_locations_toggle: Rc::new(RefCell::new(false)),
            all_locations: player_data.get_mut_location_manager().get_locations_ref_vec().clone(),
        };
        return main_play_config;
    }

    pub fn update_rendering_config(&self, rendering_config: Rc<RefCell<PlayViewRendingConfig>>) {
        



    }


}