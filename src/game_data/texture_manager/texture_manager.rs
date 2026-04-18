
use crate::game_data::{texture_manager::{texture::Texture, texture_atlas::{self, TextureAtlas}, texture_renderer::TextureRenderingManager}, types::{BlockShader, BlockTexture, BlockTriangle, CharType, DroneItemTexture, DroneUITexture, FontType, ShaderTriangle, UITextures}};
use miniquad::*;

// Expander tuning constants - adjust these to control gap prevention
static EXPANDER_BASE_MULTIPLIER: f32 = 0.005;  // Base scaling with sprite size
static EXPANDER_OFFSET_STRENGTH: f32 = 0.00005; // Additional offset to prevent gaps at low zoom
static EXPANDER_SCALE_THRESHOLD: f32 = 0.1;     // Scale below which offset strength increases

/*
#####################
## Texture Manager ##
#####################
Creates and manages the texture atlas which contains
all sprites, and manages the rendering of these
sprites.
*/


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
    // Texture Type Rendering
    //=================================================
    fn get_texture_uv(&mut self, texture: Texture) -> [f32; 4] {

        let texture_atlas = self.texture_atlas.as_ref().unwrap();
        
        match texture {
            Texture::BlockTexture(block_texture) => {
                return texture_atlas.get_precalculated_block_uv(block_texture)
            },
            Texture::BlockTriangle(block_texture, block_triangle) => {
                return texture_atlas.get_precalculated_block_triangle_uv(block_triangle, block_texture)
            },
            Texture::BlockShader(block_shader) => {
                todo!("Texture Enum rendering for block shader not implemented");
            },
            Texture::DroneItemTexture(drone_item_texture) => {
                return texture_atlas.get_precalculated_drone_item_uv(drone_item_texture)
            },
            Texture::UITexture(uitextures) => {
                return texture_atlas.get_precalculated_ui_uv(uitextures)
            },
        }
    }
    
    pub fn render_texture(&mut self, texture: Texture, pos: [f32; 4]) {
        match texture {
            Texture::BlockTexture(block_texture) => {
                self.render_block_with_pos(block_texture, pos);
            },
            Texture::BlockTriangle(block_texture, block_triangle) => {
                self.render_block_triangle_with_pos(block_texture, block_triangle, pos);
            },
            Texture::BlockShader(block_shader) => {
                todo!("Texture Enum rendering for block shader not implemented");
            },
            Texture::DroneItemTexture(drone_item_texture) => {
                self.render_drone_item_with_pos(drone_item_texture, pos);
            },
            Texture::UITexture(ui_texture) => {
                self.render_ui_element_with_pos(ui_texture, pos);
            },
        }
    }

    pub fn render_texture_within_pos(&mut self, texture: Texture, draw_pos: [f32; 4], bounds_pos: [f32; 4]) -> bool {
        let uv = self.get_texture_uv(texture);


        let [dx1, dy1, dx2, dy2] = draw_pos;
        let [bx1, by1, bx2, by2] = bounds_pos;

        let cx1 = dx1.max(bx1);
        let cy1 = dy1.max(by1);
        let cx2 = dx2.min(bx2);
        let cy2 = dy2.min(by2);

        if cx1 >= cx2 || cy1 >= cy2 { return false; }

        let dw = dx2 - dx1;
        let dh = dy2 - dy1;

        let clip_l = (cx1 - dx1) / dw;
        let clip_t = (cy1 - dy1) / dh;
        let clip_r = (dx2 - cx2) / dw;
        let clip_b = (dy2 - cy2) / dh;

        let [u, v, u2, v2] = uv;
        let uw = u2 - u;
        let uh = v2 - v;

        let pos: [f32; 4] = [cx1, cy1, cx2, cy2];
        let cropped_uv: [f32; 4] = [
            u  + clip_l * uw,
            v  + clip_t * uh,
            u2 - clip_r * uw,
            v2 - clip_b * uh,
        ];

        self.get_texture_renderer().add_quad(pos, cropped_uv);
        return true;
    }

    pub fn render_texture_within_pos_option(&mut self, texture: Texture, draw_pos: [f32; 4], bounds_pos: Option<[f32; 4]>) {
        if let Some(bounds) = bounds_pos {
            self.render_texture_within_pos(texture, draw_pos, bounds);
        }
        else {
            self.render_texture(texture, draw_pos);
        }
    }

    


    
    //=================================================
    // Block Rendering
    //=================================================
    pub fn render_block_triangle(&mut self, block : BlockTexture, triangle : BlockTriangle, draw_location : [f32; 2], scale : f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_block_triangle_uv(triangle, block);

        let pos = [
            draw_location[0] - self.cached_expander,         // x1 (left)
            draw_location[1] - self.cached_expander,         // y1 (top/bottom) 
            draw_location[0] + (scale) + self.cached_expander, // x2 (right)
            draw_location[1] + (scale) + self.cached_expander, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }

    pub fn render_block_triangle_with_pos(&mut self, block : BlockTexture, triangle : BlockTriangle, pos : [f32; 4]) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_block_triangle_uv(triangle, block);

        self.get_texture_renderer().add_quad(pos, uv);
    }

    pub fn render_block(&mut self, block : BlockTexture, draw_location : [f32; 2], scale : f32) { 
        let double_scale = scale * 2.0;
        // Right side
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_block_uv(block);

        let pos = [
            draw_location[0],         // x1 (left)
            draw_location[1],         // y1 (top/bottom) 
            draw_location[0] + double_scale, // x2 (right)
            draw_location[1] + double_scale, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }

    pub fn render_block_with_pos(&mut self, block : BlockTexture, pos : [f32; 4]) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_block_uv(block);
        self.get_texture_renderer().add_quad(pos, uv);
    }

    //=====================================
    // Shader Rendering
    //=====================================
    pub fn render_shader_triangle(&mut self, shader : BlockShader, triangle : ShaderTriangle, draw_location : [f32; 2], scale : f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_shader_triangle_uv(triangle, shader);

        let pos = [
            draw_location[0] - self.cached_expander,         // x1 (left)
            draw_location[1] - self.cached_expander,         // y1 (top/bottom) 
            draw_location[0] + (scale) + self.cached_expander, // x2 (right)
            draw_location[1] + (scale) + self.cached_expander, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }

    //=====================================
    // Font Rendering
    //=====================================
    pub fn render_char(&mut self, font: FontType, char : CharType, draw_location : [f32; 2], scale : f32) {
        if char == CharType::CharSpace {
            return; // Don't render spaces
        }
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_font_uv(font, char);

        let pos = [
            draw_location[0],         // x1 (left)
            draw_location[1],         // y1 (top/bottom) 
            draw_location[0] + scale, // x2 (right)
            draw_location[1] + scale, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }

    //=====================================
    // UI element
    //=====================================
    
    // Main UI
    pub fn render_ui_element(&mut self, ui_texture: UITextures, draw_location: [f32; 2], scale: f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_ui_uv(ui_texture);

        // Modify the y value based off texture src rect | Could cache this
        let texture_src_rect = ui_texture.get_rect();
        let y_to_x_scale = texture_src_rect [3] as f32 / texture_src_rect[2] as f32;
        let y_scale = scale * y_to_x_scale;

        let pos = [
            draw_location[0],         // x1 (left)
            draw_location[1],         // y1 (top/bottom) 
            draw_location[0] + scale, // x2 (right)
            draw_location[1] + y_scale, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }
    pub fn render_ui_element_with_pos(&mut self, ui_texture: UITextures, pos: [f32; 4]) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_ui_uv(ui_texture);
        self.get_texture_renderer().add_quad(pos, uv);
    }


    //=====================================
    // Drone UI
    //=====================================

    // Drone UI
    pub fn render_drone_ui_element(&mut self, drone_ui_texture: DroneUITexture, draw_location: [f32; 2], scale: f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_drone_ui_uv(drone_ui_texture);

        // Modify the y value based off texture src rect | Could cache this
        let texture_src_rect = drone_ui_texture.ui_texture_to_sprite_sheet_src_rect();
        let y_to_x_scale = texture_src_rect [3] as f32 / texture_src_rect[2] as f32;
        let y_scale = scale * y_to_x_scale;

        let pos = [
            draw_location[0],         // x1 (left)
            draw_location[1],         // y1 (top/bottom) 
            draw_location[0] + scale, // x2 (right)
            draw_location[1] + y_scale, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }
    pub fn render_drone_ui_element_centered(&mut self, drone_ui_texture: DroneUITexture, draw_location: [f32; 2], scale: f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_drone_ui_uv(drone_ui_texture);

        // Modify the y value based off texture src rect | Could cache this
        let texture_src_rect = drone_ui_texture.ui_texture_to_sprite_sheet_src_rect();
        let y_to_x_scale = texture_src_rect [3] as f32 / texture_src_rect[2] as f32;
        let y_scale = scale * y_to_x_scale;

        let draw_centered_location = [draw_location[0] - (scale / 2.0), draw_location[1] - (y_scale / 2.0)];

        let pos = [
            draw_centered_location[0],         // x1 (left)
            draw_centered_location[1],         // y1 (top/bottom) 
            draw_centered_location[0] + scale, // x2 (right)
            draw_centered_location[1] + y_scale, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }
    pub fn render_drone_ui_element_with_pos(&mut self, drone_ui_texture: DroneUITexture, pos: [f32; 4]) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_drone_ui_uv(drone_ui_texture);
        self.get_texture_renderer().add_quad(pos, uv);
    }


    //=====================================
    // Drone Item
    //=====================================

    // Drone Items
    pub fn render_drone_item(&mut self, drone_item_texure: DroneItemTexture, draw_location: [f32; 2], scale: f32) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_drone_item_uv(drone_item_texure);

        let pos = [
            draw_location[0],         // x1 (left)
            draw_location[1],         // y1 (top/bottom) 
            draw_location[0] + scale, // x2 (right)
            draw_location[1] + scale, // y2 (bottom/top)
        ];

        self.get_texture_renderer().add_quad(pos, uv);
    }
    pub fn render_drone_item_with_pos(&mut self, drone_item_texture: DroneItemTexture, pos: [f32; 4]) {
        let uv = self.texture_atlas.as_ref().unwrap().get_precalculated_drone_item_uv(drone_item_texture);
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
