


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

pub fn ndi_screen_cords_to_iso_cords(scale : f32, ndi_cords : [f32 ; 2], screen_rez: [f32; 2]) -> [f32; 2]
{
    let x_normalized = ndi_cords[0] * (screen_rez[0] * 0.5);
    let y_normalized = ndi_cords[1] * (screen_rez[1] * 0.5);
    
    println!("Tile : {}", scale);
    let x_mouse_cor = x_normalized - (scale);
    let y_mouse_cor = y_normalized;
    
    let mut iso_x = (x_mouse_cor + 2.0 * y_mouse_cor) / (2.0 * scale);
    let mut iso_y = (2.0 * y_mouse_cor - x_mouse_cor) / (2.0 * scale);
    
    // Cord correction due to 0, 0 center
    if (iso_x < 0.0)
    {
        iso_x -= 1.0;
    }

    if (iso_y < 0.0)
    {
        iso_y -= 1.0;
    }

    return [iso_x, iso_y];
}