use crate::game_data::{renderer::{casted_tile::CastedTile, CameraData, Direction}, Types::{BlockShaderType, BlockTriangle, BlockType, ShaderTriangle}, World};



pub fn raycast_tile(camera_data : &CameraData, world : &World, casted_tile : &mut CastedTile)
{
    // Set up world indexing values
    let mut current_block = BlockType::Air;
    let mut current_cords = casted_tile.get_cam_world_cords();

    let mut left_cords = current_cords.clone();
    let mut left_face_struck = false;

    let mut right_cords = current_cords.clone();
    let mut right_face_struck = false;

    let directions = camera_data.get_direction_mods();
    let direction = camera_data.get_direction();

    // Reset values and get triangles
    casted_tile.reset_casting_values();
    let mut triangles = casted_tile.get_mut_triangles();


    for d in 0..camera_data.get_draw_distance() {


        // x--
        left_cords[0] -= 1;
        current_block = BlockType::from_id(world.get_world_value(left_cords));

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
        current_block = BlockType::from_id(world.get_world_value(right_cords));

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

        current_block = BlockType::from_id(world.get_world_value(current_cords));
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

        current_block = BlockType::from_id(world.get_world_value(current_cords));
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

        left_cords = current_cords.clone();
        right_cords = current_cords.clone();

        if (left_face_struck && right_face_struck) {
            break;
        }

    }
}