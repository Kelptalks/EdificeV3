use std::sync::{Arc, RwLock};

use image::imageops::FilterType::Triangle;
use miniquad::{window, GlContext, KeyCode, KeyMods, MouseButton, RenderingBackend};

use crate::game_data::{TextureManager, World, debuging::debug_data::{self, DebugData}, game_event_manager::{self, game_event_manager::{Event, GameEventManager}, render_event_manager::render_event_manager::RenderEvent}, log_init, player_data::player_data::PlayerData, screen::{self, Camera, ScreenData, camera_controls, camera_data::{self, CameraData}, camera_ui::camera_ui_manager::CameraUIManager, input_data::Input, iso_cord_tool, menus::{level_select_menu::{self, level_select_menu::LevelSelectMenu}, main_menu::main_menu::MainMenu, world_creation_menu::world_creation::WorldCreationMenu}, play_view::play_view::PlayView, render_centered_string_at_ndc, renderer::casted_block_manager::{casted_block_manager::CastedChunkManager, casted_tile::{self, CastedTile}}, screen_data::{self, CurrentMenu}, screen_task_manager::rendering_task_manager::RenderingTaskManager, widget::{panel::panel::{PanelAlignment, PanelOrientation}, widget::{Widget, WidgetType}, widget_calculations::TextSize}}, tik_manager::{self, tik_manager::TikManager}, types::UITextures, world_task_manager::{self, world_task_manager::WorldTaskManager}};




/*
####################
## Screen Manager ##
####################
This file is resposible for managing the game screen. It handles whitch menu is currently visibile.
how those menus are rendered and how those controls are processed.
*/

pub struct ScreenManager {
    // Menu Structs
    main_menu : MainMenu,
    world_creation_menu: WorldCreationMenu,
    level_select_menu: LevelSelectMenu,
    
    // Camera
    play_view: PlayView,

    camera : Camera,
    camera_ui_manager: CameraUIManager,

    // Screen Data
    screen_data: ScreenData,

    // testing
    panel: Vec<Option<WidgetType>>,
}

impl ScreenManager {
    pub fn new()->Self {
        let camera = Camera::new();
        let screen_data = ScreenData::new();

        let new_screen = ScreenManager{
            // Menu Structs
            main_menu: MainMenu::new(),
            world_creation_menu: WorldCreationMenu::new(),
            level_select_menu: LevelSelectMenu::new(),
        
            // Camera
            play_view: PlayView::new(),


            camera: camera,
            camera_ui_manager: CameraUIManager::new(),

            // Screen Data
            screen_data: screen_data, 

            // testing
            panel: Vec::new(),
        };
        
        
        return  new_screen;
    }

    //=====================================
    // Init functions
    //=====================================

    pub fn init_screen(&mut self, event_manager: &mut GameEventManager, screen_rez: [f32; 2], ctx : &mut GlContext) {
        // Init level manager with all levels
        self.level_select_menu.init(event_manager);
        
        self.set_screen_rez(screen_rez, ctx);
        self.camera.initialize_camera(ctx);

        // Testing
        let parent_pos = self.screen_data.get_viewport_uv();
        let buffers = [0.0; 4];
        
        let mut panel = WidgetType::new_panel(parent_pos, buffers);
        if let WidgetType::Panel(panel) = &mut panel {
            panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
            panel.add_header("Test Header".to_string());
            let bar_button = panel.add_bar_button("Start".to_string(), Event::RenderEvent(RenderEvent::TestEvent));
            bar_button.set_text_scale(TextSize::Medium);
            

            for s in 0..5 {
                let mut sub_panel = panel.add_sub_panel();
                sub_panel.add_header("Test Header".to_string());
                sub_panel.set_color(screen::widget::panel::panel_color::PanelColor::Dark);
                sub_panel.set_orientation(PanelOrientation::Horizontal, PanelAlignment::Center);

                for i in 0..10 {
                    let mut button = sub_panel.add_button(Event::RenderEvent(RenderEvent::TestEvent));
                    button.set_text("test_text".to_string());
                }
            }

            
            panel.size();
        }

        self.panel.push(Some(panel));

    }

    //=====================================
    // menu Rendering
    //=====================================

    pub fn render_screen(&mut self, 
        texture_manager: &mut TextureManager, 
        world: Arc<RwLock<World>>, 
        world_rendering_task_manager: &mut RenderingTaskManager,
        tik_manager: &TikManager,
        player_data: &mut PlayerData,
        game_event_manager: &mut GameEventManager,
        ctx : &mut GlContext
    ){
        // If quit
        if self.screen_data.should_quit() {
            window::order_quit();
        }
        self.screen_data.update_inputs();

        // Update screen cords
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);
        
