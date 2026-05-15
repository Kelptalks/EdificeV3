use crate::game_data::{World, game_event_manager::event_manager::EventManager, player_data::game_entity::{components::entity_components::{EntityComponent, EntityComponentEvent}, game_entity_manager::GameEntityId}, screen::widget::{panel::panel::{Panel, PanelAlignment, PanelOrientation}, widget::WidgetType, widget_calculations::TextSize}, tik_manager::game_time::{GameTime, GameTimeInterval}, types::UITextures};

const DIRECTIONS: [[i32; 3]; 6] = [
    [ 1,  0,  0],
    [-1,  0,  0],
    [ 0,  1,  0],
    [ 0, -1,  0],
    [ 0,  0,  1],
    [ 0,  0, -1],
];

#[derive(Clone)]
pub struct PoweredComponent {
    pub cords: [i32; 3],
    pub entity_id: GameEntityId,

    pub search_for_links: bool,
    pub power_link_requests: Vec<GameEntityId>,
    pub power_links: Vec<GameEntityId>,

    pub power_requests: Vec<(GameEntityId, u32)>,

    pub max_power: u32,
    pub power_stored: u32,

    pub power_demand: u32,
    pub power_throughput: u32,

    pub power_consume_interval: GameTimeInterval,
    pub power_consume_per_interval: u32,
}

impl PoweredComponent {
    pub fn wrap_into_component(self) -> EntityComponent {
        EntityComponent::Powered(self)
    }

    pub fn new(entity_id: GameEntityId, cords: [i32; 3]) -> PoweredComponent {
        PoweredComponent {
            cords,
            entity_id,

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
    // Ticking
    //=====================================

    pub fn search_for_links(&mut self, world: &World, event_manager: &mut EventManager) {
        self.power_links.clear();
        for relative_cords in DIRECTIONS {
            let cords_to_search = [
                self.cords[0] + relative_cords[0],
                self.cords[1] + relative_cords[1],
                self.cords[2] + relative_cords[2],
            ];

            if let Some(entity_id) = world.get_block_entity(cords_to_search) {
                if let Some(event) = entity_id.get_component_event(
                    PoweredComponentEvent::PowerLinkRequest(self.entity_id).wrap_into_component_event()
                ) {
                    event_manager.add_event(event);
                }
            }
        }
        self.search_for_links = false;
    }

    pub fn handle_power_link_requests(&mut self, event_manager: &mut EventManager) {
        while let Some(power_link_id) = self.power_link_requests.pop() {
            if let Some(event) = power_link_id.get_component_event(
                PoweredComponentEvent::PowerLinkAccept(self.entity_id).wrap_into_component_event()
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
        while let Some((entity_id, amount_requested)) = self.power_requests.pop() {
            if self.power_surplus() > amount_requested as i32 {
                if let Some(event) = entity_id.get_component_event(PoweredComponentEvent::PowerReceived(amount_requested).wrap_into_component_event()) {
                    event_manager.add_event(event);
                    self.power_stored -= amount_requested;
                }
            }
        }

        // Request Power
        for power_link in &self.power_links {
            if self.power_surplus() < 0 {
                let power_to_request = self.power_surplus().abs() as u32;
                if let Some(event) = power_link.get_component_event(
                    PoweredComponentEvent::PowerRequest(self.entity_id, power_to_request.min(self.power_throughput)).wrap_into_component_event()
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
    // Widget
    //=====================================

    pub fn get_widget(&self) -> WidgetType {
        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        panel.add_text_display("Power".to_string()).set_text_scale(TextSize::Medium);
        let power_panel = panel.add_sub_panel();
        power_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        power_panel.add_text_display(
            format!("Stored Power: {} / {}", self.power_stored, self.max_power)
        );

        power_panel.add_text_display(
            format!("Power Consumption {} / {}", self.power_consume_per_interval, self.power_consume_interval.to_string())
        );

        power_panel.add_text_display("Links = (".to_string());
        for link in &self.power_links {
            power_panel.add_text_display(link.to_string());
        }
        power_panel.add_text_display(")".to_string());

        let demand_sub_panel = power_panel.add_sub_panel();

        demand_sub_panel.add_text_display(format!("Demand: {}", self.power_demand));

        let button = demand_sub_panel.add_button();
        button.add_texture(UITextures::ModIcon.wrap_into_texture());
        button.set_text("Demand".to_string());
        if let Some(event) = self.entity_id.get_component_event(PoweredComponentEvent::ModPowerDemand(100).wrap_into_component_event()) {
            button.add_left_click_event(event);
        }
        if let Some(event) = self.entity_id.get_component_event(PoweredComponentEvent::ModPowerDemand(-100).wrap_into_component_event()) {
            button.add_right_click_event(event);
        }

        panel.wrap_into_widget()
    }
}


#[derive(Clone)]
pub enum PoweredComponentEvent {
    // Linking
    PowerLinkRequest(GameEntityId),
    PowerLinkAccept(GameEntityId),

    PowerRequest(GameEntityId, u32),
    PowerReceived(u32),

    ModPowerDemand(i32),
}

impl PoweredComponentEvent {
    pub fn wrap_into_component_event(self) -> EntityComponentEvent {
        EntityComponentEvent::Powered(self)
    }

    pub fn execute(self, powered: &mut PoweredComponent) {
        match self {
            PoweredComponentEvent::PowerLinkRequest(entity_id) => {
                powered.power_link_requests.push(entity_id);
            },
            PoweredComponentEvent::PowerLinkAccept(entity_id) => {
                powered.power_links.push(entity_id);
            },
            PoweredComponentEvent::PowerRequest(entity_id, amount) => {
                powered.power_requests.push((entity_id, amount));
            },
            PoweredComponentEvent::PowerReceived(amount) => {
                powered.power_stored += amount;
            },
            PoweredComponentEvent::ModPowerDemand(num) => {
                let new_demand = powered.power_demand as i32 + num;
                if new_demand > 0 {
                    powered.power_demand = new_demand as u32;
                }
                else {
                    powered.power_demand = 0;
                }
            }
        }
    }
}
