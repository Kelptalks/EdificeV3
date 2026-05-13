use std::f32::consts::E;

use crate::game_data::{World, game_event_manager::{self, event_manager::{Event, EventManager}}, player_data::game_object::{game_object_manager::{GameObject, GameObjectId}, traits::game_object_trait_manager::GameObjectTrait}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType, widget_calculations::TextSize}, tik_manager::game_time::{GameTime, GameTimeInterval}, types::UITextures};

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
    pub power_link_requests: Vec<GameObjectId>,
    pub power_links: Vec<GameObjectId>,

    pub power_requests: Vec<(GameObjectId, u32)>,
    

    pub max_power: u32,
    pub power_stored: u32,

    pub power_demand: u32,
    pub power_throughput: u32,

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
            power_link_requests: Vec::new(),
            power_links: Vec::new(),

            power_requests: Vec::new(),

            max_power: 10000,
            power_stored: 0,
            power_demand: 100,
            power_throughput: 100,

            power_consume_interval: GameTimeInterval::Second,
            power_consume_per_interval: 0,
        }
    }

    pub fn power_surplus(&self) -> i32 {
        self.power_stored as i32 - self.power_demand as i32
    }

    //=====================================
    // Tiking
    //=====================================

    pub fn search_for_links(&mut self, world: &World, event_manager: &mut EventManager) {
        self.power_links.clear();
        for relative_cords in DIRECTIONS {
            let cords_to_search = [
                self.cords[0] + relative_cords[0],
                self.cords[1] + relative_cords[1],
                self.cords[2] + relative_cords[2],
            ];
            
            if let Some(object_id) = world.get_game_object(cords_to_search) {
                if let Some(event) = object_id.get_trait_event(
                    PoweredTraitEvent::PowerLinkRequest(self.object_id)
                ) {
                    event_manager.add_event(event);
                }
            }
        }
        self.search_for_links = false;
    }


    // Accept Power link requests and send link accept event
    pub fn handle_power_link_requests(&mut self, event_manager: &mut EventManager) {
        while let Some(power_link_id) = self.power_link_requests.pop() {
            if let Some(event) = power_link_id.get_trait_event(
                PoweredTraitEvent::PowerLinkAccept(self.object_id)
            ) {
                event_manager.add_event(event);
            }

            self.power_links.push(power_link_id);
        }
    }

    pub fn tik(&mut self, game_time: &GameTime, world: &World, event_manager: &mut EventManager) -> bool {
        
        // Power Links
        if self.search_for_links {
            self.search_for_links(world, event_manager);
        }
        self.handle_power_link_requests(event_manager);


        // Power Requests
        while let Some((object_id, amount_requested)) = self.power_requests.pop() {
            if self.power_surplus() > amount_requested as i32 {
                if let Some(event) = object_id.get_trait_event(PoweredTraitEvent::PowerReceived(amount_requested)) {
                    event_manager.add_event(event);
                    self.power_stored -= amount_requested;
                }
            }
        }

        // Request Power
        for power_link in &self.power_links {
            if self.power_surplus() < 0 {
                let power_to_request = self.power_surplus().abs() as u32;
                if let Some(event) = power_link.get_trait_event(
                    PoweredTraitEvent::PowerRequest(self.object_id, power_to_request.max(self.power_throughput))
                ) {
                    event_manager.add_event(event);
                }
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
        panel.add_text_display("Power".to_string()).set_text_scale(TextSize::Medium);
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


        power_panel.add_text_display("Links = (".to_string());
        for link in &self.power_links {
            power_panel.add_text_display(link.to_string());
        }
        power_panel.add_text_display(")".to_string());
        

        let damand_sub_panel = power_panel.add_sub_panel();

        damand_sub_panel.add_text_display(format!("Damand: {}", self.power_demand));

        // Mod button
        let button = damand_sub_panel.add_button();
        button.add_texture(UITextures::ModIcon.wrap_into_texture());
        button.set_text("Damand".to_string());
        if let Some(event) = self.object_id.get_trait_event(PoweredTraitEvent::ModPowerDamand(100)) {
            button.add_left_click_event(
                event
            );        
        }
        if let Some(event) = self.object_id.get_trait_event(PoweredTraitEvent::ModPowerDamand(-100)) {
            button.add_right_click_event(
                event
            );        
        }
        


        panel.wrap_into_widget()
    }

}


#[derive(Clone)]
pub enum PoweredTraitEvent {
    // Linking
    PowerLinkRequest(GameObjectId), // Requestors Id 
    PowerLinkAccept(GameObjectId),  // Acceptors Id


    PowerRequest(GameObjectId, u32),
    PowerReceived(u32), // amount

    ModPowerDamand(i32) // Mod amount
}

impl PoweredTraitEvent {
    pub fn execute(self, power_trait: &mut PoweredTrait) {
        match self {
            PoweredTraitEvent::PowerLinkRequest(game_object_id) => {
                power_trait.power_link_requests.push(game_object_id);
            },
            PoweredTraitEvent::PowerLinkAccept(game_object_id) => {
                power_trait.power_links.push(game_object_id);
            }
            PoweredTraitEvent::PowerRequest(game_object_id, amount) => {
                power_trait.power_requests.push((game_object_id, amount));
            },
            PoweredTraitEvent::PowerReceived(amount) => {
                power_trait.power_stored += amount;
            },

            PoweredTraitEvent::ModPowerDamand(num) => {
                let new_damand = power_trait.power_demand as i32 + num;
                if new_damand > 0 {
                    power_trait.power_demand = new_damand as u32;
                }
                else {
                    power_trait.power_demand = 0;
                }
            }
        }
    }
}