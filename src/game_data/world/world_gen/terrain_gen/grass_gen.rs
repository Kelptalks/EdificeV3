use crate::game_data::{World, game_event_manager::{self, event_manager, game_event_manager::GameEventManager}, types::BlockTexture};
use rand::{rngs::ThreadRng, Rng};

struct GroundItem {
    block_type: BlockTexture,
    weight: u32,
}

#[derive(Copy, Clone)]
enum PlantType {
    Tree,
    Mushroom,
    Dandelion,
}

impl PlantType {
    fn generate(&self, rng: &mut ThreadRng, world: &mut World, cords: [i32; 3]) {
        match self {
            PlantType::Tree => Self::generate_tree(rng, world, cords),
            PlantType::Mushroom => Self::generate_mushroom(rng, world, cords),
            PlantType::Dandelion => Self::generate_dandelion(rng, world, cords),
        }
    }

    /*#####################
      ## Tree Generation ##
      #####################*/
    fn generate_leaves(world: &mut World, cords: [i32; 3]) {
        world.set_world_value(BlockTexture::Leaves.id_as_u16(), [cords[0] - 1, cords[1], cords[2]]);
        world.set_world_value(BlockTexture::Leaves.id_as_u16(), [cords[0] + 1, cords[1], cords[2]]);
        world.set_world_value(BlockTexture::Leaves.id_as_u16(), [cords[0], cords[1] - 1, cords[2]]);
        world.set_world_value(BlockTexture::Leaves.id_as_u16(), [cords[0], cords[1] + 1, cords[2]]);
        world.set_world_value(BlockTexture::Leaves.id_as_u16(), [cords[0], cords[1], cords[2] + 1]);
    }

    fn generate_branch(world: &mut World, rng: &mut ThreadRng, cords: [i32; 3]){
        
        // randomize branch direction
        let mut x_branch_direction_mod = 0;
        let mut y_branch_direction_mod = 0;
        let branch_direction = rng.random_range(1..4);
            if branch_direction == 1{
                x_branch_direction_mod = 1;
            }
            else if branch_direction == 2{
                x_branch_direction_mod = -1;
            }
            else if branch_direction == 3{
                y_branch_direction_mod = 1;
            }
            else if branch_direction == 4{
                y_branch_direction_mod = -1;
            }
        
        //Generate branch
        let branch_length = 3;
        for i in 1..=branch_length {
            // Calculate cords based off direction
            let current_cords = [
                cords[0] + (x_branch_direction_mod * i),
                cords[1] + (y_branch_direction_mod * i),
                cords[2]
            ]; 
            world.set_world_value(BlockTexture::Leaves.id_as_u16(), current_cords);
            if i == branch_length {
                Self::generate_leaves(world, current_cords);
            }
        }
    }

    fn generate_tree(rng: &mut ThreadRng, world: &mut World, cords: [i32; 3]) {
        // Decide if tree is purple
        let purple_tree_likleyhood = 0.05;
        let mut block_type = BlockTexture::BrownTrunk.id_as_u16();
        if rng.random_bool(purple_tree_likleyhood) {
            block_type = BlockTexture::PurpleTrunk.id_as_u16();
        }
        
        // Tree generation propertys
        let min_tree_height = 5;
        let max_tree_height = 15;
        let tree_height = rng.random_range(min_tree_height..max_tree_height);

        // Create trunk
        for z in 0..=tree_height {
            let mut trunk_cor = cords;
            trunk_cor[2] += z;

            world.set_world_value(block_type, trunk_cor);
            if z == tree_height {
                trunk_cor[2] += 1;
                Self::generate_leaves(world, trunk_cor);
            }
        }


        // Branch generation proppertys.
        let min_branch_height = 7;
        let max_branch_height = 10;

        // Generate branch if tree is tall enough 
        if tree_height - 3 >= min_branch_height {
            let branch_height = rng.random_range(min_branch_height..max_branch_height);
            Self::generate_branch(world, rng, [cords[0], cords[1], cords[2] + branch_height]);
        }

    }

    fn generate_mushroom(rng: &mut ThreadRng, world: &mut World, cords: [i32; 3]) {
        let height = rng.gen_range(10..40);
        let stem_radius = (height / (rng.gen_range(5..25))) + 3;

        // Build the stem
        for z in 0..height {
            for x in -stem_radius..stem_radius {
                for y in -stem_radius..stem_radius {
                    let distance = ((x * x + y * y) as f64).sqrt();
                    if distance < stem_radius as f64 {
                        world.set_world_value(
                            BlockTexture::MushroomStem.id_as_u16(),
                            [cords[0] + x, cords[1] + y, cords[2] + z]
                        );
                    }
                }
            }
        }

        // Choose block type (blue or pink mushroom)
        let block_type = if rng.gen_range(0..2) == 0 {
            BlockTexture::BlueMushroom.id_as_u16()
        } else {
            BlockTexture::PinkMushroomBlock.id_as_u16()
        };

        let mut top_radius = stem_radius + 8 + rng.gen_range(0..5);
        let mut z_mod = 0;

        // Build the mushroom cap
        while top_radius > 2 {
            for x in -top_radius..top_radius {
                for y in -top_radius..top_radius {
                    let distance = ((x * x + y * y) as f64).sqrt();
                    if distance < top_radius as f64 {
                        world.set_world_value(
                            block_type,
                            [cords[0] + x, cords[1] + y, cords[2] + z_mod + height]
                        );
                    }
                }
            }
            z_mod += 1;
            top_radius -= rng.gen_range(0..2);
        }
    }

