use crate::game_data::{World, locations::world_area::WorldArea, types::BlockTexture, world_gen::{terrain_gen::{grass_gen::GrassGenManager, perlin_noise::TerrainNoise}, world_config::WorldConfig}};

struct LayerRule {
    main_block_type: BlockTexture,
    start_z: i32,
    end_z: i32,
}

impl LayerRule {
    pub fn get_block_type(&self) -> BlockTexture{
        return self.main_block_type;
    }

    pub fn get_if_z_in_lair_bounds(&self, z: i32) -> bool {
        return z>= self.start_z && z <= self.end_z;
    }

}
struct LayerManager {
    layers : Vec<LayerRule>,
}

impl LayerManager {
    pub fn new() -> Self {
        Self {
            layers : Vec::new(),
        }
    }

    pub fn add_lair(&mut self, block_type: BlockTexture, start_z: i32, end_z: i32){
        let new_layer = LayerRule {
            main_block_type: block_type,
            start_z: start_z,
            end_z: end_z,
        };
        self.layers.push(new_layer);
    }

    pub fn get_layer_rules_in_range(&self, z_start: i32, z_end: i32) -> Vec<&LayerRule> {
        self.layers.iter()
            .filter(|layer_rule| {
                layer_rule.start_z >= z_start || layer_rule.end_z <= z_end
            })
            .collect()
    }

    pub fn get_block_at_layer_z(&self, layer_z: i32) -> Option<BlockTexture> {
        for layer in &self.layers {
            if layer.get_if_z_in_lair_bounds(layer_z) {
                return Some(layer.get_block_type());
            }
        }
        None
    }


    pub fn test(&self) {
        let z_range = [100, -100];
        let layer_refs = self.get_layer_rules_in_range(z_range[0], z_range[1]);

        println!("Hi");

        for l in -100..100 {
            for layer in &layer_refs {
                if layer.get_if_z_in_lair_bounds(l) {
                    println!("Block Type at Level({}) is ({})",l , layer.get_block_type().id());
                }
            }
        }   

    }
}

const WATER_LEVEL: i32 = -40;

pub struct WorldGenManager {
    layer_manager: LayerManager,
}

impl WorldGenManager {
    pub fn new() -> Self {
        // Set up world layers
        let mut layer_manager = LayerManager::new();
        layer_manager.add_lair(BlockTexture::Grass, 0, 0);
        layer_manager.add_lair(BlockTexture::Dirt, -3, 0);
        layer_manager.add_lair(BlockTexture::Stone, -200, -4);

        Self {
            layer_manager: layer_manager,
        }
    }

    //=====================================
    // Terrain Generation
    //=====================================

    pub fn generate_area(&self, world: &mut World, area: WorldArea) {
        let start_cords = area.get_point_1_cords();
        let end_cords = area.get_point_2_cords();

        let terrain_height = 50.0;
        let terrain_noise = TerrainNoise::new(152452, 4, 500.0);
        let mut grass_gen_manager = GrassGenManager::new();

        for x in start_cords[0]..=end_cords[0] {
            for y in start_cords[1]..=end_cords[1] {
                let z_mod = terrain_noise.get_normalized(x as f32, y as f32) * terrain_height;
                let surface_z = (-z_mod) as i32;
                let below_water = surface_z < WATER_LEVEL;

                for z in start_cords[2]..=end_cords[2] {
                    // Convert world z to layer z to determine which block goes here
                    let layer_z = (z as f32 + z_mod) as i32;

                    if let Some(block_type) = self.layer_manager.get_block_at_layer_z(layer_z) {
                        if block_type == BlockTexture::Grass {
                            if below_water {
                                world.set_world_value(BlockTexture::BlueGrass.id_as_u16(), [x, y, z]);
                            } else {
                                grass_gen_manager.gen_grass([x, y, z], world);
                            }
                        } else {
                            world.set_world_value(block_type.id_as_u16(), [x, y, z]);
                        }
                    }
                }

                // Fill water from just above terrain surface up to water level
                if below_water {
                    let water_start = (surface_z + 1).max(start_cords[2]);
                    let water_end = WATER_LEVEL.min(end_cords[2]);
                    if water_start <= water_end {
                        for wz in water_start..=water_end {
                            if world.get_world_value([x, y, wz]) == 0 {
                                world.set_world_value(BlockTexture::Water.id_as_u16(), [x, y, wz]);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn test(&self) {
        self.layer_manager.test();
    }
}