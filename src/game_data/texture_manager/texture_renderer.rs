#![allow(dead_code)]

use miniquad::*;

use crate::game_data::{screen::ScreenData, texture_manager::rendering_managager::rendering_batch::RenderBatch};


// Static vars
const MAX_QUADS_PER_BATCH: usize = 500_000;
const MAX_VERTICES: usize = MAX_QUADS_PER_BATCH * 4;
const MAX_INDICES: usize = MAX_QUADS_PER_BATCH * 6;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
    tint: [f32; 3],
}

#[repr(C)]
struct Uniforms {
    alpha: f32,  // Field name should match shader uniform name (minus "u_")
}

pub struct TextureRenderingManager {
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
        let vertex_shader = r#"
            #version 100
            attribute vec2 position;
            attribute vec2 texcoord;
            attribute vec3 a_tint;
            varying lowp vec2 uv;
            varying lowp vec3 tint;

            void main() {
                gl_Position = vec4(position, 0, 1);
                uv = texcoord;
                tint = a_tint;
            }
        "#;

       let fragment_shader = r#"
            #version 100
            precision mediump float;
            varying lowp vec2 uv;
            varying lowp vec3 tint;
            uniform sampler2D tex;

            void main() {
                vec4 color = texture2D(tex, uv);
                color.rgb *= tint;
                gl_FragColor = color;
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
        let vertex_shader = r#"
            #version 100
            attribute vec2 position;
            attribute vec2 texcoord;
            attribute vec3 a_tint;
            varying lowp vec2 uv;
            varying lowp vec3 tint;

            void main() {
                gl_Position = vec4(position, 0, 1);
                uv = texcoord;
                tint = a_tint;
            }
        "#;

        let fragment_shader = r#"
            #version 100
            precision mediump float;
            varying lowp vec2 uv;
            varying lowp vec3 tint;
            uniform sampler2D tex;
            uniform lowp float u_alpha;

            void main() {
                vec4 color = texture2D(tex, uv);
                color.a *= u_alpha;
                color.rgb *= tint;
                gl_FragColor = color;
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
                VertexAttribute::new("a_tint", VertexFormat::Float3),
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
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
                )),
                ..Default::default()
            },
        );
        return pipeline;
    }

    pub fn add_quad(&mut self, pos: [f32; 4], uv: [f32; 4]) {
        self.add_quad_tinted(pos, uv, [1.0, 1.0, 1.0]);
    }

    pub fn add_quad_tinted(&mut self, pos: [f32; 4], uv: [f32; 4], tint: [f32; 3]) {
        let base_index = self.vertices.len() as u32;

        let new_vertices = vec![
            Vertex { pos: [pos[0], -pos[1]], uv: [uv[0], uv[1]], tint },
            Vertex { pos: [pos[2], -pos[1]], uv: [uv[2], uv[1]], tint },
            Vertex { pos: [pos[2], -pos[3]], uv: [uv[2], uv[3]], tint },
            Vertex { pos: [pos[0], -pos[3]], uv: [uv[0], uv[3]], tint },
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

    pub fn flush_batch(&mut self, ctx: &mut GlContext, screen_data: &ScreenData, batch: &mut RenderBatch) {
        if batch.vertices.is_empty() {
            return;
        }

        // Resolve src texture: use batch's or fall back to renderer's current texture
        let src_texture = batch.src_texture
            .or(self.current_texture).unwrap();

        // Set up render target
        if let Some(target) = batch.target_texture {
            let render_pass = ctx.new_render_pass(target, None);
            ctx.begin_pass(Some(render_pass), PassAction::Nothing);
            
        } else {
            ctx.begin_default_pass(PassAction::Nothing);
            screen_data.apply_port(ctx);
        }

        // Upload batch data to GPU buffers
        ctx.buffer_update(self.vertex_buffer, BufferSource::slice(&batch.vertices));
        ctx.buffer_update(self.index_buffer, BufferSource::slice(&batch.indices));

        let bindings = Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![src_texture],
        };

        // Select pipeline based on batch's alpha
        if batch.alpha == 1.0 {
            ctx.apply_pipeline(&self.opaque_pipeline);
            ctx.apply_bindings(&bindings);
        } else {
            ctx.apply_pipeline(&self.translucent_pipeline);
            ctx.apply_bindings(&bindings);
            ctx.apply_uniforms(UniformsSource::table(&Uniforms { alpha: batch.alpha }));
        }

        ctx.draw(0, batch.indices.len() as i32, 1);
        ctx.end_render_pass();

        batch.clear();
    }

}
