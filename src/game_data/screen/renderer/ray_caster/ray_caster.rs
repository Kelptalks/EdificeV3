use crate::game_data::{World, screen::renderer::camera_data::{CameraData, Direction}, types::{BlockShaderType, BlockTriangle, BlockTexture, ShaderTriangle}};
use super::super::casted_block_manager::casted_triangle::CastedTriangle;
use super::super::casted_block_manager::casted_tile::CastedTile;

//=====================================
// Combined
//=====================================

pub fn raycast_tile_with_shadows(camera_data: &CameraData, world: &World, casted_tile: &mut CastedTile) {
    // Clear tile data
    casted_tile.reset_casting_values();
    
    // Perform the raycasting
    raycast_tile(camera_data, world, casted_tile);
    
    // Then cast shadows for both triangles
    let triangles = casted_tile.get_mut_triangles();
    cast_left_shadow(camera_data, world, triangles[0]);
    cast_right_shadow(camera_data, world, triangles[1]);
}

//=====================================
// Shadow casting
//=====================================

pub fn cast_right_shadow(camera_data: &CameraData, world: &World, right_triangle: &mut CastedTriangle) {
    let solid_struck_cords = right_triangle.get_solid_struck_cords();
    let mut current_cords = solid_struck_cords;
    
    let direction = camera_data.get_direction();
    
    let mut block: BlockTexture;
    let mut draw_distance = camera_data.get_shadow_draw_distance();
    
    // Get the last texture in the list to determine which face was struck
    let last_texture = right_triangle.get_last_texture();
    
    if last_texture == BlockTriangle::TopRight {
        while draw_distance > 0 {
            draw_distance -= 1;
            
            // z++
            current_cords[2] += 1;
            
            block = BlockTexture::from_id(world.get_world_value(current_cords));
            if block.is_opaque() {
                right_triangle.set_shader(ShaderTriangle::TopRight, BlockShaderType::Grey);
                break;
            }
            
            // y++ side
            let temp_y = current_cords[1] + 1;
            block = BlockTexture::from_id(world.get_world_value([current_cords[0], temp_y, current_cords[2]]));
            if block.is_opaque() {
                // South
                if *direction == Direction::South {
                    let current_shader = right_triangle.get_shader_triangle();
                    if current_shader == ShaderTriangle::TopBotRight {
                        right_triangle.set_shader(ShaderTriangle::TopRight, BlockShaderType::Grey);
                    } else {
                        right_triangle.set_shader(ShaderTriangle::TopTopRight, BlockShaderType::Grey);
                    }
                }
                // West
                else if *direction == Direction::West {
                    right_triangle.set_shader(ShaderTriangle::TopRight, BlockShaderType::Grey);
                }
                // North
                else {
                    let current_shader = right_triangle.get_shader_triangle();
                    if current_shader == ShaderTriangle::TopTopRight {
                        right_triangle.set_shader(ShaderTriangle::TopRight, BlockShaderType::Grey);
                    } else {
                        right_triangle.set_shader(ShaderTriangle::TopBotRight, BlockShaderType::Grey);
                    }
                }
            }
            
            // x-- side
            current_cords[0] -= 1;
            block = BlockTexture::from_id(world.get_world_value(current_cords));
            if block.is_opaque() {
                let current_shader = right_triangle.get_shader_triangle();
                if current_shader == ShaderTriangle::TopBotRight {
                    right_triangle.set_shader(ShaderTriangle::TopRight, BlockShaderType::Grey);
                    break;
                } else {
                    if *direction == Direction::South {
                        right_triangle.set_shader(ShaderTriangle::TopBotRight, BlockShaderType::Grey);
                    } else if *direction == Direction::West {
                        // Do nothing (commented out in original C code)
                    } else {
                        right_triangle.set_shader(ShaderTriangle::TopTopRight, BlockShaderType::Grey);
                    }
                }
            }
            
            // diagonal side
            current_cords[1] += 1;
            block = BlockTexture::from_id(world.get_world_value(current_cords));
            if block.is_opaque() {
                right_triangle.set_shader(ShaderTriangle::TopRight, BlockShaderType::Grey);
                break;
            }
        }
    }
    
    if last_texture == BlockTriangle::LeftTop {
        while draw_distance > 0 {
            draw_distance -= 1;
            
            current_cords[0] -= 1;
            current_cords[1] += 1;
            
            block = BlockTexture::from_id(world.get_world_value(current_cords));
            
            if block.is_opaque() {
                right_triangle.set_shader(ShaderTriangle::LeftCenterLeft, BlockShaderType::Grey);
                current_cords[2] += 1;
                block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    right_triangle.set_shader(ShaderTriangle::LeftTop, BlockShaderType::Grey);
                    break;
                }
                current_cords[2] -= 1;
            }
            current_cords[2] += 1;
        }
    }
}

