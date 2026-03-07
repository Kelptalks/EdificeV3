use std::sync::{Arc, RwLock};

use miniquad::GlContext;

use crate::game_data::{TextureManager, World, screen::{ScreenData, render_centered_string_at_ndc, renderer::{camera, casted_block_manager::casted_block_manager::CastedChunkManager}}, types::{FontType, UITextures}};


#[derive(PartialEq, Clone)]
enum LoadingState {
    ShowLoading,
    GenerateTerrain,
    InitRendering,
    Finilize,
    Done
}
impl LoadingState {
    fn to_string(&self) -> String {
        match self {
            LoadingState::ShowLoading => {
                return "Loading...".to_string();
            }
            LoadingState::GenerateTerrain => {
                return "Generating World".to_string();
            }
            LoadingState::InitRendering => {
                return "Rendering World".to_string();
            }
            LoadingState::Finilize => {
                return "Finilizing".to_string();
            }
            LoadingState::Done => {
                return "Done".to_string();
            }
        }
    }
}

#[derive(Clone)]
pub struct WorldConfig {
    scale: u32,
    height_variation: u32,
    loading_state: LoadingState,
}

impl WorldConfig {
    pub fn new() -> Self {
        WorldConfig {
            scale: 200,
            height_variation: 100,
            loading_state: LoadingState::ShowLoading,
        }
    }

    pub fn set_scale(&mut self, scale: u32) {
        self.scale = scale;
    }
    pub fn get_scale(&self) -> u32 {
        self.scale
    }
    pub fn get_chunk_rendering_range(&self) -> u32 {
        return (self.get_scale() / CastedChunkManager::get_chunk_tile_scale() / 2) + 4;
    }

    pub fn get_height_variation(&self) -> u32 {
        self.height_variation
    }

    pub fn done_initializing(&self) -> bool {
        if self.loading_state == LoadingState::Done {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn render_loading_screen(&self, texture_manager: &mut TextureManager, ctx: &mut GlContext) {
        let screen_uv = [-1.0, -1.0, 1.0, 1.0];
        texture_manager.render_ui_element_with_pos(UITextures::VoidBackground, screen_uv);
        render_centered_string_at_ndc(texture_manager, 
            self.loading_state.to_string(), 
            FontType::Basic, 
            0.1, 
            [0.0, 0.0]
        );
        texture_manager.get_texture_renderer().flush(ctx);
    }
}