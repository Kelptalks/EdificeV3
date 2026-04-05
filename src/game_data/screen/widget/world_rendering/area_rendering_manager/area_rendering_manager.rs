
/*
############
## Header ##
############
Manages the creation of rays and lair aditions for raycasting
of an area

*/

use crate::game_data::{World, locations::{world_area::WorldArea, world_area_side::WorldAreaSide}, ray_caster::{ray::{self, TileRay}, ray_casting_config::{self, RayCastingConfig}}, screen::widget::world_rendering::area_rendering_manager::block_lair_manager::lair_block_manager::{self, LairBlockManager}, types::BlockTexture};

pub struct AreaRenderingManager {
    area_to_render: WorldArea
}

// 
impl AreaRenderingManager {
    

    pub fn get_casted_tile_rays(world_area: &WorldArea, world: &World) -> Vec<TileRay> {
  
        
          
        // Create an expanded world area for calculating ray start cords
        // Why : This prevents rays from starting inside of a solid block
        let mut expanded_face_orgins_vec: Vec<([i32; 3], [i32; 3])> = Vec::new();
        let mut expanded_world_area = world_area.clone();
        for i in 0..1 {
            expanded_world_area.shrink(&WorldAreaSide::XMinus);
            expanded_world_area.shrink(&WorldAreaSide::YMinus);
            expanded_world_area.shrink(&WorldAreaSide::ZMinus);
        }
        // println!("Expanded Area_dimentions: {:?}", expanded_world_area.get_dimensions());
        expanded_face_orgins_vec.append(&mut WorldAreaSide::XPlus.get_face_origins_paired(&expanded_world_area));
        expanded_face_orgins_vec.append(&mut WorldAreaSide::YPlus.get_face_origins_paired(&expanded_world_area));
        expanded_face_orgins_vec.append(&mut WorldAreaSide::ZPlus.get_face_origins_paired(&expanded_world_area));
        

    


        // Ray Casting Config
        let ray_casting_world_area = world_area.clone();
        let mut lair_block_manager = LairBlockManager::new();
        
        let draw_distance = (expanded_world_area.get_dimensions().iter().max()).unwrap().abs() as u32;
        let ray_casting_config = RayCastingConfig::new(
            lair_block_manager, 
            &ray_casting_world_area,
            [1, 1, 1],
            draw_distance
        );
        

        let mut tile_rays = Vec::new();
        for (world_cords, local_cords) in expanded_face_orgins_vec {
            let mut tile_ray = TileRay::new(
                world_cords, 
                local_cords, 
            );


            tile_ray.cast(world, &ray_casting_config);
            tile_rays.push(tile_ray);
        }
        
        return tile_rays;

    }



}