        match self.screen_data.get_current_menu() {
            CurrentMenu::MainMenu => {
                self.main_menu.render_main_menu(texture_manager, &self.screen_data);
            }
            CurrentMenu::WorldCreationMenu => {
                self.world_creation_menu.render(texture_manager, &self.screen_data);
            }
            CurrentMenu::LevelSelectMenu => {
                self.level_select_menu.render_menu(texture_manager, &self.screen_data);
            }
            CurrentMenu::Camera => {
                self.camera.render_camera(texture_manager, world.clone(), ctx);
                self.camera_ui_manager.render_ui(&self.screen_data, texture_manager, self.camera.get_camera_data(), &world.clone(), tik_manager);
                world_rendering_task_manager.execute_render_updates_drone(&mut self.camera, texture_manager);
            }
            CurrentMenu::PlayView => {
                self.play_view.render_view(&self.screen_data, texture_manager, &world, player_data, game_event_manager);
                
            }
        }

        for _panel in &mut self.panel {
            if let Some(panel) = _panel {
                panel.render(texture_manager, &self.screen_data, game_event_manager);
                
            }
        }

        self.screen_data.clear_inputs();

        
    }

    //=====================================
    // Input Handling
    //=====================================

    // handle mouse movment
    pub fn mouse_motion_event(&mut self, x_cor: f32, y_cor: f32) {
        self.screen_data.add_input(Input::MouseMotion(x_cor, y_cor));
        self.screen_data.set_mouse_pixel_cords([x_cor as i32, y_cor as i32]);
    
        match self.screen_data.get_current_menu() {
            CurrentMenu::MainMenu => {
                self.main_menu.handle_mouse_motion_input(&self.screen_data);
            },
            CurrentMenu::WorldCreationMenu => {
                self.world_creation_menu.handle_mouse_motion_input(&self.screen_data);
            },
            CurrentMenu::LevelSelectMenu => {
                self.level_select_menu.handle_mouse_motion_input(&self.screen_data);
            },
            CurrentMenu::Camera => {
                camera_controls::mouse_motion_event(self, x_cor, y_cor);
                self.camera_ui_manager.handle_motion_event(&self.screen_data);
            },
            CurrentMenu::PlayView => {
                self.play_view.mouse_motion_event(&self.screen_data);
            },
        }

    }

    // Handle mouse button press
    pub fn mouse_button_down_event(&mut self,
        button: MouseButton,
        tik_manager: &mut TikManager,
        world_task_manager: &mut WorldTaskManager,
        event_manager: &mut GameEventManager,
    ) {
        self.screen_data.add_input(Input::MouseButtonDown(button));
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);

        if button == MouseButton::Middle {
            self.screen_data.set_middle_mouse_held(true);
        }

        match self.screen_data.get_current_menu() {
            CurrentMenu::MainMenu => {
                self.main_menu.handle_mouse_button_down(event_manager, &mut self.screen_data, button);
            },
            CurrentMenu::WorldCreationMenu => {
                self.world_creation_menu.handle_mouse_button_down(event_manager, &mut self.screen_data, button);
            },
            CurrentMenu::LevelSelectMenu => {
                self.level_select_menu.handle_mouse_button_down(event_manager, &mut self.screen_data, button);
            },
            CurrentMenu::Camera => {
                camera_controls::mouse_button_down_event(self, button);
                self.camera_ui_manager.handle_mouse_button_down(button, &self.screen_data, tik_manager, world_task_manager);
            },
            CurrentMenu::PlayView => {
                self.play_view.mouse_button_down_event(event_manager, &self.screen_data, button);
            }
        }
    }

    // Handle mouse button release
    pub fn mouse_button_up_event(
        &mut self,
        event_manager: &mut GameEventManager,
        button: MouseButton,
    ) {
        self.screen_data.add_input(Input::MouseButtonUp(button));
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);
        
        // Screen Data updating
        if button == MouseButton::Middle {
            self.screen_data.set_middle_mouse_held(false);
        }

        // Execution
        match self.screen_data.get_current_menu() {
            CurrentMenu::MainMenu => {},
            CurrentMenu::WorldCreationMenu => {},
            CurrentMenu::LevelSelectMenu => {},
            CurrentMenu::Camera => {
                camera_controls::mouse_button_up_event(self, button);
                self.camera_ui_manager.handle_mouse_button_up(button, &self.screen_data);
            },
            CurrentMenu::PlayView => {
                self.play_view.mouse_button_up_event(event_manager, &self.screen_data, button);
            },
        }
    }

    // Handle key press
    pub fn key_down_event(&mut self,
        event_manager: &mut GameEventManager,
        tik_manager: &mut TikManager,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool
    ) {
        self.screen_data.add_input(Input::KeyDown(keycode));
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);

        // Handle debug menu
        match keycode {
            KeyCode::F3 => {
            self.screen_data.set_debug_visibility(!self.screen_data.get_debug_visiblity());
            }
            _ => {}
        }



        match self.screen_data.get_current_menu() {
            CurrentMenu::MainMenu => {

            },
            CurrentMenu::WorldCreationMenu => {

            },
            CurrentMenu::LevelSelectMenu => {

            },
            CurrentMenu::Camera => {
                camera_controls::key_down_event(event_manager, self,  keycode, keymods, repeat);
                self.camera_ui_manager.handle_key_down(event_manager, keycode, tik_manager);
            },
            CurrentMenu::PlayView => {
                self.play_view.key_down_event(event_manager, keycode);
            }
        }
    }

    // Handle mouse wheel
    pub fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        self.screen_data.add_input(Input::MouseWheel(_x, _y));
        let camera_data = &self.get_camera_data().clone();
        self.screen_data.re_calculate_mouse_cords(camera_data);
        if self.screen_data.get_current_menu() == CurrentMenu::MainMenu {
            
        }
        else if self.screen_data.get_current_menu() == CurrentMenu::Camera {
            camera_controls::mouse_wheel_event(self, _x, _y);
        }
        else if self.screen_data.get_current_menu() == CurrentMenu::PlayView {
            self.play_view.mouse_wheel_event(_x, _y);
        }
    }

