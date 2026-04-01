use crate::game_data::screen::renderer::casted_block_manager::casted_chunk::CastedChunk;

#[derive(Clone, Copy)]
pub struct CanvasData {
    pub tiles_per_row : u32,
    pub tiles_per_collumn : u32,
    pub canvas_rez: f32,
    pub max_tiles: u32,

    // tile properties
    pub tile_ndc_scale: [f32; 2],
    pub pixel_tile_scale: [f32; 2],
    pub tile_uv_scale: [f32; 2],
    pub casted_tile_ndc_scale: f32,

    // Rendering data
    pub expander: f32,

}

impl CanvasData {
    pub fn new() -> CanvasData {
        let canvas_x_tile_scale = 256;
        let canvas_y_tile_scale = canvas_x_tile_scale / 2;
        
        let tiles_per_row = 64;
        let tiles_per_collumn = tiles_per_row * 2;

        let max_tiles = tiles_per_row * tiles_per_collumn;

        let canvas_rez = canvas_x_tile_scale * tiles_per_row;

        let tile_x_uv_scale = 1.0 / tiles_per_row as f32;
        let tile_y_uv_scale = 1.0 / tiles_per_collumn as f32;

        let tile_x_ndc_scale = tile_x_uv_scale;
        let tile_y_ndc_scale = tile_y_uv_scale;
        
        let casted_tile_ndc_scale = tile_x_ndc_scale / CastedChunk::get_chunk_tile_dimensions() as f32;


        CanvasData {
            // Canvas properties
            tiles_per_row: tiles_per_row,
            tiles_per_collumn: tiles_per_collumn,

            canvas_rez: canvas_rez as f32,
            max_tiles,

            // Tile properties
            tile_ndc_scale: [tile_x_ndc_scale, tile_y_ndc_scale],
            pixel_tile_scale: [canvas_x_tile_scale as f32, canvas_y_tile_scale as f32],
            tile_uv_scale: [tile_x_uv_scale, tile_y_uv_scale],
            casted_tile_ndc_scale: casted_tile_ndc_scale,

            // Rendering Daa
            expander: 1.015,
        }   
    }

    pub fn get_render_scale(&self) -> f32 {
        return self.casted_tile_ndc_scale;
    }

    pub fn id_to_canvas_cords(&self, canvas_id: u32) -> [u32; 2] {
        let tile_x = canvas_id % self.tiles_per_row;
        let tile_y = canvas_id / self.tiles_per_row;
        [tile_x, tile_y]  // No flip here
    }

    pub fn id_to_uv_cords(&self, canvas_id: u32) -> [f32; 4] {
        let tile_canvas_cords = self.id_to_canvas_cords(canvas_id);

        // Calculate base UV coordinates
        let uv_x = tile_canvas_cords[0] as f32 * (self.tile_uv_scale[0]);
        let uv_y = tile_canvas_cords[1] as f32 * (self.tile_uv_scale[1]);

        // Invert both X and Y axes to match render-to-texture coordinate system
        // When sampling from a render target, coordinates are flipped
        // Only flip Y-axis for render-to-texture
        return [
            uv_x,                                   // left
            1.0 - uv_y,                            // bottom (flipped)
            uv_x + self.tile_uv_scale[0],          // right
            1.0 - (uv_y + self.tile_uv_scale[1])   // top (flipped)
        ];
    }

    pub fn id_to_canvas_ndc_cords(&self, canvas_id: u32) -> [f32; 2] {
        let canvas_tile_cords = self.id_to_canvas_cords(canvas_id);

        let x_cor = (canvas_tile_cords[0] as f32 * self.tile_uv_scale[0]) * 2.0 - 1.0;
        let y_cor = (canvas_tile_cords[1] as f32 * self.tile_uv_scale[1]) * 2.0 - 1.0;


        [x_cor, y_cor]
    }

    /// Get the render offset coordinates for a canvas chunk
    /// This applies an X offset to center the chunk for rendering to the canvas texture
    pub fn id_to_canvas_render_offset(&self, canvas_id: u32) -> [f32; 2] {
        let base_ndc = self.id_to_canvas_ndc_cords(canvas_id);
        
        // Apply X offset to center the chunk when rendering to canvas
        [
            base_ndc[0] + self.tile_ndc_scale[0],
            base_ndc[1]
        ]
    }

    pub fn test(&self) {
        // Test ID 0 - should be top-left tile
        let uv_0 = self.id_to_uv_cords(0);
        println!("ID 0 UV: [{}, {}, {}, {}]", uv_0[0], uv_0[1], uv_0[2], uv_0[3]);

        // Test ID 1 - should be second tile in first row
        let uv_1 = self.id_to_uv_cords(1);
        println!("ID 1 UV: [{}, {}, {}, {}]", uv_1[0], uv_1[1], uv_1[2], uv_1[3]);

        // Test ID 16 - should be first tile in second row
        let uv_16 = self.id_to_uv_cords(16);
        println!("ID 16 UV: [{}, {}, {}, {}]", uv_16[0], uv_16[1], uv_16[2], uv_16[3]);
    }
}
