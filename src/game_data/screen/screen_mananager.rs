use miniquad::{GlContext, RenderingBackend};

use crate::game_data::screen::Camera;





enum CurrentMenu {
    MainMenu,
    Renderer,

}

/*
####################
## Screen Manager ##
####################
This file is resposible for managing the game screen. It handles whitch menu is currently visibile.
how those menus are rendered and how those controls are processed.



*/

pub struct ScreenManager {
    // Menu Structs

    //
    
    // Screen Data
    screen_rez: [f32; 2],
    viewport_rez: [f32; 2],
    viewport_offset: [f32; 2],

}



impl ScreenManager {
    pub fn new()->Self {
        
        let new_screen = ScreenManager{
            screen_rez: [0.0, 0.0],
            viewport_rez: [0.0, 0.0],
            viewport_offset: [0.0, 0.0],
        };
        
        return  new_screen;
    }

    pub fn init_screen(&mut self, screen_rez: [f32; 2], ctx : &mut GlContext) {
        self.set_screen_rez(screen_rez, ctx);
    }

    pub fn set_screen_rez(&mut self, screen_rez: [f32; 2], ctx : &mut GlContext) {
        // Calculate and setup viewport and set the correct values 

        let mut viewport_size: f32;

        // Set up square viewport to allow for consistant rendering
        if screen_rez[0] > screen_rez[1] {
            viewport_size = screen_rez[0];
            self.viewport_offset = [0.0, -(viewport_size - screen_rez[1]) / 2.0];
        } else {
            viewport_size = screen_rez[1];
            self.viewport_offset = [-(viewport_size - screen_rez[0]) / 2.0, 0.0];
        }

        self.screen_rez = screen_rez;
        self.viewport_rez = [viewport_size, viewport_size];

        // set the viewport using ctx
        ctx.apply_viewport(
            self.viewport_offset[0] as i32, 
            self.viewport_offset[1] as i32, 
            self.viewport_rez[0] as i32, 
            self.viewport_rez[1] as i32, 
        );
    }

}


