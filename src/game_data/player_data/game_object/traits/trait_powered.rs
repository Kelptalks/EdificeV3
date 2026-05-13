use std::f32::consts::E;

use crate::game_data::{World, game_event_manager::{self, event_manager::{Event, EventManager}}, player_data::game_object::{game_object_manager::{GameObject, GameObjectId}, traits::game_object_trait_manager::GameObjectTrait}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType}, tik_manager::game_time::{GameTime, GameTimeInterval}};

const DIRECTIONS: [[i32; 3]; 6] = [
    [ 1,  0,  0], // +X
    [-1,  0,  0], // -X
    [ 0,  1,  0], // +Y
    [ 0, -1,  0], // -Y
    [ 0,  0,  1], // +Z
    [ 0,  0, -1], // -Z
];

#[derive(Clone)]
pub struct PoweredTrait {
    pub cords: [i32; 3],
    pub object_id: GameObjectId, 

    pub search_for_links: bool,
    pub power_links: Vec<GameObjectId>,
    pub power_requests: Vec<(GameObjectId, u32)>,
    

    pub max_power: u32,
    pub power_stored: u32,
    pub power_demand: u32,

    pub power_consume_interval: GameTimeInterval,
    pub power_consume_per_interval: u32,
}

impl PoweredTrait {
    pub fn wrap_into_trait(self) -> GameObjectTrait {
        GameObjectTrait::Powered(self)
    }
    
    pub fn new(object_id: GameObjectId, cords: [i32; 3]) -> PoweredTrait {
        PoweredTrait {
            // Parent Object Data
            cords,
            object_id, 

            // Personal Data
            search_for_links: true,
            power_links: Vec::new(),
            power_requests: Vec::new(),

            max_power: 10000,
            power_stored: 0,
            power_demand: 5000,

            power_consume_interval: GameTimeInterval::Second,
            power_consume_per_interval: 0,
        }
    }

    pub fn power_surplus(&self) -> u32 {
        self.power_stored - self.power_demand
    }

    //=====================================
    // Tiking
    //=====================================

    pub fn tik(&mut self, game_time: &GameTime, world: &World, event_manager: &mut EventManager) -> bool {
        
        // Handle Power Link Checks
        if self.search_for_links {
            println!("Searching for links");
            self.power_links.clear();
            for relative_cords in DIRECTIONS {
                let cords_to_search = [
                    self.cords[0] + relative_cords[0],
                    self.cords[1] + relative_cords[1],
                    self.cords[2] + relative_cords[2],
                ];
                
                if let Some(object_id) = world.get_game_object(cords_to_search) {
                    object_id.add_trait_event(event_manager, PoweredTraitEvent::PowerLinkRequest(self.object_id));
                }
            }
            self.search_for_links = false;
        }

        // Power requests
        while let Some((object_id, amount_requested)) = self.power_requests.pop() {
            if self.power_surplus() < amount_requested {
                self.power_stored -= amount_requested;
            }
        }

        // Power Consumption
        if self.power_stored > 0 {
            if self.power_consume_interval.is_interval(game_time) {
                if self.power_stored > self.power_consume_per_interval {
                    self.power_stored -= self.power_consume_per_interval;
                }
                else {
                    self.power_stored = 0;
                    return false
                }
            }
        }
        else {
            return false
        }
        return true;
    }


    //=====================================
    // Section
    //=====================================

    pub fn get_widget(&self) -> WidgetType {
        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        // Power
        panel.add_text_display("Vision Range".to_string());
        let power_panel = panel.add_sub_panel();
        power_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        power_panel.add_text_display(
            format!(
                "Stored Power: {} / {}", 
                self.power_stored, 
                self.max_power
            )
        );

        power_panel.add_text_display(
            format!(
                "Power Consumption {} / {}", 
                self.power_consume_per_interval, 
                self.power_consume_interval.to_string()
            )
        );


        let mut link_display = "Links = (".to_string();
        power_panel.add_text_display(link_display);


        for link in &self.power_links {
            power_panel.add_text_display(link.to_str().to_string());
        }


        power_panel.add_text_display(")".to_string());
        

        panel.wrap_into_widget()
    }

}


#[derive(Clone)]
pub enum PoweredTraitEvent {
    PowerLinkRequest(GameObjectId), // Source Supplying 
    PowerRequest(GameObjectId, u32),
    PowerReceived(u32), // amount
}

impl PoweredTraitEvent {
    pub fn execute(self, power_trait: &mut PoweredTrait) {
        match self {
            PoweredTraitEvent::PowerLinkRequest(game_object_id) => {
                println!("Adding Power link");
                power_trait.power_links.push(game_object_id);
            },
            PoweredTraitEvent::PowerRequest(game_object_id, amount) => {
                power_trait.power_requests.push((game_object_id, amount));
            },
            PoweredTraitEvent::PowerReceived(amount) => {
                power_trait.power_stored += amount;
            },
        }
    }
}