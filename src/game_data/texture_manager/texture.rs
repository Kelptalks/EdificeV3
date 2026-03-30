use crate::game_data::types::{BlockShader, BlockTexture, BlockTriangle, DroneItemTexture, UITextures};

#[derive(Clone)]
pub enum Texture {
    // Blocks
    BlockTexture(BlockTexture),
    BlockTriangle(BlockTexture, BlockTriangle),
    BlockShader(BlockShader),

    DroneItemTexture(DroneItemTexture),
    UITexture(UITextures),

}