use crate::game_data::{world::world_gen::grass_gen::{GrassGenManager}, types::BlockType, World};

struct LayerRule {
    main_block_type: BlockType,
    start_z: i32,
    end_z: i32,
}

impl LayerRule {
    pub fn get_block_type(&self) -> BlockType{
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

    pub fn add_lair(&mut self, block_type: BlockType, start_z: i32, end_z: i32){
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
                layer_rule.start_z >= z_start && layer_rule.end_z <= z_end
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
        layer_manager.add_lair(BlockType::Grass, 0, 0);
        layer_manager.add_lair(BlockType::Dirt, -3, -1);
        layer_manager.add_lair(BlockType::Stone, -10, -4);

        Self {
            layer_manager: layer_manager,
        }
    }

    pub fn generate_area(&self, world: &mut World, start_cords: [i32; 3], end_cords: [i32; 3]) {
        let lair_rules_in_range = self.layer_manager.get_layer_rules_in_range(start_cords[2], end_cords[2]);

        println!("Generating Terrain");
        println!(" - Total Lair rules in area = {}", lair_rules_in_range.len());
        
        //Create generator managers
        let mut grass_gen_manager = GrassGenManager::new(); 
        

        // Loop through cords
        for x in start_cords[0]..end_cords[0] {
            for y in start_cords[1]..end_cords[1] {
                for z in start_cords[2]..end_cords[2] {
                    let current_cords = [x, y, z];
                        
                    for layer in &lair_rules_in_range {
                        if layer.get_if_z_in_lair_bounds(z) {
                            let block_to_gen = layer.get_block_type().id();
                            if (block_to_gen == BlockType::Grass.id()) {
                                grass_gen_manager.gen_grass(current_cords, world);
                            }
                            else {
                                world.set_world_value(layer.get_block_type().id_as_u16(), current_cords);
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