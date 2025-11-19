


// Convert world cords to isometric rendering location
pub fn world_to_screen_cords(scale : f32, cords : [f32 ; 3]) -> [f32; 2]
{
    let iso_x = cords[0] - cords[2];
    let iso_y = cords[1] - cords[2];
    
    let screen_x = (iso_x - iso_y) * scale;
    let screen_y = (iso_x + iso_y) * (scale / 2.0);
    return [screen_x, screen_y];
}

pub fn casted_to_ndc_cords(scale : f32, cords : [i32 ; 2]) -> [f32; 2] {
    let iso_x = cords[0] as f32;
    let iso_y = cords[1] as f32;
    
    let screen_x = (iso_x - iso_y) * scale;
    let screen_y = (iso_x + iso_y) * (scale / 2.0);
    return [screen_x, screen_y];
}

pub fn ndi_screen_cords_to_iso_cords(scale : f32, ndi_cords : [f32 ; 2]) -> [f32; 2]
{
    // NDC coordinates are already normalized, work directly with them
    let screen_x = ndi_cords[0];
    let screen_y = ndi_cords[1];
    
    println!("Scale : {}", scale);
    println!("Screen cords : ({}, {})", screen_x, screen_y);
    
    // Inverse isometric projection formulas
    // Forward: screen_x = (iso_x - iso_y) * scale, screen_y = (iso_x + iso_y) * (scale / 2.0)
    // Inverse: iso_x = (screen_x + 2 * screen_y) / (2 * scale), iso_y = (2 * screen_y - screen_x) / (2 * scale)
    let iso_x = (screen_x + 2.0 * screen_y) / (2.0 * scale);
    let iso_y = (2.0 * screen_y - screen_x) / (2.0 * scale);
    
    println!("Calculated iso cords (unrounded): ({}, {})", iso_x, iso_y);

    // Return unrounded values so caller can determine tile side before flooring
    return [iso_x, iso_y];
}

// Convert screen pixel coordinates to isometric coordinates
// Converted from C function: screenToIso
pub fn screen_pixel_cords_iso_cords(scale: f32, x: i32, y: i32) -> [i32; 2] {
    // Calculate isometric coordinates using inverse projection
    let mut temp_iso_x = (x as f32 + 2.0 * y as f32) / (2.0 * scale);
    let mut temp_iso_y = (2.0 * y as f32 - x as f32) / (2.0 * scale);
    
    // Coordinate correction for negative values
    if temp_iso_x < 0.0 {
        temp_iso_x -= 1.0;
    }
    if temp_iso_y < 0.0 {
        temp_iso_y -= 1.0;
    }
    
    // Convert to integers and return
    [temp_iso_x as i32, temp_iso_y as i32]
}
