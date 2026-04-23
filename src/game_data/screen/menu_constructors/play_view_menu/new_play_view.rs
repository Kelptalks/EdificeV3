use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{prelude::{Event, GameEvent, InputEvent, UsizeEvent}, render_event_manager::render_event_manager::RenderEvent}, player_data::{drone_script::var::{game_vars::dynamic_var::DynamicVarType, var::Var, var_type::VarType}, locations::location::WorldLocation, player_data::PlayerData}, screen::{ScreenData, menu_constructors::play_view_menu::{manager_panel, selection_panel, var_ref_hot_bar, world_view}, screen_data::CurrentMenu, widget::{drone_programming::scripting_elements::scripting_panel::ScriptingPanel, panel::{panel::{PanelAlignment, PanelOrientation}, panel_background::BackgroundType, panel_color::PanelColor}, prelude::TabPanel, tab_panel::var_tab_panel::VarTabPanel, widget::WidgetType, world_rendering::rendering_config::play_world_view_config::PlayViewRenderingConfig}}};

pub enum PlayViewMode {
    Main = 0,
    Location = 1,
    BluePrint = 2,
    Drone = 3,
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


    pub selected_var: Var,
    pub play_view_mode: Rc<RefCell<usize>>,
}

impl RefManager {
    pub fn new(player_data: &mut PlayerData) -> RefManager {
        let rendering_config = PlayViewRenderingConfig::new(player_data);
        let cursor_location =  rendering_config.borrow().get_cursor_location_ref().clone();
        let show_locations_bool = rendering_config.borrow().get_should_render_all_locations_ref().clone();

        let mut selected_var = Var::new_blank();
        selected_var.set_name("selected_var".to_string());


        rendering_config.borrow_mut().set_focused_var(Some(selected_var.clone()));
        rendering_config.borrow_mut().add_var_to_render(selected_var.clone());

        RefManager {
            play_view_rendering_config: rendering_config.clone(),

            cursor_location: cursor_location,

            show_drones_toggle: Rc::new(RefCell::new(false)),
            show_locations_toggle: show_locations_bool,
            render_only_selected_location: Rc::new(RefCell::new(false)),

            selected_var: selected_var,
            
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

    pub fn build_panel(&mut self, screen_data: &ScreenData, player_data: &mut PlayerData) -> WidgetType {
        let mut panel = WidgetType::new_panel(screen_data.get_viewport_uv(), [0.0; 4]);

        if let WidgetType::Panel(panel) = &mut panel {

            panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::TopLeft);
            panel.set_color(PanelColor::Clear);
            panel.set_new_background(BackgroundType::Scrolling(crate::game_data::types::UITextures::VoidBackground));


            // Play view 
            panel.add_widget(var_ref_hot_bar::get_widget(&mut self.ref_manager));
            panel.add_widget(world_view::get_widget(&mut self.ref_manager));


            // Manager Panel
            let mut manager_tab_panel = TabPanel::new(&Rc::new(RefCell::new(0)));
            

            // Var_Tab Panel Sub Widget
            let mut var_tab_panel = VarTabPanel::new();
            var_tab_panel.set_prefered_scale([0.50, 1.9]);
            let var_tab_panel_button = manager_tab_panel.add_panel(WidgetType::VarTabPanel(var_tab_panel));
            var_tab_panel_button.set_icon(crate::game_data::types::UITextures::AnyVarIcon);


            // Scripting Panel
            // let mut scipting_panel = ScriptingPanel::new();
            // manager_tab_panel.add_panel(scipting_panel.wrap_into_widget());

            panel.add_widget(manager_tab_panel.wrap_into_widget());


            // Selection Panel
            panel.add_widget(selection_panel::selection_panel::get_var_managment_panel_widget(&mut self.ref_manager));


            

            panel.size();
        }
        return panel;
    }
}

