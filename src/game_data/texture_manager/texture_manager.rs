use crate::game_data::{log_init, texture_manager::{texture_renderer::TextureRenderingManager, texture_atlas::TextureAtlas}, Types::{BlockShaderType, BlockTriangle, BlockType, ShaderTriangle}};
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

    pub fn are_textures_initialized(&self) -> bool {
        return self.textures_initialized;
    }

    pub fn get_texture_renderer(&mut self) -> &mut TextureRenderingManager {
        return self.texture_renderer.as_mut().unwrap();
    }

    /// Update the expander cache for the current frame's scale. Call this once per frame before rendering triangles.
    pub fn update_expander_cache(&mut self, scale: f32) {
        self.cached_scale = scale;
        self.cached_expander = (scale * EXPANDER_BASE_MULTIPLIER) 
                             + (EXPANDER_OFFSET_STRENGTH / scale.max(EXPANDER_SCALE_THRESHOLD));
    }

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


    pub fn init_textures(&mut self, ctx : &mut GlContext) {
        // Create Texture render manager
        let texture_render_manager = TextureRenderingManager::new(ctx);
        self.texture_renderer = Some(texture_render_manager);
        
        // Init Texture atlas
        let new_texture_atlas = TextureAtlas::new(ctx);
        
        // Set texture for renderer
        if let Some(renderer) = self.texture_renderer.as_mut() {
            renderer.set_texture(new_texture_atlas.texture_id);
        }
        
        self.texture_atlas = Some(new_texture_atlas);

        // Set to initialized 
        self.textures_initialized = true;
        println!("Created miniquad Texture");
    }

    // Testing the rendering of sprites
    pub fn test_sprites(&mut self, ctx : &mut GlContext) {
        if !self.textures_initialized {
            println!("Textures not initialized yet!");
            return;
        }
        
        if let Some(texture_renderer) = self.texture_renderer.as_mut() {
            if let Some(texture_atlas) = self.texture_atlas.as_mut() {
                texture_renderer.set_texture(texture_atlas.texture_id);
                

                

                //self.render_block_triangle(BlockType::Debug, BlockTriangle::LeftBot, [0.0, 0.0], 0.3);
                //self.render_shader_triangle(BlockShaderType::Grey, ShaderTriangle::LeftCenterRight, [0.0, 0.0], 0.3);

                
                // Get shader spritesheet coordinates from the atlas
                let shader_start = texture_atlas.shader_texture_manager.start_cords;
                let shader_end = texture_atlas.shader_texture_manager.end_cords;
                let atlas_size = texture_atlas.atlas_dimensions as f32;
                
                // Render the entire shader spritesheet for debugging
                texture_renderer.add_quad(
                    [-0.5, -0.5, 5.0, 5.0],  // Fill the screen
                    [0.0, 0.0, 1.0, 1.0]
                );
                
                
            }
        }
    }

}
