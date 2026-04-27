
/*
############
## Header ##
############
Manages the creation of rays and lair aditions for raycasting
of an area

*/

use crate::game_data::{
    World, locations::{world_area::WorldArea, world_area_side::WorldAreaSide}, player_data::player_data::PlayerData, screen::widget::{
        prelude::play_world_view_config::PlayViewRenderingConfig, 
        world_rendering::area_rendering_manager::{block_lair_manager::{lair_block::LairBlockMod, 
        lair_block_manager::LairBlockManager}, 
        ray_caster::{ray::TileRay, ray_casting_config::RayCastingConfig}}
    }, types::BlockTexture
};

pub struct AreaRenderingManager {
    area_to_render: WorldArea,
}

// 
impl AreaRenderingManager {

    pub fn new(world_area: &WorldArea) -> AreaRenderingManager {
        AreaRenderingManager {
            area_to_render: *world_area,
        }
    }


    pub fn get_casted_tile_rays(&mut self, world: &World, player_data: &PlayerData) -> Vec<TileRay> {
  
        
          
        // Create an expanded world area for calculating ray start cords
        // Why : This prevents rays from starting inside of a solid block
        let mut expanded_face_orgins_vec: Vec<([i32; 3], [i32; 3])> = Vec::new();
        let mut expanded_world_area = self.area_to_render.clone();
        for _i in 0..1 {
            expanded_world_area.shrink(&WorldAreaSide::XMinus);
            expanded_world_area.shrink(&WorldAreaSide::YMinus);
            expanded_world_area.shrink(&WorldAreaSide::ZMinus);
        }
        // println!("Expanded Area_dimentions: {:?}", expanded_world_area.get_dimensions());
        expanded_face_orgins_vec.append(&mut WorldAreaSide::XPlus.get_face_origins_paired(&expanded_world_area));
        expanded_face_orgins_vec.append(&mut WorldAreaSide::YPlus.get_face_origins_paired(&expanded_world_area));
        expanded_face_orgins_vec.append(&mut WorldAreaSide::ZPlus.get_face_origins_paired(&expanded_world_area));

        // Ray Casting Config
        let ray_casting_world_area = self.area_to_render.clone();

        // Lair managment
        let mut lair_block_manager = LairBlockManager::new();
        
        /*
        if rendering_config.should_render_all_location() {
            for location in &*rendering_config.get_locations_to_render().borrow() {
                lair_block_manager.outline_world_area(location.borrow().get_area(), BlockTexture::Dot);
            }
        }
         */

        /*
        for var in rendering_config.get_vars_to_render() {
            lair_block_manager.render_var(var.get_var_type_ref());
        }
         */

        // Render Cursor location
        let cursor = player_data.get_cursor();

        let lair_block_mod = &LairBlockMod::Cursor(cursor.get_cords(), cursor.get_block_ghost(), cursor.get_zoom());
        lair_block_manager.add_lair_block_mod(lair_block_mod);

        
        let draw_distance = (expanded_world_area.get_dimensions().iter().max()).unwrap().abs() as u32;
        let ray_casting_config = RayCastingConfig::new(
            lair_block_manager, 
            &ray_casting_world_area,
            [1, 1, 1],
            draw_distance
        );
        

        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let chunk_size = ((expanded_face_orgins_vec.len() + num_threads - 1) / num_threads).max(1);

        let mut tile_rays: Vec<TileRay> = Vec::new();
        std::thread::scope(|s| {
            let handles: Vec<_> = expanded_face_orgins_vec
                .chunks(chunk_size)
                .map(|chunk| {
                    s.spawn(|| {
                        chunk.iter().map(|(world_cords, local_cords)| {
                            let mut tile_ray = TileRay::new(*world_cords, *local_cords);
                            tile_ray.cast(world, &ray_casting_config);
                            tile_ray
                        }).collect::<Vec<_>>()
                    })
                })
                .collect();

            for handle in handles {
                tile_rays.extend(handle.join().unwrap());
            }
        });

        return tile_rays;

    }



}



