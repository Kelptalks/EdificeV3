
/*
############
## Header ##
############
Manages the creation of rays and lair aditions for raycasting
of an area

*/

use crate::game_data::{World, locations::{world_area::WorldArea, world_area_side::WorldAreaSide}, ray_caster::ray::{self, TileRay}};

pub struct AreaRenderingManager {
    area_to_render: WorldArea
}

// 
impl AreaRenderingManager {
    
    pub fn get_debug_cords(world_area: &WorldArea) -> Vec<[i32; 3]> {
        let mut vec = Vec::new();

        vec.append(&mut WorldAreaSide::XPlus.get_face_origins(world_area));
        vec.append(&mut WorldAreaSide::YPlus.get_face_origins(world_area));
        vec.append(&mut WorldAreaSide::ZPlus.get_face_origins(world_area));

        return vec;
    }

    // Set Area


    pub fn get_casted_tile_rays(world_area: &WorldArea, world: &World) -> Vec<TileRay> {
        let max_point = world_area.get_max_point();
        let min_point = world_area.get_min_point();
        
        let mut vec: Vec<([i32; 3], [i32; 3])> = Vec::new();
        vec.append(&mut WorldAreaSide::XPlus.get_face_origins_paired(world_area));
        vec.append(&mut WorldAreaSide::YPlus.get_face_origins_paired(world_area));
        vec.append(&mut WorldAreaSide::ZPlus.get_face_origins_paired(world_area));


        let mut tile_rays = Vec::new();
        for (world_cords, local_cords) in vec {
            let mut tile_ray = TileRay::new(
                world_cords, 
                local_cords, 
                [1, 1, 1], 
                50
            );
            tile_ray.cast(world);
            tile_rays.push(tile_ray);
        }

        
        return tile_rays;

    }



}



