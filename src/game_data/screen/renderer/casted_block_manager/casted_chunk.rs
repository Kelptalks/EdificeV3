use crate::game_data::screen::renderer::camera_data::CameraData;
use crate::game_data::{TextureManager, World};
use super::casted_tile::CastedTile;
use super::super::{ray_caster};

static CHUNK_TILE_DIMENSIONS : u32 = 16;
static CHUNK_TILE_AREA : u32 = CHUNK_TILE_DIMENSIONS * CHUNK_TILE_DIMENSIONS;

pub struct CastedChunk
{
    chunk_casted_cords : [i32; 2],
    tiles : [CastedTile ; CHUNK_TILE_AREA as usize],
    ray_casted : bool,
}

impl CastedChunk {

    pub fn new(chunk_cords : [i32; 2], camera_data : &CameraData) -> Self {
        let cam_orgin_cords = camera_data.get_cam_world_cords();

        let x_chunk_tile_cords = (chunk_cords[0] * CHUNK_TILE_DIMENSIONS as i32) as i32;
        let y_chunk_tile_cords = (chunk_cords[1] * CHUNK_TILE_DIMENSIONS as i32) as i32;

        // Create chunks tile set
        let mut tiles = std::array::from_fn(|i| {

            // Calculate the cast_cor and create tile
            let x_tile_internal_chunk_cor = (i % CHUNK_TILE_DIMENSIONS as usize) as i32;
            let y_tile_internal_chunk_cor = (i / CHUNK_TILE_DIMENSIONS as usize) as i32;

            // Calculate the casted cords
            let x_tile_casted_cor = x_tile_internal_chunk_cor + x_chunk_tile_cords;
            let y_tile_casted_cor = y_tile_internal_chunk_cor + y_chunk_tile_cords;


            let mut casted_tile = CastedTile::new([x_tile_casted_cor, y_tile_casted_cor]);
            
            
            // Set camera render cords based off casted cords
            let x = x_tile_casted_cor + cam_orgin_cords[0] as i32;
            let y = y_tile_casted_cor + cam_orgin_cords[1] as i32;
            let z = cam_orgin_cords[2] as i32;
            
            casted_tile.set_world_camera_cords([x, y, z]);

            return casted_tile;
        });

        Self {
            chunk_casted_cords : chunk_cords,
            tiles: tiles,
            ray_casted: false,
        }
    }

    pub fn get_mut_tile_at_index(&mut self, index : usize) -> &mut CastedTile {
        return &mut self.tiles[index];
    }

    pub fn raycast_chunk(&mut self, camera_data : &CameraData, world : &World) {
        for tile in &mut self.tiles {
            ray_caster::raycast_tile_with_shadows(camera_data, world, tile);
        }
        self.ray_casted = true;
    }


    pub fn render_chunk(&self, camera_data : &CameraData, texture_manager : &mut TextureManager)
    {   
        for i in 0..CHUNK_TILE_AREA {
            self.tiles[i as usize].render_tile(camera_data, texture_manager);
        }
    }

    pub fn is_ray_casted(&self) -> bool {
        return self.ray_casted;
    }

    pub fn set_ray_casted(&mut self, ray_casted: bool) {
        self.ray_casted = ray_casted;
    }
    
    pub fn get_chunk_cords(&self) -> [i32; 2] {
        return self.chunk_casted_cords;
    }

}
