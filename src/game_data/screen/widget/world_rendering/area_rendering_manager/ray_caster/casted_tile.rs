
use crate::game_data::{
    TextureManager, World, screen::{iso_cord_tool, widget::world_rendering::area_rendering_manager::ray_caster::{casted_triangle::CastedTriangle, ray_casting_config::{Direction, RayCastingConfig}}}, texture_manager::texture_cashe::texture_cashe::CashedTextureID, types::{
        BlockShader, BlockTexture, BlockTriangle, ShaderTriangle
    }
};

#[derive(Clone)]
pub struct CastedTile {
    start_cords: [i32; 3],
    area_cords: [i32; 3],

    left_triangle: CastedTriangle,
    right_triangle: CastedTriangle,
}

impl CastedTile {
    pub fn new_with_triangles(
        left_triangle: CastedTriangle,
        right_triangle: CastedTriangle,
    ) -> CastedTile {
        CastedTile {
            // Input
            start_cords: [0; 3],
            area_cords: [0; 3],

            // Output
            left_triangle: left_triangle,
            right_triangle: right_triangle,
        }
    }

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

    pub fn struck(&self) -> bool {
        self.left_triangle.has_struck_solid || self.right_triangle.has_struck_solid
    }

    pub fn both_struck(&self) -> bool {
        self.left_triangle.has_struck_solid && self.right_triangle.has_struck_solid
    }

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

    pub fn set_left_triangle(&mut self, triangle: CastedTriangle) {
        self.left_triangle = triangle;
    }

    pub fn set_right_triangle(&mut self, triangle: CastedTriangle) {
        self.right_triangle = triangle;
    }

    pub fn get_triangles_depths(&self) -> [i32; 2] {
        let left_max = self.left_triangle.get_solid_struck_depth();
        let right_max = self.right_triangle.get_solid_struck_depth();

        [left_max, right_max]

    }



    //=====================================
    // Ray Casting
    //=====================================
    
