use crate::game_data::{log_init, texture_manager::{texture_renderer::TextureRenderingManager, texture_atlas::TextureAtlas}, Types::{BlockShaderType, BlockTriangle, BlockType, ShaderTriangle}};
use image::{ImageBuffer, RgbaImage};
use miniquad::*;

static BLOCK_PIXLE_REZ : u32 = 64;
static TRIANGLE_PIXLE_REZ : u32 = BLOCK_PIXLE_REZ/2;

static BLOCK_TEXTURES_PER_ROW : u32 = 20;
static SPLICED_TRIANGLE_DIMENSIONS : u32 = 32;
static TRIANGLES_PER_BLOCK : u32 = 6;

// Expander tuning constants - adjust these to control gap prevention
static EXPANDER_BASE_MULTIPLIER: f32 = 0.005;  // Base scaling with sprite size
static EXPANDER_OFFSET_STRENGTH: f32 = 0.00005; // Additional offset to prevent gaps at low zoom
static EXPANDER_SCALE_THRESHOLD: f32 = 0.1;     // Scale below which offset strength increases


pub struct TextureManager {
    textures_initialized : bool,
    spliced_block_texture_id : Option<TextureId>,
    texture_renderer : Option<TextureRenderingManager>,

    // New Testing
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
            spliced_block_texture_id : None,
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
        // Create spliced block spritesheet
        let spliced_block_texture = Self::init_spliced_block_texture(ctx);
        self.spliced_block_texture_id = Some(spliced_block_texture);
        
        // Create Texutre render manager
        let mut texture_render_manager = TextureRenderingManager::new(ctx);
        texture_render_manager.set_texture(spliced_block_texture);
        self.texture_renderer = Some(texture_render_manager);
        
        // Init Texture atlas
        let mut new_texture_atlas = TextureAtlas::new(ctx);
        self.texture_atlas = Some(new_texture_atlas);

        // Set to to initialized 
        self.textures_initialized = true;
        println!("Created miniquad Texture");
    }

    pub fn init_spliced_block_texture(ctx : &mut GlContext) -> TextureId {   
        let spliced_image_width = SPLICED_TRIANGLE_DIMENSIONS * BlockType::get_total_blocks();
        let spliced_image_height = SPLICED_TRIANGLE_DIMENSIONS * TRIANGLES_PER_BLOCK;

        // Create spliced sprite sheet image for pixle minipulation
        let mut spliced_block_image: RgbaImage = ImageBuffer::new(spliced_image_width, spliced_image_height);

        // Get the block sprite sheet and masking block
        let block_sprite_image = image::open("assets/blocks.png").unwrap().to_rgba8();
        let block_masks_image = image::open("assets/masking_textures.png").unwrap().to_rgba8();

        //Loop through all block locations
        for block_id in 0..BlockType::get_total_blocks() {

            let block_src_x_cor = BLOCK_PIXLE_REZ * (block_id % BLOCK_TEXTURES_PER_ROW);
            let block_src_y_cor = BLOCK_PIXLE_REZ * (block_id / BLOCK_TEXTURES_PER_ROW);

            let x_dest_cor = TRIANGLE_PIXLE_REZ * block_id;

            for y in 0..BLOCK_PIXLE_REZ {
                for x in 0..BLOCK_PIXLE_REZ {
                    // Get masking textures pixle color
                    let source_pixel = block_masks_image.get_pixel(x, y);
                    let [r, g, b, a] = source_pixel.0;  // Gets [u8; 4] array

                    let x_draw_cor = x + x_dest_cor;

                    //Top Left
                    if r == 243 && g == 255 && b == 0 
                    {
                        spliced_block_image.put_pixel(x_draw_cor, y, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Top Right
                    else if r == 7 && g == 255 && b == 0 
                    {
                        spliced_block_image.put_pixel(x_draw_cor - 32, y + 32, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Left Top
                    else if r == 255 && g == 0 && b == 0
                    {
                        spliced_block_image.put_pixel(x_draw_cor, y + 48, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Left Bot
                    else if r == 253 && g == 0 && b == 232 
                    {
                        spliced_block_image.put_pixel(x_draw_cor, y + 64, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Right Top
                    else if r == 0 && g == 193 && b == 255 
                    {
                        spliced_block_image.put_pixel(x_draw_cor - 32, y + 112, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                    //Right Bot
                    else if r == 110 && g == 0 && b == 255 
                    {
                        spliced_block_image.put_pixel(x_draw_cor - 32, y + 128, *block_sprite_image.get_pixel(x + block_src_x_cor, y + block_src_y_cor));
                    }
                }
            }
        }

        //Create microquad texture
        // Your original method with proper filtering
        let rgba_bytes: Vec<u8> = spliced_block_image.clone().into_raw();

        let spliced_block_texture_id: TextureId = ctx.new_texture_from_rgba8(
            spliced_image_width as u16,
            spliced_image_height as u16,
            &rgba_bytes,
        );

        // Set both min and mag filter modes for pixel art
        //ctx.texture_set_min_filter(spliced_block_texture_id, FilterMode::Nearest, MipmapFilterMode::None);
        //ctx.texture_set_mag_filter(spliced_block_texture_id, FilterMode::Nearest);
        
        return spliced_block_texture_id;
    }

    pub fn get_spliced_block_sprite_sheet(&self) -> &TextureId {
        self.spliced_block_texture_id
            .as_ref()
            .unwrap()
    }

    // Testing the rendering of sprites
    pub fn test_sprites(&mut self, ctx : &mut GlContext) {
        if !self.textures_initialized {
            println!("Textures not initialized yet!");
            return;
        }
        
        if let Some(texture_renderer) = self.texture_renderer.as_mut() {
            let scale = 0.2;
            
            if let Some(texture_atlas) = self.texture_atlas.as_mut() {
                texture_renderer.set_texture(texture_atlas.texture_id);
                texture_renderer.add_quad([-1.0, -1.0, 1.0, 1.0], [0.0, 0.0, scale, scale]);

                //let block_uv = texture_atlas.get_block_triangle_uv(BlockTriangle::TopLeft, BlockType::CopperOre);
                //texture_renderer.add_quad([0.0, 0.0, scale, scale], block_uv);
            }
            
        }
        
    }

}