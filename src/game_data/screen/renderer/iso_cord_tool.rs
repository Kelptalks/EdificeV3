

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
    
    let mut iso_x = (screen_x + (screen_y * 2.0)) / (2.0 * scale);
    let mut iso_y = ((screen_y * 2.0) - screen_x) / (2.0 * scale);

    if iso_x < 0.0 {
        iso_x -= 1.0;
    }
    if iso_y < 0.0 {
        iso_y -= 1.0;
    }

    // Return unrounded values so caller can determine tile side before flooring
    return [iso_x, iso_y];
}
