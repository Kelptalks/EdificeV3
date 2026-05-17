#![allow(dead_code)]
use crate::game_data::{World, screen::{iso_cord_tool, widget::world_rendering::area_rendering_manager::ray_caster::ray_casting_config::RayCastingConfig}, texture_manager::texture::Texture, types::{BlockShader, BlockTexture, BlockTriangle, ShaderTriangle}};



#[derive(Clone)]
pub struct CastedTriangle {
    pub has_first_struck: bool,
    pub has_struck_solid: bool,


    first_block_struck_cords: [i32; 3],
    first_block_struck_type: BlockTexture,
    first_block_triangle_struck: BlockTriangle,

    solid_block_cords_struck: [i32; 3],
    solid_block_type_struck: BlockTexture,
    solid_block_triangle_struck: BlockTriangle,

    shader_triangle: ShaderTriangle,
    shader_type: BlockShader,

    // rendering
    draw_pos: [f32; 4],
    textures: Vec<Texture>,
}

impl CastedTriangle {
    pub fn new() -> CastedTriangle {
        CastedTriangle {
            // Ray Data
            has_first_struck: false,
            has_struck_solid: false,

            first_block_struck_cords: [0; 3],
            first_block_struck_type: BlockTexture::Air,
            first_block_triangle_struck: BlockTriangle::LeftBot,


            // rendering
            solid_block_cords_struck: [0; 3],
            solid_block_type_struck: BlockTexture::Air,
            solid_block_triangle_struck: BlockTriangle::LeftBot,

            shader_triangle: ShaderTriangle::TopLeft,
            shader_type: BlockShader::None,

            draw_pos: [0.0; 4],
            textures: Vec::new(),
        }
    }

    #[inline]
    pub fn check_block(
        &mut self, 
        block_cords: [i32; 3], 
        block_to_check: BlockTexture, 
        triangle: BlockTriangle
    ) {
        if block_to_check.is_visible() {
            self.textures.insert(0, Texture::BlockTriangle(block_to_check, triangle));
            if block_to_check.is_opaque() {
                self.has_struck_solid = true;
                self.solid_block_type_struck = block_to_check;
                self.solid_block_triangle_struck = triangle;
                self.solid_block_cords_struck = block_cords;
                
                if !self.has_first_struck {
                    self.has_first_struck = true;

                    self.first_block_triangle_struck = triangle;
                    self.first_block_struck_type = block_to_check;
                    self.first_block_struck_cords = block_cords;
                }
            }
            else if block_to_check.is_translucent() {
                if !self.has_first_struck {
                    self.has_first_struck = true;

                    self.first_block_triangle_struck = triangle;
                    self.first_block_struck_type = block_to_check;
                    self.first_block_struck_cords = block_cords;
                }
            }
        }
    } 

    #[inline]
    pub fn handle_current_block(
        &mut self, 
        world: &World, 
        ray_casting_config: &RayCastingConfig, 
        current_cords: [i32; 3], 
        triangle: BlockTriangle
    ) {
        if !ray_casting_config.world_area.cords_in_area(current_cords) {
            return;
        }
        
        // Overlay Lair
        let lair_block_to_check = ray_casting_config.lair_manager.get_lair_block_at_cords(current_cords);
        if let Some(lair_block) = lair_block_to_check {
            for texture in lair_block.get_overlay_textures() {
                self.check_block(current_cords, *texture, triangle);
            }
        }

        // World Block
        let block_to_check = BlockTexture::from_id(world.get_world_value(current_cords));
        self.check_block(current_cords, block_to_check, triangle);

        // Underlay Lair
        let lair_block_to_check = ray_casting_config.lair_manager.get_lair_block_at_cords(current_cords);
        if let Some(lair_block) = lair_block_to_check {
            for texture in lair_block.get_underlay_textures() {
                self.check_block(current_cords, *texture, triangle);
            }
        }
    }

    
    pub fn get_textures(&self) -> &Vec<Texture>{
        &self.textures
    }

    pub fn get_first_block_cords_struck(&self) -> [i32; 3] {
        return self.first_block_struck_cords;
    }

    pub fn get_solid_block_struck_cords(&self) -> [i32; 3] {
        return self.solid_block_cords_struck;
    }

    pub fn get_solid_struck_depth(&self) -> i32 {
        iso_cord_tool::get_depth_from_world_cords(
            self.get_solid_block_struck_cords()
        )
    }

    pub fn get_solid_block_depth(&self) -> i32 {
        iso_cord_tool::get_depth_from_world_cords(self.solid_block_cords_struck)
    }

    pub fn get_last_texture(&self) -> BlockTriangle {
        self.solid_block_triangle_struck
    }

    pub fn set_shader(&mut self, shader_triangle: ShaderTriangle, shader_type: BlockShader) {
        self.shader_triangle = shader_triangle;
        self.shader_type = shader_type;
    }

    pub fn get_shader_triangle(&self) -> ShaderTriangle {
        self.shader_triangle
    }

    pub fn get_shader_type(&self) -> BlockShader {
        self.shader_type
    }

    pub fn has_shader(&self) -> bool {
        self.shader_type.id() != 0
    }

}



#[cfg(test)]
mod tests {
    use crate::game_data::screen::iso_cord_tool;

    #[test]
    fn test_depth_check() {

        let cords_1 = [10; 3];

        let cords_2 = [5; 3];

        println!("Depth: {}", iso_cord_tool::get_depth_from_world_cords(cords_1));
        println!("Depth 2: {}", iso_cord_tool::get_depth_from_world_cords(cords_2));

    }
}