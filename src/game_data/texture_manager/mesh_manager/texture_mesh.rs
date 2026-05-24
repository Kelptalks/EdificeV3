use std::sync::atomic::{AtomicU32, Ordering};
use miniquad::TextureId;

static NEXT_MESH_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureMeshId {
    id: u32,
}

impl TextureMeshId {
    pub fn next() -> Self {
        Self { id: NEXT_MESH_ID.fetch_add(1, Ordering::Relaxed) }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct MeshVertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub tint: [f32; 3],
    pub alpha: f32,
}

pub struct TextureMesh {
    pub id: TextureMeshId,
    pub depth: i32,

    pub offset: [f32; 2],
    pub scale: f32,
    pub alpha: f32,

    pub src_texture: Option<TextureId>,

    pub(crate) vertices: Vec<MeshVertex>,
    pub(crate) indices: Vec<u32>,
}

impl TextureMesh {
    pub fn new(id: TextureMeshId) -> Self {
        Self {
            id,
            depth: 0,
            offset: [0.0; 2],
            scale: 1.0,
            alpha: 1.0,
            src_texture: None,
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn add_quad(&mut self, pos: [f32; 4], uv: [f32; 4]) {
        self.add_quad_tinted_translucent(pos, uv, [1.0, 1.0, 1.0], 1.0);
    }

    pub fn add_quad_tinted(&mut self, pos: [f32; 4], uv: [f32; 4], tint: [f32; 3]) {
        self.add_quad_tinted_translucent(pos, uv, tint, 1.0);
    }

    pub fn add_quad_translucent(&mut self, pos: [f32; 4], uv: [f32; 4], alpha: f32) {
        self.add_quad_tinted_translucent(pos, uv, [1.0, 1.0, 1.0], alpha);
    }

    pub fn add_quad_tinted_translucent(&mut self, pos: [f32; 4], uv: [f32; 4], tint: [f32; 3], alpha: f32) {
        let base = self.vertices.len() as u32;
        self.vertices.extend([
            MeshVertex { pos: [pos[0], pos[1]], uv: [uv[0], uv[1]], tint, alpha },
            MeshVertex { pos: [pos[2], pos[1]], uv: [uv[2], uv[1]], tint, alpha },
            MeshVertex { pos: [pos[2], pos[3]], uv: [uv[2], uv[3]], tint, alpha },
            MeshVertex { pos: [pos[0], pos[3]], uv: [uv[0], uv[3]], tint, alpha },
        ]);
        self.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}
