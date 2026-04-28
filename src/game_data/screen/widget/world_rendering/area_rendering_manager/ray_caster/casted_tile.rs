
use crate::game_data::{
    World, 
    screen::widget::world_rendering::area_rendering_manager::ray_caster::{casted_triangle::CastedTriangle, ray_casting_config::RayCastingConfig}, 
    texture_manager::texture::Texture, 
    types::{
        BlockTexture, BlockTriangle
    }
};


struct RaySide {
    textures: Vec<Texture>,
    struck: bool,

    block_struck: BlockTexture,
    block_triangle_struck: BlockTriangle,

    cords_struck: [i32; 3],
}

impl RaySide {
    fn new() -> RaySide {
        RaySide {
            textures: Vec::new(), 
            struck: false, 
            block_struck: BlockTexture::Air,
            block_triangle_struck: BlockTriangle::LeftBot,
            
            cords_struck: [0; 3],
        }
    }

    pub fn check_block(&mut self, block_to_check: BlockTexture, triangle: BlockTriangle) -> bool {
        if block_to_check.is_visible() {
            self.textures.insert(0, Texture::BlockTriangle(block_to_check, triangle));
            if block_to_check.is_opaque() {
                self.struck = true;
                self.block_triangle_struck = triangle;
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
                    self.cords_struck = current_cords;
                    return true;
                }
            }
        }

        // World Block
        let block_to_check = BlockTexture::from_id(world.get_world_value(current_cords));
        if self.check_block(block_to_check, triangle) {
            self.block_struck = block_to_check;
            self.cords_struck = current_cords;
            return true;
        }

        // Underlay Lair
        let lair_block_to_check = ray_casting_config.lair_manager.get_lair_block_at_cords(current_cords);
        if let Some(lair_block) = lair_block_to_check {
            for texture in lair_block.get_underlay_textures() {
                if self.check_block(*texture, triangle) {
                    self.cords_struck = current_cords;
                    return true;
                }
            }
        }

        return false;
    }

    pub fn simple_current_block(&mut self, world: &World, current_cords: [i32; 3], triangle: BlockTriangle) -> bool {
        // World Block
        let block_to_check = BlockTexture::from_id(world.get_world_value(current_cords));
        if self.check_block(block_to_check, triangle) {
            self.block_struck = block_to_check;
            self.cords_struck = current_cords;
            return true;
        }

        false
    }

    pub fn get_block_struck(&self) -> BlockTexture {
        self.block_struck
    }

    pub fn get_textures(self) -> Vec<Texture> {
        return self.textures;
    }

    pub fn get_block_triangle(&self) -> BlockTriangle {
        self.block_triangle_struck
    } 
}

pub struct CastedTile {
    start_cords: [i32; 3],
    area_cords: [i32; 3],

    left_triangle: CastedTriangle,
    right_triangle: CastedTriangle,
}


impl CastedTile {
    pub fn new(
        start_cords:[i32; 3], 
        area_cords: [i32; 3],
    ) -> CastedTile {
        CastedTile {
            // Input
            start_cords,
            area_cords,

            // Output
            left_triangle: CastedTriangle::new(),
            right_triangle: CastedTriangle::new(),

        }
    }

    pub fn get_area_cords(&self) -> [i32; 3] {
        self.area_cords
    }

    pub fn get_left_triangle(&self) -> &CastedTriangle {
        &self.left_triangle
    }

    pub fn get_right_triangle(&self) -> &CastedTriangle {
        &self.right_triangle
    }


    fn cast_left_branch(&mut self, world: &World, ray_casting_config: &RayCastingConfig, current_cords: [i32; 3]) {
        let mut left_current_cords = current_cords;
        let left_triangle = &mut self.left_triangle;
        
        // x
        left_current_cords[0] -= ray_casting_config.direction[0];
        if !left_triangle.has_struck_solid {
            left_triangle.handle_current_block(world, ray_casting_config, left_current_cords, BlockTriangle::RightTop);
        }

        // y
        left_current_cords[1] -= ray_casting_config.direction[1];
        if !left_triangle.has_struck_solid {
            left_triangle.handle_current_block(world, ray_casting_config, left_current_cords, BlockTriangle::LeftBot);
        }
        // z

        if !left_triangle.has_struck_solid {
        left_current_cords[2] -= ray_casting_config.direction[2];
            left_triangle.handle_current_block(world, ray_casting_config,left_current_cords, BlockTriangle::TopLeft);
        }
    }

    fn cast_right_branch(&mut self, world: &World, ray_casting_config: &RayCastingConfig, current_cords: [i32; 3]) {
        let mut right_current_cords = current_cords;
        let right_triangle = &mut self.right_triangle;

        // y
        right_current_cords[1] -= ray_casting_config.direction[1];
        if !right_triangle.has_struck_solid {
            right_triangle.handle_current_block(world, ray_casting_config, right_current_cords, BlockTriangle::LeftTop);
        }

        // x
        right_current_cords[0] -= ray_casting_config.direction[0];
        if !right_triangle.has_struck_solid {
            right_triangle.handle_current_block(world, ray_casting_config, right_current_cords, BlockTriangle::RightBot);
        }
        // z
        right_current_cords[2] -= ray_casting_config.direction[2];
        if !right_triangle.has_struck_solid {
            right_triangle.handle_current_block(world, ray_casting_config, right_current_cords, BlockTriangle::TopRight);
        }
    }

    pub fn cast(&mut self, world: &World, ray_casting_config: &RayCastingConfig) {
        let mut current_cords = self.start_cords;
        
        for _ in 0..ray_casting_config.view_distance {
            if !self.left_triangle.has_struck_solid { 
                self.cast_left_branch(world, ray_casting_config, current_cords);
            }
            if !self.right_triangle.has_struck_solid {
                self.cast_right_branch(world, ray_casting_config, current_cords);
            }
            if self.left_triangle.has_struck_solid && self.right_triangle.has_struck_solid {
                return;
            }

            current_cords[0] -= ray_casting_config.direction[0];
            current_cords[1] -= ray_casting_config.direction[1];
            current_cords[2] -= ray_casting_config.direction[2];
        }
    }

}