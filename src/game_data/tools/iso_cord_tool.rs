

//=====================================
// Screen
//=====================================

// Convert isometric coordinates to normalized device coordinates
pub fn casted_to_ndc_cords(scale : f32, cords : [i32 ; 2]) -> [f32; 2] {
    let iso_x = cords[0] as f32;
    let iso_y = cords[1] as f32;

    let screen_x = (iso_x - iso_y) * scale;
    let screen_y = (iso_x + iso_y) * (scale / 2.0);
    return [screen_x, screen_y];
}

// Convert isometric coordinates to normalized device coordinates
pub fn float_iso_to_ndc_cords(scale : f32, cords : [f32 ; 2]) -> [f32; 2] {
    let iso_x = cords[0];
    let iso_y = cords[1];

    let screen_x = (iso_x - iso_y) * scale;
    let screen_y = (iso_x + iso_y) * (scale / 2.0);
    return [screen_x, screen_y];
}

pub fn ndi_screen_cords_to_iso_cords(scale : f32, ndi_cords : [f32 ; 2]) -> [f32; 2]
{
    // NDC coordinates are already normalized, work directly with them
    let screen_x = ndi_cords[0];
    let screen_y = ndi_cords[1];

    let iso_x = (screen_x + (screen_y * 2.0)) / (2.0 * scale);
    let iso_y = ((screen_y * 2.0) - screen_x) / (2.0 * scale);


    // Return unrounded values so caller can determine tile side before flooring
    return [iso_x, iso_y];
}

//=====================================
// World
//=====================================

pub fn flatten_world_cords(world_cords : [i32 ; 3]) -> [i32; 2] {
    [
        world_cords[0] - world_cords[2],
        world_cords[1] - world_cords[2],
    ]
}

pub fn get_depth_from_world_cords(world_cords: [i32; 3]) -> i32 {
    world_cords[0] + world_cords[1] + world_cords[2]
}

pub fn world_pos_to_ndc_cords(scale : f32, world_pos: [f32; 3]) -> [f32; 2] {
    let flattened_iso_cords =
    [
        world_pos[0] - world_pos[2],
        world_pos[1] - world_pos[2]
    ];

    return float_iso_to_ndc_cords(scale, flattened_iso_cords)
}

pub fn world_cords_to_world_pos(world_cords: [i32; 3]) -> [f32; 3] {
    [
        world_cords[0] as f32,
        world_cords[1] as f32,
        world_cords[2] as f32,
    ]
}

pub fn world_pos_to_world_cords(world_pos: [f32; 3]) -> [i32; 3] {
    [
        world_pos[0].round() as i32,
        world_pos[1].round() as i32,
        world_pos[2].round() as i32,
    ]
}

pub fn world_pos_to_tile_cords(world_pos: [f32; 3]) -> [i32; 2] {
    let world_cords = [
        world_pos[0].round() as i32,
        world_pos[1].round() as i32,
        world_pos[2].round() as i32,
    ];

    world_cords_to_tile_cords(world_cords)
}

pub fn world_cords_to_tile_cords(world_cords: [i32; 3]) -> [i32; 2] {
    [
        world_cords[0] - world_cords[2],
        world_cords[1] - world_cords[2],
    ]
}
