use crate::game_data::{TextureManager, locations::world_area::WorldArea, screen::widget::world_rendering::area_rendering_manager::area_rendering_manager::AreaRenderingManager, types::BlockTexture};

#[derive(Clone, Copy)]
pub struct PlayBlock {
    // World
    pub block_type: BlockTexture,
    pub block_world_cords: [i32; 3], 

    // Rendering
    pub rendering_block_cords: [i32; 3],
    pub draw_cords: [f32; 2],
    pub ndc_block_scale: f32,
}

impl PlayBlock {

    pub fn new_blank() -> PlayBlock{
        PlayBlock { 
            block_type:  BlockTexture::Air, 
            block_world_cords: [0, 0, 0], 
            rendering_block_cords: [0, 0, 0], 
            draw_cords: [0.0, 0.0], 
            ndc_block_scale: 0.0 
        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_block(&self, texture_manager: &mut TextureManager) {
        if self.block_type != BlockTexture::Air {
            texture_manager.render_block(self.block_type, self.draw_cords, self.ndc_block_scale);
        }
    }

    pub fn render_cursor(&self, texture_manager: &mut TextureManager, block_ghost: BlockTexture) {
        // Render Selector if at center
        let x = self.rendering_block_cords[0];
        let y = self.rendering_block_cords[1];
        let z = self.rendering_block_cords[2];
        if x == 0 && y == 0 && z == 0 {
            texture_manager.render_block(block_ghost, self.draw_cords, self.ndc_block_scale);
            texture_manager.render_block(BlockTexture::Selector, self.draw_cords, self.ndc_block_scale);
            if self.block_type != BlockTexture::Air {
                texture_manager.render_block(BlockTexture::translucent_red, self.draw_cords, self.ndc_block_scale);
            }
            
        }
        // Render Selector bars
        else if !self.block_type.is_solid(){
            
            if self.block_type == BlockTexture::Air {
                if z == 0 {
                    if x == 0 {
                        texture_manager.render_block(BlockTexture::SelectorBarRight, self.draw_cords, self.ndc_block_scale);
                    }
                    else if y == 0 {
                        texture_manager.render_block(BlockTexture::SelectorBarLeft, self.draw_cords, self.ndc_block_scale);
                    }
                }
                else if x == 0 && y == 0 {
                    texture_manager.render_block(BlockTexture::SelectorVertical, self.draw_cords, self.ndc_block_scale);
                }
            }
            else if z == 0 {
                if x == 0 {
                    texture_manager.render_block(BlockTexture::SelectorBarRightRed, self.draw_cords, self.ndc_block_scale);
                }
                else if y == 0 {
                    texture_manager.render_block(BlockTexture::SelectorBarLeftRed, self.draw_cords, self.ndc_block_scale);
                }
            }
            else if x == 0 && y == 0 {
                texture_manager.render_block(BlockTexture::SelectorVerticalRed, self.draw_cords, self.ndc_block_scale);
            }
        }
    }

    pub fn render_area(&self, texture_manager: &mut TextureManager, world_area: &WorldArea) {        
        
        if world_area.cords_in_area(self.block_world_cords) {
            if world_area.cords_on_edge(self.block_world_cords) {
                texture_manager.render_block(BlockTexture::Dot, self.draw_cords, self.ndc_block_scale);
            }
            else if world_area.cords_on_corner(self.block_world_cords) {
                texture_manager.render_block(BlockTexture::Dot, self.draw_cords, self.ndc_block_scale);
            }

            

            
        }

        // Debug
        /*
        let debug_cords = AreaRenderingManager::get_debug_cords(world_area);
        for debug_cord in debug_cords {
            if debug_cord == self.block_world_cords {
                texture_manager.render_block(BlockTexture::Debug, self.draw_cords, self.ndc_block_scale);
            }
        }
         */

    }


}

