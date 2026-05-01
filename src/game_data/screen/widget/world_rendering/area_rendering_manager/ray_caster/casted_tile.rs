
use crate::game_data::{
    TextureManager, World, screen::{iso_cord_tool, widget::world_rendering::area_rendering_manager::ray_caster::{casted_triangle::CastedTriangle, ray_casting_config::RayCastingConfig}}, texture_manager::texture::Texture, types::{
        BlockTexture, BlockTriangle
    }
};


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

    //=====================================
    // Getters 
    //=====================================

    pub fn get_world_cords(&self) -> [i32; 3] {
        return self.start_cords
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


    //=====================================
    // Ray Casting
    //=====================================
    
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

    //=====================================
    // Rendering
    //=====================================

    pub fn render(&self, texture_manager: &mut TextureManager, draw_block_scale: f32, draw_offset: [f32; 2]) {
        let flattened_cords = iso_cord_tool::flatten_world_cords(self.get_world_cords());
        let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(draw_block_scale, flattened_cords);
        
        draw_cords[0] += draw_offset[0];
        draw_cords[1] += draw_offset[1];


        let left_textures = self.get_left_triangle().get_textures().clone();
        let left_pos = [
            draw_cords[0],
            draw_cords[1],
            draw_cords[0] + draw_block_scale,
            draw_cords[1] + draw_block_scale,
        ];
        for texture in left_textures {
            texture_manager.render_expanded_texture(texture, left_pos);
        }

        let right_textures = self.get_right_triangle().get_textures().clone();
        let right_pos = [
            draw_cords[0] + draw_block_scale,
            draw_cords[1],
            draw_cords[0] + (draw_block_scale * 2.0),
            draw_cords[1] + draw_block_scale,
        ];
        for texture in right_textures {
            texture_manager.render_expanded_texture(texture, right_pos);
        }
    }

}