    fn generate_dandelion(rng: &mut ThreadRng, world: &mut World, cords: [i32; 3]) {
        let height = rng.gen_range(12..37);
        let stem_radius = height / 15;

        // Build the stem
        for z in 0..height {
            for x in -stem_radius..stem_radius {
                for y in -stem_radius..stem_radius {
                    let distance = ((x * x + y * y) as f64).sqrt();
                    if distance < stem_radius as f64 {
                        world.set_world_value(
                            BlockTexture::DandiStem.id_as_u16(),
                            [cords[0] + x, cords[1] + y, cords[2] + z]
                        );
                    }
                }
            }
        }

        // Build the dandelion puff ball
        let puff_top = height - 1;
        let mut puff_radius = (stem_radius as f64 * 3.2) as i32;

        // Make sure number is odd
        if puff_radius % 2 == 0 {
            puff_radius += 1;
        }

        for x in -puff_radius..puff_radius {
            for y in -puff_radius..puff_radius {
                for z in -puff_radius..puff_radius {
                    // Calculate the distance from the center
                    let distance_sq = x * x + y * y + z * z;
                    if distance_sq <= puff_radius * puff_radius {
                        // 80% chance to place a block (creating a fluffy appearance)
                        if rng.gen_range(0..5) != 0 {
                            world.set_world_value(
                                BlockTexture::PinkCloud.id_as_u16(),
                                [cords[0] + x, cords[1] + y, cords[2] + z + puff_top]
                            );
                        }
                    }
                }
            }
        }
    }

}

struct Plant {
    plant_type: PlantType,
    weight: u32,
}

pub struct GrassGenManager {
    rng: ThreadRng,

    nothing_weight: u32, 

    // Ground item probs
    ground_item_weight: u32,
    ground_item_total_weight: u32,
    ground_items: Vec<GroundItem>,


    // Plant gen probs
    plant_weight: u32,
    plant_total_weight: u32,
    plants: Vec<Plant>,

}

impl GrassGenManager {
    pub fn new() -> Self {
        // Setup Ground item generation probabilitys
        let ground_items:Vec<GroundItem> = vec![
            GroundItem {block_type: BlockTexture::white_flowers, weight: 20},
            GroundItem {block_type: BlockTexture::yellow_flowers, weight: 20},
            GroundItem {block_type: BlockTexture::Flungle, weight: 1},
            GroundItem {block_type: BlockTexture::mushroom, weight: 3},
            GroundItem {block_type: BlockTexture::log, weight: 2},
            GroundItem {block_type: BlockTexture::rock, weight: 3},
        ];
        // Setup Plant generation probabilitys
        let plants:Vec<Plant> = vec![
            Plant {plant_type: PlantType::Tree, weight: 200},
            Plant {plant_type: PlantType::Mushroom, weight: 0},
            Plant {plant_type: PlantType::Dandelion, weight: 10},
        ];


        // Calculate total weights
        let mut total_ground_item_weight = 0;
        for i in 0..ground_items.len() {
            total_ground_item_weight += ground_items[i].weight;
        }
        let mut total_plant_weight = 0;
        for i in 0..plants.len() {
            total_plant_weight += plants[i].weight;
        }
        
        Self {
            rng: rand::rng(),
            nothing_weight: 150, 

            // Ground items
            ground_item_weight: 50,
            ground_item_total_weight: total_ground_item_weight,
            ground_items: ground_items,

            // Plants
            plant_weight: 1,
            plant_total_weight: total_plant_weight,
            plants: plants,
        }
    }



    pub fn gen_grass(&mut self, cords: [i32; 3], world : &mut World) {
        world.set_world_value(BlockTexture::Grass.id_as_u16(), cords);

        // Get cords to generate item
        let mut above_grass = cords;
        above_grass[2] += 1;
        

        let total_weight = self.nothing_weight + self.plant_weight + self.ground_item_weight;
        let roll = self.rng.random_range(0..total_weight);

        let mut threshold  = 0;


        threshold += self.nothing_weight;
        if roll < threshold  {
            return;
        }

        threshold += self.ground_item_weight;
        if roll < threshold{
            self.gen_ground_item(world, above_grass);
            return;
        }

        threshold += self.plant_weight;
        if roll < threshold {
            self.spawn_plant(world, above_grass);
            return;
        }

    }

    pub fn gen_ground_item(&mut self, world : &mut World, cords: [i32; 3]){
        let mut roll = self.rng.random_range(0..self.ground_item_total_weight);
    
        for item in &self.ground_items {
            if roll < item.weight {
                if world.get_world_value(cords) == BlockTexture::Air.id_as_u16() {
                    world.set_world_value(item.block_type.id() as u16, cords);
                }
                return;
            }
            roll -= item.weight;
        }
    }

    fn spawn_plant(&mut self, world: &mut World, cords: [i32; 3]) {
        let mut roll = self.rng.gen_range(0..self.plant_total_weight);
        
        for plant in &self.plants {
            if roll < plant.weight {
                plant.plant_type.generate(&mut self.rng, world, cords);
                return;
            }
            roll -= plant.weight;
        }
    }


}
