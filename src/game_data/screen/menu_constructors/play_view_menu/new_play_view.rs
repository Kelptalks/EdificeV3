use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, PrimEvent, UsizeEvent}, locations::world_area::WorldArea, player_data::{drone_programming::var::{game_vars::dynamic_var::DynamicVar, var_type::Var}, locations::location::WorldLocation, player_data::PlayerData}, screen::{ScreenData, menu_constructors::play_view_menu::{managmenet_panel, var_ref_hot_bar, world_hot_bar, world_view}, widget::{panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, widget::WidgetType, world_rendering::rendering_config::play_world_view_config::PlayViewRenderingConfig}}};

pub enum PlayViewMode {
    Main = 0,
    Location = 1,
    BluePrint = 2,
}

impl PlayViewMode {
    pub fn to_usize(self) -> usize {
        self as usize
    }

    pub fn to_tab_panel_event(self, usize: &Rc<RefCell<usize>>) -> Event {
        return UsizeEvent::SetUsize(usize.clone(), self.to_usize()).wrap_into_event();
    }
}


pub struct RefManager {
    pub cursor_location: Rc<RefCell<WorldLocation>>,
    pub play_view_rendering_config: Rc<RefCell<PlayViewRenderingConfig>>,

    pub show_drones_toggle: Rc<RefCell<bool>>,
    pub show_locations_toggle: Rc<RefCell<bool>>,
    pub render_only_selected_location: Rc<RefCell<bool>>,


    pub selected_var: Rc<RefCell<Var>>,
    pub play_view_mode: Rc<RefCell<usize>>,
}

impl RefManager {
    pub fn new(player_data: &mut PlayerData) -> RefManager {
        let rendering_config = PlayViewRenderingConfig::new(player_data);
        let cursor_location =  rendering_config.borrow().get_cursor_location_ref().clone();
        let show_locations_bool = rendering_config.borrow().get_should_render_all_locations_ref().clone();

        RefManager {
            play_view_rendering_config: rendering_config.clone(),

            cursor_location: cursor_location,

            show_drones_toggle: Rc::new(RefCell::new(false)),
            show_locations_toggle: show_locations_bool,
            render_only_selected_location: Rc::new(RefCell::new(false)),

            selected_var: Rc::new(RefCell::new(DynamicVar::Location(None).wrap_into_var())),
            
            play_view_mode: Rc::new(RefCell::new(0)),
        }
    }
}

pub struct PlayViewConstructionManager {
    ref_manager: RefManager,
}

impl PlayViewConstructionManager {
    pub fn new(player_data: &mut PlayerData) -> PlayViewConstructionManager {
        PlayViewConstructionManager {
            ref_manager: RefManager::new(player_data)
        }
    }

    pub fn build_panel(&mut self, screen_data: &ScreenData) -> WidgetType {
        let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);

        if let WidgetType::Panel(panel) = &mut panel {
            panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::TopLeft);
            panel.set_color(PanelColor::Clear);
            panel.set_new_background(BackgroundType::Scrolling(crate::game_data::types::UITextures::VoidBackground));


            panel.add_widget(var_ref_hot_bar::get_widget());
            panel.add_widget(world_view::get_widget(&mut self.ref_manager));
            panel.add_widget(managmenet_panel::get_widget(&mut self.ref_manager));

            panel.size();
        }
        return panel;
    }
}

