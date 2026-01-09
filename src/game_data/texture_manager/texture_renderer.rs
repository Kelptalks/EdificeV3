 use image::imageops::FilterType::Triangle;
use miniquad::*;

use crate::game_data::{logging_tool, types::{BlockTriangle, BlockType}};

// Static vars
const MAX_QUADS_PER_BATCH: usize = 500_000;
const MAX_VERTICES: usize = MAX_QUADS_PER_BATCH * 4;
const MAX_INDICES: usize = MAX_QUADS_PER_BATCH * 6;

// Types
type QuadUVs = [f32; 4];
type TriangleUVs = [QuadUVs; 6];

pub struct UVManager {
    triangles : Vec<TriangleUVs>,
}

static TRIANGLE_PIXLE_REZ : f32 = 32.0;

impl UVManager{
    pub fn new() -> Self {
        let triangles = Self::create_triangle_uvs();
        Self {
            triangles : triangles,
        }
    }

    fn create_triangle_uvs() -> Vec<TriangleUVs> {
        let total_blocks = BlockType::get_total_blocks();
        let triangles_per_block = 6;

        let sprite_sheet_width = TRIANGLE_PIXLE_REZ * total_blocks as f32;
        let sprite_sheet_height = TRIANGLE_PIXLE_REZ * triangles_per_block as f32;
        
        let mut triangle_vec: Vec<TriangleUVs> = Vec::new();

        for current_block in 0..total_blocks {
            let pixel_start_x_cor = TRIANGLE_PIXLE_REZ * current_block as f32 + 0.5;
            let pixel_end_x_cor = (TRIANGLE_PIXLE_REZ * current_block as f32) + TRIANGLE_PIXLE_REZ as f32 - 0.5;

            let mut triangle_uvs: TriangleUVs = [[0.0, 0.0, 0.0, 0.0]; 6];

            for current_triangle in 0..triangles_per_block {
                let pixel_start_y_cor = TRIANGLE_PIXLE_REZ * current_triangle as f32 + 0.5;
                let pixel_end_y_cor = (TRIANGLE_PIXLE_REZ * current_triangle as f32) + TRIANGLE_PIXLE_REZ as f32 - 0.5;
                
                triangle_uvs[current_triangle] = [
                    pixel_start_x_cor / sprite_sheet_width,
                    pixel_start_y_cor / sprite_sheet_height,
                    pixel_end_x_cor / sprite_sheet_width,
                    pixel_end_y_cor / sprite_sheet_height,
                ];
            }
            triangle_vec.push(triangle_uvs);
        }


        logging_tool::log_init("created triangle UVs");


        return triangle_vec;
    }

    pub fn get_block_triangle_uv(&self, block : BlockType, triangle : BlockTriangle) -> [f32; 4] {
        return self.triangles[block.id_as_usize() as usize][triangle.id_as_usize()];
    }

}


#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

#[repr(C)]
struct Uniforms {
    alpha: f32,  // Field name should match shader uniform name (minus "u_")
}

pub struct TextureRenderingManager {
    //SpriteSheet UV manager
    uv_manager : UVManager,
    
    // Shaders
    opaque_shader: ShaderId,
    translucent_shader: ShaderId,

    // Piplines
    opaque_pipeline: Pipeline,
    translucent_pipeline: Pipeline,
    
    // Buffers
    vertex_buffer: BufferId,
    index_buffer: BufferId,

    // Frame Data
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    current_texture: Option<TextureId>,
    
    // Uniforms
    alpha: f32,
    
}

impl TextureRenderingManager {
    pub fn new(ctx : &mut GlContext) -> Self {
        let alpha = 1.0;
        let opaque_shader = Self::create_opaque_shader(ctx);
        let opaque_pipeline = Self::create_pipeline(ctx, opaque_shader);

        let translucent_shader = Self::create_translucent_shader(ctx);
        let translucent_pipeline = Self::create_pipeline(ctx, translucent_shader);

        // Create buffers
        // Create empty vertex buffer
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,  // Use Stream for frequently updated data
            BufferSource::empty::<Vertex>(MAX_VERTICES), // Reserve space for max vertices
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer, 
            BufferUsage::Stream,
            BufferSource::empty::<u32>(MAX_INDICES), // Reserve space for max indices
        );

        Self {
            uv_manager: UVManager::new(), 
            
            // Renderer
            opaque_shader: opaque_shader,
            translucent_shader: translucent_shader,

            
            // Pipelines
            opaque_pipeline: opaque_pipeline,
            translucent_pipeline: translucent_pipeline,

            // Buffers
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,

            // Frame data
            vertices: Vec::new(),
            indices: Vec::new(),
            current_texture: None,

            // Uniforms
            alpha: alpha,
        }
    }

