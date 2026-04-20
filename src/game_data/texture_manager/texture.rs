use crate::game_data::types::{BlockShader, BlockTexture, BlockTriangle, DroneItemTexture, UITextures};

#[derive(Clone, Copy)]
pub enum Texture {
    // Blocks
    BlockTexture(BlockTexture),
    BlockTriangle(BlockTexture, BlockTriangle),
    BlockShader(BlockShader),

    DroneItemTexture(DroneItemTexture),
    UITexture(UITextures),

    TintedUITexture(UITextures, [f32; 3]),

}