//=====================================
// Getters / Setters
 //=====================================

    pub fn get_mut_camera(&mut self) -> &mut Camera {
        return &mut self.camera;
    }

    // Camera Data
    pub fn get_mut_camera_data(&mut self) -> &mut CameraData{
        return self.camera.get_mut_camera_data();
    }
    pub fn get_camera_data(&self) -> &CameraData {
        return &self.camera.get_camera_data();
    }

    pub fn set_screen_rez(&mut self, screen_rez: [f32; 2], ctx : &mut GlContext) {
        // Calculate and setup viewport and set the correct values 
        self.screen_data.set_screen_rez(screen_rez, ctx);

        // Update viewport in camera
        self.camera.update_viewport(self.screen_data.get_viewport_rez(), self.screen_data.get_viewport_offset());

        // Update ui 
        self.main_menu.window_resize_update(&self.screen_data);
        self.camera_ui_manager.window_resize_update(&self.screen_data);
        self.level_select_menu.window_resize_update(&self.screen_data);
        self.world_creation_menu.window_resize_update(&self.screen_data);
        self.play_view.window_resize_update(&self.screen_data);
    }

    pub fn get_screen_data(&self) -> &ScreenData {
        return &self.screen_data;
    }

    pub fn get_mut_screen_data(&mut self) -> &mut ScreenData {
        return &mut self.screen_data;
    }


    //=====================================
    // Mouse / Control handling
    //=====================================

    pub fn update_mouse_cords(&mut self, x_cor: f32, y_cor: f32){
        self.screen_data.set_mouse_pixel_cords([x_cor as i32, y_cor as i32]);
    }


    pub fn get_mouse_debug_casted_tile(&self) -> CastedTile {
        let screen_data = self.get_screen_data();
        let iso_cords = screen_data.get_mouse_iso_world_cords();
        let mut casted_tile = CastedTile::new(iso_cords);
        
        let triangles = casted_tile.get_mut_triangles();
        triangles[0].add_texture(crate::game_data::types::BlockTexture::Debug, crate::game_data::types::BlockTriangle::TopLeft);
        triangles[1].add_texture(crate::game_data::types::BlockTexture::Debug, crate::game_data::types::BlockTriangle::TopRight);
        return casted_tile;
    }

    //=====================================
    // Debugging
    //=====================================

    pub fn collect_debug_data(&self, debug_data: &mut DebugData) { 
        self.camera.collect_debug_data(debug_data);
        self.play_view.collect_debug_data(debug_data);

        debug_data.set_mouse_tile_cords(self.screen_data.get_mouse_iso_world_cords());
    }

}
