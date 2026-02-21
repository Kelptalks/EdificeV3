use crate::game_data::{TextureManager, screen::{ScreenData, screen_data, ui_elements::panel::Panel}, types::BlockTexture};

pub struct BlockSelection {
    ndc: [f32; 2],
    scale: [f32; 2],
    ndc_end_cords: [f32; 2],

    panel: Panel,

    // Block rendering
    blocks_per_row: u32,
    block_ndc_scale: f32,
    block_ndc_gap: f32,
    block_ndc_spacing: f32,

    mouse_on_block: u16,
}


impl BlockSelection {
    pub fn new() -> BlockSelection {
        BlockSelection {
            ndc: [0.0, 0.0],
            scale: [0.0, 0.0],
            ndc_end_cords: [0.0, 0.0],

            panel: Panel::new_blank(),

            // Block rendering
            blocks_per_row: 0,
            block_ndc_scale: 0.0,
            block_ndc_gap: 0.0,
            block_ndc_spacing: 0.0,
            mouse_on_block: 0,
        }
    }

    //=====================================
    // Updates
    //=====================================

    fn re_calculate_ndc_end_cords(&mut self) {
        // Recalculate end cords
        self.ndc_end_cords = [
            self.ndc[0] + self.scale[0],
            self.ndc[1] + self.scale[1],
        ];

        // Re calculate block dimentions based of asspect racio
        let total_blocks = BlockTexture::get_total_blocks();
        let x_to_y_racio = self.scale[0] / self.scale[1];

        let square_root_of_total_blocks = (total_blocks as f32).sqrt();
        self.blocks_per_row = (x_to_y_racio * square_root_of_total_blocks) as u32;
        
        let spacing_racio = 0.1;
        self.block_ndc_spacing = self.scale[0] / self.blocks_per_row as f32;
        self.block_ndc_scale = self.block_ndc_spacing * (1.0 - spacing_racio);
        self.block_ndc_gap = self.block_ndc_spacing * spacing_racio;

    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
        self.panel.set_ndc(ndc);
        self.re_calculate_ndc_end_cords();
    }

    pub fn set_scale(&mut self, scale: [f32; 2]) {
        self.scale = scale;
        self.panel.set_ndc_scale(scale);
        self.panel.set_tile_ndc_scale(0.025);
        self.re_calculate_ndc_end_cords();
    }

    pub fn get_ndc(&self) -> [f32; 2] {
        return self.ndc;
    }

    pub fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    pub fn get_ndc_end_cords(&self) -> [f32; 2] {
        return self.ndc_end_cords;
    }

    pub fn get_panel(&self) -> &Panel {
        return &self.panel;
    }

    pub fn get_block_of_mouse(&self) -> BlockTexture {
        return BlockTexture::from_id(self.mouse_on_block);
    }

    //=====================================
    // Renderer
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        self.panel.render(texture_manager);

        // Setup rendering values
        let total_blocks = BlockTexture::get_total_blocks();

        let ndc_start_cords = [
            self.ndc[0] + self.block_ndc_gap, 
            self.ndc[1] + self.block_ndc_gap
        ];

        let block_real_ndc_scale = self.block_ndc_scale / 2.0;
        for block in 0..total_blocks {
            let block_ndc = [
                ndc_start_cords[0] + ((block % self.blocks_per_row) as f32 * self.block_ndc_spacing),
                ndc_start_cords[1] + ((block / self.blocks_per_row) as f32 * self.block_ndc_spacing),
            ];

            texture_manager.render_block(BlockTexture::from_id(block as u16), block_ndc, block_real_ndc_scale);
        }

        // Get the block the mouse is on
        let mouse_ndc = screen_data.get_mouse_ndc();
        let relative_mouse_cor = [
            mouse_ndc[0] - self.ndc[0],
            mouse_ndc[1] - self.ndc[1]
        ];

        let block_indexes = [
            (relative_mouse_cor[0] / self.block_ndc_spacing) as u32,
            (relative_mouse_cor[1] / self.block_ndc_spacing ) as u32
        ];

        self.mouse_on_block = (block_indexes[0] + (block_indexes[1] * self.blocks_per_row)) as u16;


    }

}

