
use std::{cell::RefCell, rc::Rc};

use crate::game_data::{TextureManager, game_event_manager::{game_event_manager::game_event_manager::GameEventManager, prelude::{Event, GameEvent}, render_event_manager::{texture_manager_event::TextureManagerEvent, tile_map_event::{self, TileMapEvent}, window_manager_event::WindowManagerEvent}}, player_data::player_data::PlayerData, screen::{menu_constructors, screen_data::CurrentMenu, screen_mananager::ScreenManager, widget::{widget::WidgetType, world_rendering::tile_map_manager}}, texture_manager};

/*
##################
## Render Event ##
##################
Events relating to rendering of menus / game camera

*/
#[derive(Clone)]
pub enum RenderEvent {
    WindowMangerEvent(WindowManagerEvent),
    TileMapEvent(TileMapEvent),
    TextureManagerEvent(TextureManagerEvent),


    // Window
    QuitGame,
    
    // Camera
    _InitWorldRender(),               // Range 
    ReRenderBlock([i32; 3]),    // Cords of block modified

    // Menu
    ChangeMenu(CurrentMenu),    // Current menu
    Clear,                    // None

    // Testing
    DebugEvent,
}



impl RenderEvent {

    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::RenderEvent(self));
    }

    pub fn wrap_into_game_event(self) -> GameEvent {
        GameEvent::RenderEvent(self)
    }

    pub fn construct_menu(
        current_menu: CurrentMenu, 
        screen_mananager: &mut ScreenManager, 
        event_tools: &mut GameEventManager, 
        player_data: &PlayerData,
    ) -> WidgetType 
    {
        let mut menu_panel= WidgetType::new_panel([0.0; 4], [0.0; 4]);
        match current_menu {
            CurrentMenu::MainMenu => {
                menu_panel = menu_constructors::main_menu::get_menu(&screen_mananager.get_mut_screen_data());
            }
            CurrentMenu::WorldCreationMenu => {
                
                menu_panel =
                    menu_constructors::world_creation_menu::get_menu(
                        &screen_mananager.get_mut_screen_data()
                    )
                
            }
            CurrentMenu::SettingsMenu => {
                menu_panel = 
                    menu_constructors::settings_menu::get_menu(
                        &screen_mananager.get_mut_screen_data(),
                    );
            }
            CurrentMenu::PlayView => {
                menu_panel = menu_constructors::play_view_menu::play_view_menu::get_menu(
                    &screen_mananager.get_mut_screen_data(), 
                    player_data
                );
            }
            _ => {

            }
        }

        return menu_panel;
    }
    

    //=====================================
    // Execution
    //=====================================
    pub fn execute_render_event(
        self, 
        event_tools: &mut GameEventManager, 
        texture_manager: &mut TextureManager,
        screen_mananager: &mut ScreenManager,
        player_data: &PlayerData
    ) -> Vec<Event> {
        let camera = screen_mananager.get_mut_camera();
        let camera_data = &camera.get_camera_data().clone();
        let world = player_data.get_world_ref();
        match self {
            RenderEvent::WindowMangerEvent(window_manager_event) => {
                if let Some(window_manager) = screen_mananager.get_mut_window_manager() {
                    window_manager_event.execute_event(window_manager);
                }
            },
            RenderEvent::TileMapEvent(tile_map_event) => {
                if let Some(window_manager) = screen_mananager.get_mut_window_manager() {
                    return tile_map_event.execute_event(window_manager.get_tile_map_manager())
                    
                }    
            }
            RenderEvent::TextureManagerEvent(texture_manager_event) => {
                texture_manager_event.execute_event(texture_manager);
            },

            RenderEvent::QuitGame => {
                screen_mananager.get_mut_screen_data().quit();
            }
            RenderEvent::_InitWorldRender() => {
                /*
                let range = event_tools.get_world_gen_manager().get_world_config().get_chunk_rendering_range();
                camera.dirty_chunks_in_area(
                    &camera_data, 
                    range as i32
                );
                camera.ray_cast_dirty_chunks(camera_data.clone().get_arc_ref(), &world);
                 */
            },
            RenderEvent::ReRenderBlock(cords) => {
                let casted_tile_cords = camera_data.world_to_casted_tile_cords(cords);
                camera.dirty_tiles_in_area(casted_tile_cords, 2);
            }
            RenderEvent::ChangeMenu(current_menu) => {
                let menu_panel: WidgetType = Self::construct_menu(current_menu, screen_mananager, event_tools, player_data);
                screen_mananager.set_menu_panel(menu_panel);
                screen_mananager.get_mut_screen_data().set_current_menu(current_menu);
            }
            RenderEvent::Clear => {
                
            }
            RenderEvent::DebugEvent => {
                println!("Test Render Event");
            }
        }
        Vec::new()
    }
}