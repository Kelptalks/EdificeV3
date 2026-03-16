use crate::game_data::types::{BlockShader, BlockTexture, BlockTriangle, DroneItemTexture, UITextures};

pub enum Texture {
    // Blocks
    BlockTexture(BlockTexture),
    BlockTriangle(BlockTexture, BlockTriangle),
    BlockShader(BlockShader),

    DroneItemTexture(DroneItemTexture),
    UITexture(UITextures),

}