pub fn cast_left_shadow(camera_data: &CameraData, world: &World, left_triangle: &mut CastedTriangle) {
    let solid_struck_cords = left_triangle.get_solid_struck_cords();
    let mut current_cords = solid_struck_cords;
    
    let direction = camera_data.get_direction();
    let direction_mods = camera_data.get_direction_mods();
    
    let mut block: BlockTexture;
    let mut draw_distance = camera_data.get_shadow_draw_distance();
    
    // Get the last texture in the list to determine which face was struck
    let last_texture = left_triangle.get_last_texture();
    
    if last_texture == BlockTriangle::TopLeft {
        while draw_distance > 0 {
            draw_distance -= 1;
            
            // z++
            current_cords[2] += 1;
            block = BlockTexture::from_id(world.get_world_value(current_cords));
            if block.is_opaque() {
                left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShaderType::Grey);
                break;
            }
            
            // y++ side
            let temp_y = current_cords[1] + 1;
            block = BlockTexture::from_id(world.get_world_value([current_cords[0], temp_y, current_cords[2]]));
            if block.is_opaque() {
                if *direction == Direction::South {
                    let current_shader = left_triangle.get_shader_triangle();
                    if current_shader == ShaderTriangle::TopBotLeft {
                        left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShaderType::Grey);
                    } else {
                        left_triangle.set_shader(ShaderTriangle::TopTopLeft, BlockShaderType::Grey);
                    }
                } else if *direction == Direction::West {
                    // Do nothing (commented out in original C code)
                } else {
                    let current_shader = left_triangle.get_shader_triangle();
                    if current_shader == ShaderTriangle::TopTopLeft {
                        left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShaderType::Grey);
                    } else {
                        left_triangle.set_shader(ShaderTriangle::TopBotLeft, BlockShaderType::Grey);
                    }
                }
            }
            
            // x-- side
            current_cords[0] -= 1;
            block = BlockTexture::from_id(world.get_world_value(current_cords));
            if block.is_opaque() {
                let current_shader = left_triangle.get_shader_triangle();
                if current_shader == ShaderTriangle::TopBotLeft {
                    left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShaderType::Grey);
                    break;
                } else {
                    if direction_mods[0] == -1 && direction_mods[1] == -1 {
                        left_triangle.set_shader(ShaderTriangle::TopBotLeft, BlockShaderType::Grey);
                    } else if *direction == Direction::West {
                        left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShaderType::Grey);
                    } else {
                        left_triangle.set_shader(ShaderTriangle::TopTopLeft, BlockShaderType::Grey);
                    }
                }
            }
            
            // diagonal side
            current_cords[1] += 1;
            block = BlockTexture::from_id(world.get_world_value(current_cords));
            if block.is_opaque() {
                left_triangle.set_shader(ShaderTriangle::TopLeft, BlockShaderType::Grey);
                break;
            }
        }
    }
    
    if last_texture == BlockTriangle::LeftBot {
        while draw_distance > 0 {
            draw_distance -= 1;
            current_cords[0] -= 1;
            current_cords[1] += 1;
            
            block = BlockTexture::from_id(world.get_world_value(current_cords));
            
            if block.is_opaque() {
                left_triangle.set_shader(ShaderTriangle::LeftCenterBot, BlockShaderType::Grey);
                current_cords[2] += 1;
                block = BlockTexture::from_id(world.get_world_value(current_cords));
                if block.is_opaque() {
                    left_triangle.set_shader(ShaderTriangle::LeftBot, BlockShaderType::Grey);
                    break;
                }
                current_cords[2] -= 1;
            }
            current_cords[2] += 1;
        }
    }
}

//=====================================
// Tile Casting 
//=====================================

