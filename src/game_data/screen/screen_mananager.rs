use std::sync::{Arc, RwLock};

use miniquad::{window, GlContext, KeyCode, KeyMods, MouseButton};

use crate::game_data::{TextureManager, World, game_event_manager::{game_event_manager::EventManager, render_event_manager::render_event_manager::RenderEvent}, player_data::player_data::PlayerData, screen::{ScreenData, input_data::Input, screen_data::CurrentMenu, screen_task_manager::rendering_task_manager::RenderingTaskManager, widget::{widget::{Widget, WidgetType}, window_manager::widget_window_manager::WidgetWindowManager}}, tik_manager::tik_manager::TikManager, world_task_manager::world_task_manager::WorldTaskManager};

pub struct ScreenManager {
    screen_data: ScreenData,
    menu_panels: Vec<WidgetType>,
}

impl ScreenManager {
    pub fn new() -> Self {
        ScreenManager {
            screen_data: ScreenData::new(),
            menu_panels: Vec::new(),
        }
    }

    //=====================================
    // Init
    //=====================================

    pub fn init_screen(&mut self, event_manager: &mut EventManager, screen_rez: [f32; 2], ctx: &mut GlContext) {
        self.set_screen_rez(screen_rez, ctx);
        event_manager.add_render_event(RenderEvent::ChangeMenu(CurrentMenu::MainMenu));
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_screen(&mut self,
        texture_manager: &mut TextureManager,
        world: Arc<RwLock<World>>,
        world_rendering_task_manager: &mut RenderingTaskManager,
        tik_manager: &TikManager,
        player_data: &mut PlayerData,
        game_event_manager: &mut EventManager,
        ctx: &mut GlContext
    ) {
        if self.screen_data.should_quit() {
            window::order_quit();
        }
        self.screen_data.update_inputs();

        match self.screen_data.get_current_menu() {
            CurrentMenu::MapView => {
                world_rendering_task_manager.execute_render_updates_drone();
            }
            _ => {}
        }

        for panel in &mut self.menu_panels {
            panel.render(texture_manager, &self.screen_data, game_event_manager, player_data);
        }

        game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().render(texture_manager, &self.screen_data);
    }

    //=====================================
    // Input Handling
    //=====================================

    pub fn mouse_motion_event(&mut self, x_cor: f32, y_cor: f32) {
        self.screen_data.add_input(Input::MouseMotion(x_cor, y_cor));
        self.screen_data.set_mouse_pixel_cords([x_cor as i32, y_cor as i32]);
    }

    pub fn mouse_button_down_event(&mut self,
        button: MouseButton,
        _tik_manager: &mut TikManager,
        _world_task_manager: &mut WorldTaskManager,
        _event_manager: &mut EventManager,
    ) {
        self.screen_data.add_input(Input::MouseButtonDown(button));
        if button == MouseButton::Middle {
            self.screen_data.set_middle_mouse_held(true);
        }
    }

    pub fn mouse_button_up_event(&mut self, _event_manager: &mut EventManager, button: MouseButton) {
        self.screen_data.add_input(Input::MouseButtonUp(button));
        if button == MouseButton::Middle {
            self.screen_data.set_middle_mouse_held(false);
        }
    }

    pub fn key_down_event(&mut self,
        _event_manager: &mut EventManager,
        _tik_manager: &mut TikManager,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) {
        self.screen_data.add_input(Input::KeyDown(keycode));
        match keycode {
            KeyCode::F3 => {
                self.screen_data.set_debug_visibility(!self.screen_data.get_debug_visiblity());
            }
            _ => {}
        }
    }

    pub fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        self.screen_data.add_input(Input::MouseWheel(_x, _y));
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn set_menu_panel(&mut self, panel: WidgetType) {
        self.menu_panels.clear();
        self.menu_panels.push(panel);
    }

    pub fn get_screen_data(&self) -> &ScreenData {
        return &self.screen_data;
    }

    pub fn get_mut_screen_data(&mut self) -> &mut ScreenData {
        return &mut self.screen_data;
    }

    pub fn get_mut_window_manager(&mut self) -> Option<&mut WidgetWindowManager> {
        match self.screen_data.get_current_menu() {
            CurrentMenu::PlayView => {
                if let WidgetType::WidgetWindowManager(window_manager) = &mut self.menu_panels[0] {
                    Some(window_manager)
                } else {
                    eprintln!("Missing window manager from play view");
                    None
                }
            }
            _ => None,
        }
    }

    pub fn set_screen_rez(&mut self, screen_rez: [f32; 2], ctx: &mut GlContext) {
        self.screen_data.set_screen_rez(screen_rez, ctx);
    }
}
