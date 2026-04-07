
use crate::game_data::{
    World, 
    screen::widget::world_rendering::area_rendering_manager::{
        ray_caster::ray_casting_config::RayCastingConfig
    }, 
    texture_manager::texture::Texture, 
    types::{
        BlockTexture, BlockTriangle
    }
};


struct RaySide {
    textures: Vec<Texture>,
    struck: bool,
}

impl RaySide {
    fn new() -> RaySide {
        RaySide {
            textures: Vec::new(), 
            struck: false, 
        }
    }

    pub fn check_block(&mut self, block_to_check: BlockTexture, triangle: BlockTriangle) -> bool{
        if block_to_check.is_visible() {
            self.textures.insert(0, Texture::BlockTriangle(block_to_check, triangle));
            if block_to_check.is_opaque() {
                self.struck = true;
                return true;
            }
        }
        return false;
    }

    #[inline]
    pub fn handle_current_block(&mut self, world: &World, ray_casting_config: &RayCastingConfig, current_cords: [i32; 3], triangle: BlockTriangle) -> bool {
        if !ray_casting_config.world_area.cords_in_area(current_cords) {
            return false;
        }
        
        // Overlay Lair
        let lair_block_to_check = ray_casting_config.lair_manager.get_lair_block_at_cords(current_cords);
        if let Some(lair_block) = lair_block_to_check {
            for texture in lair_block.get_overlay_textures() {
                if self.check_block(*texture, triangle) {
                    return true;
                }
            }
        }

        // World Block
        let block_to_check = BlockTexture::from_id(world.get_world_value(current_cords));
        if self.check_block(block_to_check, triangle) {
            return true;
        }

        // Underlay Lair
        let lair_block_to_check = ray_casting_config.lair_manager.get_lair_block_at_cords(current_cords);
        if let Some(lair_block) = lair_block_to_check {
            for texture in lair_block.get_underlay_textures() {
                if self.check_block(*texture, triangle) {
                    return true;
                }
            }
        }

        
        return false;

    }


    pub fn get_textures(self) -> Vec<Texture> {
        return self.textures;
    }
}

pub struct TileRay {
    start_cords: [i32; 3],
    area_cords: [i32; 3],

    left_side: RaySide,
    right_side: RaySide,
}


impl TileRay {
    pub fn new(
        start_cords:[i32; 3], 
        area_cords: [i32; 3],
    ) -> TileRay {
        TileRay {
            // Input
            start_cords,
            area_cords,

            // Output
            left_side: RaySide::new(),
            right_side: RaySide::new(),

        }
    }

    pub fn get_area_cords(&self) -> [i32; 3] {
        return self.area_cords;
    }

    pub fn get_tile_textures(self) -> [Vec<Texture>; 2] {
        return [self.left_side.get_textures(), self.right_side.get_textures()];
    }

    fn left_branch(&mut self, world: &World, ray_casting_config: &RayCastingConfig, current_cords: [i32; 3]) {
        let mut left_current_cords = current_cords;
        // x
        left_current_cords[0] -= ray_casting_config.direction[0];
        if self.left_side.handle_current_block(world, ray_casting_config, left_current_cords, BlockTriangle::RightTop) {return;}

        // y
        left_current_cords[1] -= ray_casting_config.direction[1];
        if self.left_side.handle_current_block(world, ray_casting_config, left_current_cords, BlockTriangle::LeftBot) {return;}

        // z
        left_current_cords[2] -= ray_casting_config.direction[2];
        if self.left_side.handle_current_block(world, ray_casting_config,left_current_cords, BlockTriangle::TopLeft) {return;}
    }

    fn right_branch(&mut self, world: &World, ray_casting_config: &RayCastingConfig, current_cords: [i32; 3]) {
        let mut right_current_cords = current_cords;
        // y
        right_current_cords[1] -= ray_casting_config.direction[1];
        if self.right_side.handle_current_block(world, ray_casting_config, right_current_cords, BlockTriangle::LeftTop) {return;}

        // x
        right_current_cords[0] -= ray_casting_config.direction[0];
        if self.right_side.handle_current_block(world, ray_casting_config, right_current_cords, BlockTriangle::RightBot) {return;}

        // z
        right_current_cords[2] -= ray_casting_config.direction[2];
        if self.right_side.handle_current_block(world, ray_casting_config, right_current_cords, BlockTriangle::TopRight) {return;}
    }

    pub fn cast(&mut self, world: &World, ray_casting_config: &RayCastingConfig) {
        let mut current_cords = self.start_cords;
        
        for _ in 0..ray_casting_config.view_distance {
            if !self.left_side.struck { 
                self.left_branch(world, ray_casting_config, current_cords);
            }
            if !self.right_side.struck {
                self.right_branch(world, ray_casting_config, current_cords);
            }
            if self.left_side.struck && self.right_side.struck {
                return;
            }

            current_cords[0] -= ray_casting_config.direction[0];
            current_cords[1] -= ray_casting_config.direction[1];
            current_cords[2] -= ray_casting_config.direction[2];
        }
    }
}