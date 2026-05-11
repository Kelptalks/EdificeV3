use crate::game_data::player_data::game_object::traits::{trait_block::BlockTrait, trait_vision::VisionTrait};

pub enum GameObjectTrait {
    Vision(VisionTrait),
    BlockTrait(BlockTrait),

}
