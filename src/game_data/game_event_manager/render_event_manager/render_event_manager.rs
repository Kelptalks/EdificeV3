use std::sync::{Arc, RwLock};

use crate::game_data::{World, game_event_manager::{game_event_manager::EventData, render_event_manager}, screen::{menu_constructors, screen_data::CurrentMenu, screen_mananager::ScreenManager, widget::widget::{Widget, WidgetType}}};

/*
##################
## Render Event ##
##################
Events relating to rendering of menus / game camera

*/
#[derive(Clone, PartialEq)]
pub enum RenderEvent {
    // Window
    QuitGame,
    
    // Camera
    InitWorldRender(),               // Range 
    ReRenderBlock([i32; 3]),    // Cords of block modified

    // Menu
    ChangeMenu(CurrentMenu),    // Current menu
    Clear,                    // None

    // Testing
    TestEvent,
}



impl RenderEvent {


    pub fn construct_menu(current_menu: CurrentMenu, screen_mananager: &mut ScreenManager, event_tools: &mut EventData) -> WidgetType 
    {
        let mut menu_panel= WidgetType::new_panel([0.0; 4], [0.0; 4]);
        match current_menu {
            CurrentMenu::MainMenu => {
                menu_panel = menu_constructors::main_menu::get_menu(&screen_mananager.get_mut_screen_data());
            }
            CurrentMenu::WorldCreationMenu => {
                menu_panel =
                    menu_constructors::world_creation_menu::get_menu(
                        &screen_mananager.get_mut_screen_data(), 
                        event_tools.get_mut_world_gen_manager().get_mut_world_config(
                    )
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
    pub fn execute_render_event(&self, event_tools: &mut EventData, screen_mananager: &mut ScreenManager, world:&Arc<RwLock<World>>) {
        let camera = screen_mananager.get_mut_camera();
        let camera_data = &camera.get_camera_data().clone();
        match self {
            RenderEvent::QuitGame => {
                screen_mananager.get_mut_screen_data().quit();
            }
            RenderEvent::InitWorldRender() => {
                let range = event_tools.get_world_gen_manager().get_world_config().get_chunk_rendering_range();
                camera.dirty_chunks_in_area(
                    &camera_data, 
                    range as i32
                );
                camera.ray_cast_dirty_chunks(camera_data.clone().get_arc_ref(), world);
            },
            RenderEvent::ReRenderBlock(cords) => {
                let casted_tile_cords = camera_data.world_to_casted_tile_cords(*cords);
                camera.dirty_tiles_in_area(casted_tile_cords, 2);
            }
            RenderEvent::ChangeMenu(current_menu) => {
                let menu_panel: WidgetType = Self::construct_menu(*current_menu, screen_mananager, event_tools);
                screen_mananager.set_menu_panel(menu_panel);
                screen_mananager.get_mut_screen_data().set_current_menu(*current_menu);
            }
            RenderEvent::Clear => {
                
            }
            RenderEvent::TestEvent => {
                println!("Test Render Event");
            }
        }
    }
}