use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::game_data::{locations::world_area::WorldArea, player_data::drone_programming::var::var_type::Var, screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block::LairBlock, texture_manager::texture::Texture, types::BlockTexture};

pub struct LairBlockManager {
    // block_hashmap
    lair_block_map: HashMap<u64, LairBlock>,

    //


}


impl LairBlockManager {

    pub fn new() -> LairBlockManager {
        LairBlockManager {
            lair_block_map: HashMap::new()
        }
    }

    // Applys vars rendering modifications
    pub fn apply_vars(&mut self, vars: Vec<Rc<RefCell<Var>>>) {

    }

    pub fn debug_fill_location(&mut self, world_area: &WorldArea, block_texture: BlockTexture) {
        let max = world_area.get_max_point().cords;
        let min = world_area.get_min_point().cords;

        
        
        let corners = [
            [min[0], min[1], min[2]],
            [min[0], min[1], max[2]],
            [min[0], max[1], min[2]],
            [min[0], max[1], max[2]],
            [max[0], min[1], min[2]],
            [max[0], min[1], max[2]],
            [max[0], max[1], min[2]],
            [max[0], max[1], max[2]],
        ];

        for corner in corners {
            self.add_texture_at_cords(corner, block_texture);
        }
        

    }

    pub fn outline_world_area(&mut self, world_area: &WorldArea, block_texture: BlockTexture) {
        let max = world_area.get_max_point().cords;
        let min = world_area.get_min_point().cords;

        // 12 edges: each pair shares 2 axes and differs on 1
        let edges: [([i32; 3], [i32; 3]); 12] = [
            // X-aligned edges (vary x, fix y and z)
            ([min[0], min[1], min[2]], [max[0], min[1], min[2]]),
            ([min[0], max[1], min[2]], [max[0], max[1], min[2]]),
            ([min[0], min[1], max[2]], [max[0], min[1], max[2]]),
            ([min[0], max[1], max[2]], [max[0], max[1], max[2]]),
            // Y-aligned edges (vary y, fix x and z)
            ([min[0], min[1], min[2]], [min[0], max[1], min[2]]),
            ([max[0], min[1], min[2]], [max[0], max[1], min[2]]),
            ([min[0], min[1], max[2]], [min[0], max[1], max[2]]),
            ([max[0], min[1], max[2]], [max[0], max[1], max[2]]),
            // Z-aligned edges (vary z, fix x and y)
            ([min[0], min[1], min[2]], [min[0], min[1], max[2]]),
            ([max[0], min[1], min[2]], [max[0], min[1], max[2]]),
            ([min[0], max[1], min[2]], [min[0], max[1], max[2]]),
            ([max[0], max[1], min[2]], [max[0], max[1], max[2]]),
        ];

        for (start, end) in edges {
            self.draw_line(start, end, block_texture);
        }
    }

    fn draw_line(&mut self, start: [i32; 3], end: [i32; 3], block_texture: BlockTexture) {
        let dx = end[0] - start[0];
        let dy = end[1] - start[1];
        let dz = end[2] - start[2];

        let steps = dx.abs().max(dy.abs()).max(dz.abs());
        if steps == 0 {
            self.add_texture_at_cords(start, block_texture);
            return;
        }

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = start[0] + (dx as f32 * t).round() as i32;
            let y = start[1] + (dy as f32 * t).round() as i32;
            let z = start[2] + (dz as f32 * t).round() as i32;
            self.add_texture_at_cords([x, y, z], block_texture);
        }
    }


    //=====================================
    // Helpers
    //=====================================

    fn cords_to_key(cords: [i32; 3]) -> u64 {
        const BITS: u32 = 21;
        const MASK: u64 = (1 << BITS) - 1; // 0x1FFFFF

        let x = (cords[0] as u64) & MASK;
        let y = (cords[1] as u64) & MASK;
        let z = (cords[2] as u64) & MASK;

        x | (y << BITS) | (z << (BITS * 2))
    }


    //=====================================
    // Hashmap mangement
    //=====================================

    // Get a lair block at some cords
    pub fn get_lair_block_at_cords(&self, cords: [i32; 3]) -> Option<&LairBlock> {
        let key = Self::cords_to_key(cords);
        return self.lair_block_map.get(&key);
    }

    pub fn add_texture_at_cords(&mut self, cords: [i32; 3], texture: BlockTexture) {
        let key = Self::cords_to_key(cords);
        let lair_block_at_cords = self.lair_block_map.get_mut(&key);
        if let Some(lair_block) = lair_block_at_cords {
            lair_block.add_texture(texture);
        }
        else {
            let mut new_lair_block = LairBlock::new();
            new_lair_block.add_texture(texture);
            self.lair_block_map.insert(key, new_lair_block);
        }
    }

    // Get
    

}