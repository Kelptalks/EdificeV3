use std::collections::HashMap;
use miniquad::*;

use crate::game_data::screen::ScreenData;

use super::texture_mesh::{MeshVertex, TextureMesh, TextureMeshId};

const MAX_MESH_QUADS: usize = 100_000;
const MAX_MESH_VERTICES: usize = MAX_MESH_QUADS * 4;
const MAX_MESH_INDICES: usize = MAX_MESH_QUADS * 6;

#[repr(C)]
struct MeshUniforms {
    offset: [f32; 2],
    scale: f32,
    alpha: f32,
}

pub struct MeshManager {
    pipeline: Pipeline,
    vertex_buffer: BufferId,
    index_buffer: BufferId,

    meshes: HashMap<TextureMeshId, TextureMesh>,
    draw_queue: Vec<TextureMeshId>,
}

impl MeshManager {
    pub fn new(ctx: &mut GlContext) -> Self {
        let shader = Self::create_shader(ctx);
        let pipeline = Self::create_pipeline(ctx, shader);

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<MeshVertex>(MAX_MESH_VERTICES),
        );
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<u32>(MAX_MESH_INDICES),
        );

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            meshes: HashMap::new(),
            draw_queue: Vec::new(),
        }
    }

    //=====================================
    // Mesh Management
    //=====================================

    pub fn new_mesh(&mut self) -> TextureMeshId {
        let id = TextureMeshId::next();
        self.meshes.insert(id, TextureMesh::new(id));
        id
    }

    pub fn get_mut_mesh(&mut self, id: TextureMeshId) -> Option<&mut TextureMesh> {
        self.meshes.get_mut(&id)
    }

    pub fn remove_mesh(&mut self, id: TextureMeshId) {
        self.meshes.remove(&id);
    }

    pub fn queue_draw(&mut self, id: TextureMeshId) {
        self.draw_queue.push(id);
    }

    //=====================================
    // Flush
    //=====================================

    pub fn flush(&mut self, ctx: &mut GlContext, screen_data: &ScreenData) {
        if self.draw_queue.is_empty() { return; }

        ctx.begin_default_pass(PassAction::Nothing);
        screen_data.apply_port(ctx);
        ctx.apply_pipeline(&self.pipeline);

        for id in self.draw_queue.drain(..) {
            let Some(mesh) = self.meshes.get(&id) else { continue };
            if mesh.vertices.is_empty() { continue; }
            let Some(src_texture) = mesh.src_texture else { continue; };

            ctx.buffer_update(self.vertex_buffer, BufferSource::slice(&mesh.vertices));
            ctx.buffer_update(self.index_buffer, BufferSource::slice(&mesh.indices));

            ctx.apply_bindings(&Bindings {
                vertex_buffers: vec![self.vertex_buffer],
                index_buffer: self.index_buffer,
                images: vec![src_texture],
            });
            ctx.apply_uniforms(UniformsSource::table(&MeshUniforms {
                offset: mesh.offset,
                scale: mesh.scale,
                alpha: mesh.alpha,
            }));
            ctx.draw(0, mesh.indices.len() as i32, 1);
        }

        ctx.end_render_pass();
    }

    //=====================================
    // Shader / Pipeline
    //=====================================

    fn create_shader(ctx: &mut GlContext) -> ShaderId {
        let vertex_shader = r#"
            #version 100
            attribute vec2 position;
            attribute vec2 texcoord;
            attribute vec3 a_tint;
            attribute float a_alpha;
            varying lowp vec2 uv;
            varying lowp vec3 tint;
            varying lowp float v_alpha;
            uniform vec2 u_offset;
            uniform float u_scale;

            void main() {
                vec2 transformed = position * u_scale + u_offset;
                gl_Position = vec4(transformed.x, -transformed.y, 0.0, 1.0);
                uv = texcoord;
                tint = a_tint;
                v_alpha = a_alpha;
            }
        "#;

        let fragment_shader = r#"
            #version 100
            precision mediump float;
            varying lowp vec2 uv;
            varying lowp vec3 tint;
            varying lowp float v_alpha;
            uniform sampler2D tex;
            uniform lowp float u_alpha;

            void main() {
                vec4 color = texture2D(tex, uv);
                color.a *= v_alpha * u_alpha;
                color.rgb *= tint;
                gl_FragColor = color;
            }
        "#;

        ctx.new_shader(
            ShaderSource::Glsl { vertex: vertex_shader, fragment: fragment_shader },
            ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![
                        UniformDesc::new("u_offset", UniformType::Float2),
                        UniformDesc::new("u_scale", UniformType::Float1),
                        UniformDesc::new("u_alpha", UniformType::Float1),
                    ],
                },
            },
        ).unwrap()
    }

    fn create_pipeline(ctx: &mut GlContext, shader: ShaderId) -> Pipeline {
        ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("position", VertexFormat::Float2),
                VertexAttribute::new("texcoord", VertexFormat::Float2),
                VertexAttribute::new("a_tint", VertexFormat::Float3),
                VertexAttribute::new("a_alpha", VertexFormat::Float1),
            ],
            shader,
            PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                alpha_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
        )
    }
}
