use crate::game_data::{log_init, texture_manager::{texture_atlas::TextureAtlas, texture_renderer::TextureRenderingManager}, types::{BlockShaderType, BlockTriangle, BlockType, CharType, ShaderTriangle, UITextures}};
use miniquad::*;

// Expander tuning constants - adjust these to control gap prevention
static EXPANDER_BASE_MULTIPLIER: f32 = 0.005;  // Base scaling with sprite size
static EXPANDER_OFFSET_STRENGTH: f32 = 0.00005; // Additional offset to prevent gaps at low zoom
static EXPANDER_SCALE_THRESHOLD: f32 = 0.1;     // Scale below which offset strength increases


pub struct TextureManager {
    textures_initialized : bool,
    texture_renderer : Option<TextureRenderingManager>,
    texture_atlas : Option<TextureAtlas>,

    // Expander caching for performance
    cached_scale: f32,
    cached_expander: f32,
}

impl TextureManager {

    //======================
    // Initialization
    //======================

    pub fn new() -> Self {
        log_init("Creating Texture Manager");
        Self {
            textures_initialized : false,
            texture_renderer : None,
            texture_atlas : None,
            cached_scale: 0.0,
            cached_expander: 0.0,
        }
    }

    pub fn init_textures(&mut self, ctx : &mut GlContext) {
        // Create Texture render manager
        let texture_render_manager = TextureRenderingManager::new(ctx);
        self.texture_renderer = Some(texture_render_manager);
        
        // Init Texture atlas
        let new_texture_atlas = TextureAtlas::new(ctx);
        
        // Set texture for renderer
        if let Some(renderer) = self.texture_renderer.as_mut() {
            renderer.set_texture(new_texture_atlas.texture_id);
            self.get_texture_renderer().set_texture(new_texture_atlas.get_atlas_texture_id());
        }
        
        self.texture_atlas = Some(new_texture_atlas);

        // Set to initialized 
        self.textures_initialized = true;
    }

    pub fn are_textures_initialized(&self) -> bool {
        return self.textures_initialized;
    }

    pub fn get_texture_renderer(&mut self) -> &mut TextureRenderingManager {
        return self.texture_renderer.as_mut().unwrap();
    }

    pub fn set_texture_renderer_to_atlas(&mut self) {
        let texture_id = self.texture_atlas.as_ref().unwrap().get_atlas_texture_id();
        self.get_texture_renderer().set_texture(texture_id);
    }

    //======================
    // Expander managment
    //======================

    /// Update the expander cache for the current frame's scale. Call this once per frame before rendering triangles.
    pub fn update_expander_cache(&mut self, scale: f32) {
        self.cached_scale = scale;
        self.cached_expander = (scale * EXPANDER_BASE_MULTIPLIER) 
                             + (EXPANDER_OFFSET_STRENGTH / scale.max(EXPANDER_SCALE_THRESHOLD));
    }

    pub fn set_cached_expander(&mut self, cashed_expander: f32){
        self.cached_expander = cashed_expander;
    }

    //=================================================
    // Render Specific Sprites from Main Texture Atlas
    //=================================================

    pub fn render_block_triangle(&mut self, block : BlockType, triangle : BlockTriangle, draw_location : [f32; 2], scale : f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_block_triangle_uv(triangle, block);

        let mut pos = [
            draw_location[0] - self.cached_expander,         // x1 (left)
            draw_location[1] - self.cached_expander,         // y1 (top/bottom) 
            draw_location[0] + (scale) + self.cached_expander, // x2 (right)
            draw_location[1] + (scale) + self.cached_expander, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }

    pub fn render_shader_triangle(&mut self, shader : BlockShaderType, triangle : ShaderTriangle, draw_location : [f32; 2], scale : f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_shader_triangle_uv(triangle, shader);

        let mut pos = [
            draw_location[0] - self.cached_expander,         // x1 (left)
            draw_location[1] - self.cached_expander,         // y1 (top/bottom) 
            draw_location[0] + (scale) + self.cached_expander, // x2 (right)
            draw_location[1] + (scale) + self.cached_expander, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }

    pub fn render_char(&mut self, font: String, char : CharType, draw_location : [f32; 2], scale : f32) {
        if char == CharType::CharSpace {
            return; // Don't render spaces
        }
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_font_uv(font, char);

        let mut pos = [
            draw_location[0],         // x1 (left)
            draw_location[1],         // y1 (top/bottom) 
            draw_location[0] + scale, // x2 (right)
            draw_location[1] + scale, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }

    pub fn render_ui_element(&mut self, ui_texture: UITextures, draw_location: [f32; 2], scale: f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_ui_uv(ui_texture);

        let pos = [
            draw_location[0],         // x1 (left)
            draw_location[1],         // y1 (top/bottom) 
            draw_location[0] + scale, // x2 (right)
            draw_location[1] + scale, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }

    pub fn render_ui_element_with_pos(&mut self, ui_texture: UITextures, pos: [f32; 4]) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_ui_uv(ui_texture);

        self.get_texture_renderer().add_quad(pos, uv);
    }

    //==========
    // Testing
    //==========

    pub fn test_sprites(&mut self, ctx : &mut GlContext) {
        if !self.textures_initialized {
            println!("Textures not initialized yet!");
            return;
        }
        
        if let Some(texture_renderer) = self.texture_renderer.as_mut() {
            if let Some(texture_atlas) = self.texture_atlas.as_mut() {
                //texture_renderer.set_texture(texture_atlas.texture_id);
                

                
                // Render the entire shader spritesheet for debugging
                texture_renderer.add_quad(
                    [-0.5, -0.5, 5.0, 5.0],  // Fill the screen
                    [0.0, 0.0, 1.0, 1.0]
                );

                self.render_ui_element(UITextures::ButtonCheck, [0.0, 0.0], 0.1);
                self.render_ui_element(UITextures::ButtonCheck_Down, [0.2, 0.0], 0.1);
                //self.render_ui_element(UITextures::ButtonLeftArrow, [0.2, 0.0], 0.1);

                }
        }
    }

    

}
