use miniquad::TextureId;

pub struct CashedTileMapTexture {
    src_texture_sheet: TextureId,
    uv: [f32; 4]
}

impl CashedTileMapTexture {
    pub fn new(src_texture_sheet: TextureId, uv: [f32; 4]) -> CashedTileMapTexture {
        CashedTileMapTexture {
            src_texture_sheet,
            uv
        }
    }
}