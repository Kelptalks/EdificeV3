use crate::game_data::{locations::world_area::WorldArea, types::BlockTexture, world::world::WorldEvent, world_gen::terrain_gen::{grass_gen::GrassGenManager, perlin_noise::TerrainNoise}};

const CHUNK_SIZE: i32 = 16;

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

const WATER_LEVEL: i32 = -50;

pub struct WorldGenManager {
    layer_manager: LayerManager,
}

impl WorldGenManager {
    pub fn new() -> Self {
        // Set up world layers
        let mut layer_manager = LayerManager::new();
        layer_manager.add_lair(BlockTexture::Grass, 0, 0);
        layer_manager.add_lair(BlockTexture::Dirt, -3, -1);
        layer_manager.add_lair(BlockTexture::Stone, -20, -4);

        Self {
            layer_manager: layer_manager,
        }
    }

    //=====================================
    // Terrain Generation
    //=====================================

    // Returns Some(block) if every block in the area would be the same type (None = all air).
    // Returns None if the area is non-uniform and needs per-block generation.
    // Only checks min and max z per column — sufficient because layer ranges are contiguous.
    fn try_uniform_block(&self, area: &WorldArea, terrain_noise: &TerrainNoise, terrain_height: f32) -> Option<Option<BlockTexture>> {
        let start = area.get_point_1_cords();
        let end = area.get_point_2_cords();

        let mut uniform: Option<Option<BlockTexture>> = None;

        for x in start[0]..=end[0] {
            for y in start[1]..=end[1] {
                let z_mod = terrain_noise.get_normalized(x as f32, y as f32) * terrain_height;
                let surface_z = (-z_mod) as i32;

                // Surface passes through chunk — terrain transition, not uniform
                if surface_z >= start[2] && surface_z <= end[2] {
                    return None;
                }

                // Water zone overlaps chunk — mixed content
                if surface_z < WATER_LEVEL {
                    let water_start = (surface_z + 1).max(start[2]);
                    let water_end = WATER_LEVEL.min(end[2]);
                    if water_start <= water_end {
                        return None;
                    }
                }

                let layer_z_min = (start[2] as f32 + z_mod).floor() as i32;
                let layer_z_max = (end[2] as f32 + z_mod).floor() as i32;

                let block_min = self.layer_manager.get_block_at_layer_z(layer_z_min);
                let block_max = self.layer_manager.get_block_at_layer_z(layer_z_max);

                // Column spans two different layers
                if block_min != block_max {
                    return None;
                }

                // Grass surface would be in this column — vegetation makes it non-uniform
                if block_min == Some(BlockTexture::Grass) {
                    return None;
                }

                match &uniform {
                    None => uniform = Some(block_min),
                    Some(prev) if *prev != block_min => return None,
                    _ => {}
                }
            }
        }

        uniform
    }

    pub fn generate_area(&self, area: WorldArea) -> Vec<WorldEvent> {
        let mut events = Vec::new();

        let start_cords = area.get_point_1_cords();
        let end_cords = area.get_point_2_cords();

        let terrain_height = 100.0;
        let terrain_noise = TerrainNoise::new(152452, 4, 500.0);

        // Fast path: entire chunk is one block type
        match self.try_uniform_block(&area, &terrain_noise, terrain_height) {
            Some(None) => return Vec::new(), // All air — chunk default is already 0
            Some(Some(block)) => {
                let chunk_cords = [
                    start_cords[0].div_euclid(CHUNK_SIZE) as i16,
                    start_cords[1].div_euclid(CHUNK_SIZE) as i16,
                    start_cords[2].div_euclid(CHUNK_SIZE) as i16,
                ];
                return vec![WorldEvent::FillChunk(chunk_cords, block)];
            }
            None => {} // Non-uniform, fall through to per-block generation
        }

        let mut grass_gen_manager = GrassGenManager::new();

        for x in start_cords[0]..=end_cords[0] {
            for y in start_cords[1]..=end_cords[1] {
                let z_mod = terrain_noise.get_normalized(x as f32, y as f32) * terrain_height;
                let surface_z = (-z_mod) as i32;
                let below_water = surface_z < WATER_LEVEL;

                for z in start_cords[2]..=end_cords[2] {
                    // Convert world z to layer z to determine which block goes here
                    let layer_z = (z as f32 + z_mod).floor() as i32;

                    if let Some(block_type) = self.layer_manager.get_block_at_layer_z(layer_z) {
                        if block_type == BlockTexture::Grass {
                            if below_water {
                                events.push(WorldEvent::ModBlock([x, y, z], BlockTexture::Sand));
                            } else {
                                events.extend(grass_gen_manager.gen_grass([x, y, z]));
                            }
                        } else {
                            events.push(WorldEvent::ModBlock([x, y, z], block_type));
                        }
                    }
                }

                // Fill water from just above terrain surface up to water level
                if below_water {
                    let water_start = (surface_z + 1).max(start_cords[2]);
                    let water_end = WATER_LEVEL.min(end_cords[2]);
                    if water_start <= water_end {
                        for wz in water_start..=water_end {
                            events.push(WorldEvent::ModBlock([x, y, wz], BlockTexture::Water));
                        }
                    }
                }
            }
        }

        events
    }

    pub fn test(&self) {
        self.layer_manager.test();
    }
}