pub fn raycast_tile(camera_data : &CameraData, world : &World, casted_tile : &mut CastedTile) {
    // Set up world indexing values
    let mut current_block = BlockTexture::Air;
    let mut current_cords = casted_tile.get_cam_world_cords();

    let mut left_cords = current_cords;
    let mut left_face_struck = false;

    let mut right_cords = current_cords;
    let mut right_face_struck = false;

    let directions = camera_data.get_direction_mods();
    let direction = camera_data.get_direction();

    // Reset values and get triangles
    let triangles = casted_tile.get_mut_triangles();


    for d in 0..camera_data.get_draw_distance() {


        // x--
        left_cords[0] -= 1;
        current_block = BlockTexture::from_id(world.get_world_value(left_cords));

        if (!left_face_struck && !current_block.is_transparent()){
            if (current_block.is_translucent()) {
                triangles[0].add_texture(current_block, BlockTriangle::RightTop);
                
                if (!triangles[0].has_struck_translucent()) {
                    triangles[0].struck_translucent(left_cords);
                }
            }
            // If block is solid
            else {
                triangles[0].add_texture(current_block, BlockTriangle::RightTop);
                // TODO : Add shader rendering later 
                if (*direction == Direction::North || *direction == Direction::West) {
                    // Add Shader
                    triangles[0].set_shader(ShaderTriangle::RightTop, BlockShaderType::Grey);
                }
                triangles[0].struck_solid(left_cords);
                left_face_struck = true;
            }
        }



        // y--
        right_cords[1] -= 1;
        current_block = BlockTexture::from_id(world.get_world_value(right_cords));

        if (!right_face_struck && !current_block.is_transparent()){
            triangles[1].add_texture(current_block, BlockTriangle::LeftTop);
            if (current_block.is_translucent()) {
                if (!triangles[1].has_struck_translucent()) {
                    triangles[1].struck_translucent(right_cords);
                }
            }
            // If block is solid
            else {
                // TODO : Add shader rendering later 
                if (*direction == Direction::South || *direction == Direction::West) {
                    // Add Shader
                    
                }
                triangles[1].struck_solid(right_cords);
                right_face_struck = true;
            }
        }


        // x-- & y--
        current_cords[0] -= directions[0];
        current_cords[1] -= directions[1];

        current_block = BlockTexture::from_id(world.get_world_value(current_cords));
        if (!current_block.is_transparent()) {
            if (!left_face_struck) {
                triangles[0].add_texture(current_block, BlockTriangle::LeftBot);
                if (current_block.is_translucent()) {
                    if (!triangles[0].has_struck_translucent()){
                        triangles[0].struck_translucent(current_cords);
                    }
                }
                else {
                    if (*direction == Direction::South || *direction == Direction::West) {
                        // Add shader
                    }
                    triangles[0].struck_solid(current_cords);
                    left_face_struck = true;
                }
            }

            if (!right_face_struck) {
                triangles[1].add_texture(current_block, BlockTriangle::RightBot);
                if (current_block.is_translucent()) {
                    if (!triangles[1].has_struck_translucent()){
                        triangles[1].struck_translucent(current_cords);
                    }
                }
                else {
                    if (*direction == Direction::North || *direction == Direction::West) {
                        // Add shader
                        triangles[1].set_shader(ShaderTriangle::RightBot, BlockShaderType::Grey);
                    }
                    triangles[1].struck_solid(current_cords);
                    right_face_struck = true;
                }
            }
        }

        // z--
        current_cords[2] -= 1;

        current_block = BlockTexture::from_id(world.get_world_value(current_cords));
        if (!current_block.is_transparent()) {
            if (!left_face_struck) {
                triangles[0].add_texture(current_block, BlockTriangle::TopLeft);
                if (current_block.is_translucent()) {
                    if (!triangles[0].has_struck_translucent()){
                        triangles[0].struck_translucent(current_cords);
                    }
                }
                else {
                    triangles[0].struck_solid(current_cords);
                    left_face_struck = true;
                }
            }

            if (!right_face_struck) {
                triangles[1].add_texture(current_block, BlockTriangle::TopRight);
                if (current_block.is_translucent()) {
                    if (!triangles[1].has_struck_translucent()){
                        triangles[1].struck_translucent(current_cords);
                    }
                }
                else {
                    triangles[1].struck_solid(current_cords);
                    right_face_struck = true;
                }
            }
        }

        left_cords = current_cords;
        right_cords = current_cords;

        if (left_face_struck && right_face_struck) {
            break;
        }
    }
}