    fn cast_left_branch(&mut self, world: &World, ray_casting_config: &RayCastingConfig, current_cords: [i32; 3]) {
        let mut left_current_cords = current_cords;
        let left_triangle = &mut self.left_triangle;

        // x — right-facing surface; shade it for North/West views
        left_current_cords[0] -= ray_casting_config.direction[0];
        if !left_triangle.has_struck_solid {
            left_triangle.handle_current_block(world, ray_casting_config, left_current_cords, BlockTriangle::RightTop);
            if left_triangle.has_struck_solid
                && (ray_casting_config.camera_direction == Direction::North
                    || ray_casting_config.camera_direction == Direction::West)
            {
                left_triangle.set_shader(ShaderTriangle::RightTop, BlockShader::Grey);
            }
        }

        // y
        left_current_cords[1] -= ray_casting_config.direction[1];
        if !left_triangle.has_struck_solid {
            left_triangle.handle_current_block(world, ray_casting_config, left_current_cords, BlockTriangle::LeftBot);
        }

        // z
        if !left_triangle.has_struck_solid {
            left_current_cords[2] -= ray_casting_config.direction[2];
            left_triangle.handle_current_block(world, ray_casting_config, left_current_cords, BlockTriangle::TopLeft);
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

        // x — right-facing surface; shade it for North/West views
        right_current_cords[0] -= ray_casting_config.direction[0];
        if !right_triangle.has_struck_solid {
            right_triangle.handle_current_block(world, ray_casting_config, right_current_cords, BlockTriangle::RightBot);
            if right_triangle.has_struck_solid
                && (ray_casting_config.camera_direction == Direction::North
                    || ray_casting_config.camera_direction == Direction::West)
            {
                right_triangle.set_shader(ShaderTriangle::RightBot, BlockShader::Grey);
            }
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
    // Shadow Casting
    //=====================================

    fn cast_right_shadow(&mut self, world: &World, ray_casting_config: &RayCastingConfig) {
        if !self.right_triangle.has_struck_solid {
            return;
        }
        let mut current_cords = self.right_triangle.get_solid_block_struck_cords();
        let last_texture = self.right_triangle.get_last_texture();
        let direction = &ray_casting_config.camera_direction;
        let mut draw_distance = ray_casting_config.shadow_draw_distance;

        if last_texture == BlockTriangle::TopRight {
            while draw_distance > 0 {
                draw_distance -= 1;

                // z++
                current_cords[2] += 1;
                let block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    self.right_triangle.set_shader(ShaderTriangle::TopRight, BlockShader::Grey);
                    break;
                }

                // y+1 side check
                let block = BlockTexture::from_id(world.get_world_value([current_cords[0], current_cords[1] + 1, current_cords[2]]));
                if block.is_opaque() {
                    if *direction == Direction::South {
                        let cur = self.right_triangle.get_shader_triangle();
                        if cur == ShaderTriangle::TopBotRight {
                            self.right_triangle.set_shader(ShaderTriangle::TopRight, BlockShader::Grey);
                        } else {
                            self.right_triangle.set_shader(ShaderTriangle::TopTopRight, BlockShader::Grey);
                        }
                    } else if *direction == Direction::West {
                        self.right_triangle.set_shader(ShaderTriangle::TopRight, BlockShader::Grey);
                    } else {
                        let cur = self.right_triangle.get_shader_triangle();
                        if cur == ShaderTriangle::TopTopRight {
                            self.right_triangle.set_shader(ShaderTriangle::TopRight, BlockShader::Grey);
                        } else {
                            self.right_triangle.set_shader(ShaderTriangle::TopBotRight, BlockShader::Grey);
                        }
                    }
                }

                // x--
                current_cords[0] -= 1;
                let block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    let cur = self.right_triangle.get_shader_triangle();
                    if cur == ShaderTriangle::TopBotRight {
                        self.right_triangle.set_shader(ShaderTriangle::TopRight, BlockShader::Grey);
                        break;
                    } else if *direction == Direction::South {
                        self.right_triangle.set_shader(ShaderTriangle::TopBotRight, BlockShader::Grey);
                    } else if *direction == Direction::West {
                        // no change
                    } else {
                        self.right_triangle.set_shader(ShaderTriangle::TopTopRight, BlockShader::Grey);
                    }
                }

                // diagonal y++
                current_cords[1] += 1;
                let block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    self.right_triangle.set_shader(ShaderTriangle::TopRight, BlockShader::Grey);
                    break;
                }
            }
        }

        if last_texture == BlockTriangle::LeftTop {
            while draw_distance > 0 {
                draw_distance -= 1;

                current_cords[0] -= 1;
                current_cords[1] += 1;
                let block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    self.right_triangle.set_shader(ShaderTriangle::LeftCenterLeft, BlockShader::Grey);
                    current_cords[2] += 1;
                    let block = BlockTexture::from_id(world.get_world_value(current_cords));
                    if block.is_opaque() {
                        self.right_triangle.set_shader(ShaderTriangle::LeftTop, BlockShader::Grey);
                        break;
                    }
                    current_cords[2] -= 1;
                }
                current_cords[2] += 1;
            }
        }
    }

    fn cast_left_shadow(&mut self, world: &World, ray_casting_config: &RayCastingConfig) {
        if !self.left_triangle.has_struck_solid {
            return;
        }
        let mut current_cords = self.left_triangle.get_solid_block_struck_cords();
        let last_texture = self.left_triangle.get_last_texture();
        let direction = &ray_casting_config.camera_direction;
        let direction_mods = ray_casting_config.direction;
        let mut draw_distance = ray_casting_config.shadow_draw_distance;

        if last_texture == BlockTriangle::TopLeft {
            while draw_distance > 0 {
                draw_distance -= 1;

                // z++
                current_cords[2] += 1;
                let block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    self.left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShader::Grey);
                    break;
                }

                // y+1 side check
                let block = BlockTexture::from_id(world.get_world_value([current_cords[0], current_cords[1] + 1, current_cords[2]]));
                if block.is_opaque() {
                    if *direction == Direction::South {
                        let cur = self.left_triangle.get_shader_triangle();
                        if cur == ShaderTriangle::TopBotLeft {
                            self.left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShader::Grey);
                        } else {
                            self.left_triangle.set_shader(ShaderTriangle::TopTopLeft, BlockShader::Grey);
                        }
                    } else if *direction == Direction::West {
                        // no change
                    } else {
                        let cur = self.left_triangle.get_shader_triangle();
                        if cur == ShaderTriangle::TopTopLeft {
                            self.left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShader::Grey);
                        } else {
                            self.left_triangle.set_shader(ShaderTriangle::TopBotLeft, BlockShader::Grey);
                        }
                    }
                }

                // x--
                current_cords[0] -= 1;
                let block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    let cur = self.left_triangle.get_shader_triangle();
                    if cur == ShaderTriangle::TopBotLeft {
                        self.left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShader::Grey);
                        break;
                    } else if direction_mods[0] == -1 && direction_mods[1] == -1 {
                        self.left_triangle.set_shader(ShaderTriangle::TopBotLeft, BlockShader::Grey);
                    } else if *direction == Direction::West {
                        self.left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShader::Grey);
                    } else {
                        self.left_triangle.set_shader(ShaderTriangle::TopTopLeft, BlockShader::Grey);
                    }
                }

                // diagonal y++
                current_cords[1] += 1;
                let block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    self.left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShader::Grey);
                    break;
                }
            }
        }

        if last_texture == BlockTriangle::LeftBot {
            while draw_distance > 0 {
                draw_distance -= 1;

                current_cords[0] -= 1;
                current_cords[1] += 1;
                let block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    self.left_triangle.set_shader(ShaderTriangle::LeftCenterBot, BlockShader::Grey);
                    current_cords[2] += 1;
                    let block = BlockTexture::from_id(world.get_world_value(current_cords));
                    if block.is_opaque() {
                        self.left_triangle.set_shader(ShaderTriangle::LeftBot, BlockShader::Grey);
                        break;
                    }
                    current_cords[2] -= 1;
                }
                current_cords[2] += 1;
            }
        }
    }

    pub fn cast_shadows(&mut self, world: &World, ray_casting_config: &RayCastingConfig) {
        self.cast_left_shadow(world, ray_casting_config);
        self.cast_right_shadow(world, ray_casting_config);
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
        if self.left_triangle.has_shader() {
            texture_manager.render_shader_triangle(
                self.left_triangle.get_shader_type(),
                self.left_triangle.get_shader_triangle(),
                [draw_cords[0], draw_cords[1]],
                draw_block_scale,
            );
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
        if self.right_triangle.has_shader() {
            texture_manager.render_shader_triangle(
                self.right_triangle.get_shader_type(),
                self.right_triangle.get_shader_triangle(),
                [draw_cords[0] + draw_block_scale, draw_cords[1]],
                draw_block_scale,
            );
        }
    }

    pub fn render_left_triangle(&self, texture_manager: &mut TextureManager, draw_block_scale: f32, draw_offset: [f32; 2]) {
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
        if self.left_triangle.has_shader() {
            texture_manager.render_shader_triangle(
                self.left_triangle.get_shader_type(),
                self.left_triangle.get_shader_triangle(),
                [draw_cords[0], draw_cords[1]],
                draw_block_scale,
            );
        }
    }

    pub fn render_right_triangle(&self, texture_manager: &mut TextureManager, draw_block_scale: f32, draw_offset: [f32; 2]) {
        let flattened_cords = iso_cord_tool::flatten_world_cords(self.get_world_cords());
        let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(draw_block_scale, flattened_cords);

        draw_cords[0] += draw_offset[0];
        draw_cords[1] += draw_offset[1];

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
        if self.right_triangle.has_shader() {
            texture_manager.render_shader_triangle(
                self.right_triangle.get_shader_type(),
                self.right_triangle.get_shader_triangle(),
                [draw_cords[0] + draw_block_scale, draw_cords[1]],
                draw_block_scale,
            );
        }
    }

    pub fn render_to_cashed_texture(
        &self,
        texture_manager: &mut TextureManager,
        cashed_texture_id: CashedTextureID,
        draw_block_scale: f32,
        draw_offset: [f32; 2]
    ){
        let left_textures = self.get_left_triangle().get_textures().clone();
        let left_pos = [
            draw_offset[0],
            draw_offset[1],
            draw_offset[0] + draw_block_scale,
            draw_offset[1] + draw_block_scale,
        ];
        for texture in left_textures {
            texture_manager.render_to_cashed_texture(cashed_texture_id, texture, left_pos);
        }
        if self.left_triangle.has_shader() {
            texture_manager.render_shader_triangle_to_cashed_texture(
                cashed_texture_id,
                self.left_triangle.get_shader_type(),
                self.left_triangle.get_shader_triangle(),
                [draw_offset[0], draw_offset[1]],
                draw_block_scale,
            );
        }

        let right_textures = self.get_right_triangle().get_textures().clone();
        let right_pos = [
            draw_offset[0] + draw_block_scale,
            draw_offset[1],
            draw_offset[0] + (draw_block_scale * 2.0),
            draw_offset[1] + draw_block_scale,
        ];
        for texture in right_textures {
            texture_manager.render_to_cashed_texture(cashed_texture_id, texture, right_pos);
        }
        if self.right_triangle.has_shader() {
            texture_manager.render_shader_triangle_to_cashed_texture(
                cashed_texture_id,
                self.right_triangle.get_shader_type(),
                self.right_triangle.get_shader_triangle(),
                [draw_offset[0] + draw_block_scale, draw_offset[1]],
                draw_block_scale,
            );
        }
    }

    
}