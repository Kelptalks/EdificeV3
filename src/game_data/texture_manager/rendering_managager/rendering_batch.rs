
/*
#################
## RenderBatch ##
#################

A Collection Of verticies with a src and destination texture 


*/

use miniquad::TextureId;

#[repr(C)]
pub struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
    tint: [f32; 3],
}


pub struct RenderBatch {
    // Textures
    pub src_texture: TextureId,
    pub target_texture: Option<TextureId>, // If non to viewport

    // Quads
    pub vertices: Vec<Vertex>, 
    pub indices: Vec<u32>,

    pub alpha: f32,
}

impl RenderBatch {

    pub fn new(src_texture: TextureId) -> RenderBatch {
        RenderBatch {
            src_texture: src_texture,
            target_texture: None,
            vertices: Vec::new(),
            indices: Vec::new(),

            alpha: 1.0
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();

    }

    //=====================================
    // Texture Managment
    //=====================================

    pub fn set_src_texture(&mut self, texture_id: TextureId) {
        self.src_texture = texture_id;
    }

    pub fn set_tartget_texture(&mut self, texture_id: TextureId) {
        self.target_texture = Some(texture_id);
    }

    //=====================================
    // Quads
    //=====================================
    
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

    //=====================================
    // Render
    //=====================================

    

}