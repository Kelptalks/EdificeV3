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

pub struct WorldGenManager {
    layer_manager: LayerManager,
}

impl WorldGenManager {
    pub fn new() -> Self {
        // Set up world layers
        let mut layer_manager = LayerManager::new();
        layer_manager.add_lair(BlockTexture::Grass, 0, 0);
        layer_manager.add_lair(BlockTexture::Dirt, -3, -1);
        layer_manager.add_lair(BlockTexture::Stone, -100, -4);

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
        
        
        let lair_rules_in_range = self.layer_manager.get_layer_rules_in_range(start_cords[2], end_cords[2]);


        let terrain_height = 100.0;
        
        let terrain_noise = TerrainNoise::new(152452, 4, 500.0);

        println!("Generating Terrain");
        println!(" - Total Lair rules in area = {}", lair_rules_in_range.len());
        
        //Create generator managers
        let mut grass_gen_manager = GrassGenManager::new(); 
        

        // Loop through cords
        for x in start_cords[0]..end_cords[0] {
            for y in start_cords[1]..end_cords[1] {
                for z in start_cords[2]..end_cords[2] {
                    let current_cords = [x, y, z];

                    // Apply terrain noise modification
                    let z_mod = terrain_noise.get_normalized(x as f32, y as f32) * terrain_height;
                    let modded_cords = [x, y, (z as f32 - z_mod) as i32];
                        
                    for layer in &lair_rules_in_range {
                        if layer.get_if_z_in_lair_bounds(z) {
                            
                    
                            let block_to_gen = layer.get_block_type().id();
                            if block_to_gen == BlockTexture::Grass.id() {
                                grass_gen_manager.gen_grass(modded_cords, world);
                            }
                            else {
                                world.set_world_value(layer.get_block_type().id_as_u16(), modded_cords);
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