//=========
// Setters
//=========

    pub fn set_translucent(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    pub fn set_texture(&mut self, texture : TextureId) {
        self.current_texture = Some(texture);
    }

//=============================
//Rendering pipeline functions
//=============================

    fn create_opaque_shader(ctx: &mut GlContext) -> ShaderId {
        // Create simple vertex shader
        let vertex_shader = r#"
            #version 100
            attribute vec2 position;
            attribute vec2 texcoord;
            varying lowp vec2 uv;
            
            void main() {
                gl_Position = vec4(position, 0, 1);
                uv = texcoord;
            }
        "#;


       let fragment_shader = r#"
            #version 100
            precision mediump float;
            varying lowp vec2 uv;
            uniform sampler2D tex;

            void main() {
                gl_FragColor = texture2D(tex, uv);  // No alpha multiply
            }
        "#;

        // Create shader
        let shader = ctx.new_shader(
            ShaderSource::Glsl {
                vertex: vertex_shader,
                fragment: fragment_shader,
            },
            ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout { uniforms: vec![] },
            },
        ).unwrap();

        return shader;
    }

    fn create_translucent_shader(ctx: &mut GlContext) -> ShaderId {
        // Create simple vertex shader
        let vertex_shader = r#"
            #version 100
            attribute vec2 position;
            attribute vec2 texcoord;
            varying lowp vec2 uv;
            
            void main() {
                gl_Position = vec4(position, 0, 1);
                uv = texcoord;
            }
        "#;

        let fragment_shader = r#"
            #version 100
            precision mediump float;
            varying lowp vec2 uv;
            uniform sampler2D tex;
            uniform lowp float u_alpha;
            
            void main() {
                vec4 color = texture2D(tex, uv);  // Sample texture
                color.a *= u_alpha;               // Multiply alpha channel by uniform
                gl_FragColor = color;             // Output final color
            }
        "#;

        // Create shader
        let shader = ctx.new_shader(
            ShaderSource::Glsl {
                vertex: vertex_shader,
                fragment: fragment_shader,
            },
            ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![
                        UniformDesc::new("u_alpha", UniformType::Float1),
                    ]
                },
            },
        ).unwrap();

        return shader;
    }

    fn create_pipeline(ctx: &mut GlContext, shader: ShaderId) -> Pipeline {
        // Create pipeline from shader with alpha blending
        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("position", VertexFormat::Float2),
                VertexAttribute::new("texcoord", VertexFormat::Float2),
            ],
            shader,
            PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
                )),
                alpha_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
                )),
                ..Default::default()
            },
        );
        return pipeline;
    }

    pub fn add_quad(&mut self, pos: [f32; 4], uv: [f32; 4]) {
        let base_index = self.vertices.len() as u32;

        // Clone the vertices when extending
        let new_vertices = vec![
            Vertex { pos: [pos[0], -pos[1]], uv: [uv[0], uv[1]] },
            Vertex { pos: [pos[2], -pos[1]], uv: [uv[2], uv[1]] },
            Vertex { pos: [pos[2], -pos[3]], uv: [uv[2], uv[3]] },
            Vertex { pos: [pos[0], -pos[3]], uv: [uv[0], uv[3]] },
        ];

        let new_indices = vec![
            base_index, base_index + 1, base_index + 2,
            base_index, base_index + 2, base_index + 3,
        ];

        self.vertices.extend(new_vertices);
        self.indices.extend(new_indices);
    }

    pub fn flush(&mut self, ctx: &mut GlContext) {
        // If there is nothing to render
        if self.vertices.is_empty() {
            return;
        }

        // Upload data to GPU
        ctx.buffer_update(self.vertex_buffer, BufferSource::slice(&self.vertices));
        ctx.buffer_update(self.index_buffer, BufferSource::slice(&self.indices));

        
        // reasoreses for the GPU
        let bindings = Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![self.current_texture.unwrap()],
        };

        // Give gpu rendering config
        if self.alpha == 1.0 {
            ctx.apply_pipeline(&self.opaque_pipeline);
            ctx.apply_bindings(&bindings);
        }
        else {
            ctx.apply_pipeline(&self.translucent_pipeline);
            ctx.apply_bindings(&bindings);
            ctx.apply_uniforms(UniformsSource::table(&Uniforms { alpha: self.alpha }));
        }

        // Bind recorces to gpu
        ctx.draw(0, self.indices.len() as i32, 1);

        // Clear for next frame
        self.vertices.clear();
        self.indices.clear();
    }

} 