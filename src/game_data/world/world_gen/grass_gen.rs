use crate::game_data::{Types::BlockType, World};
use rand::{random_bool, rngs::ThreadRng, Rng};

struct GroundItem {
    block_type: BlockType,
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
        world.set_world_value(BlockType::Leaves.id_as_u16(), [cords[0] - 1, cords[1], cords[2]]);
        world.set_world_value(BlockType::Leaves.id_as_u16(), [cords[0] + 1, cords[1], cords[2]]);
        world.set_world_value(BlockType::Leaves.id_as_u16(), [cords[0], cords[1] - 1, cords[2]]);
        world.set_world_value(BlockType::Leaves.id_as_u16(), [cords[0], cords[1] + 1, cords[2]]);
        world.set_world_value(BlockType::Leaves.id_as_u16(), [cords[0], cords[1], cords[2] + 1]);
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
            world.set_world_value(BlockType::Leaves.id_as_u16(), current_cords);
            if i == branch_length {
                Self::generate_leaves(world, current_cords);
            }
        }
    }

    fn generate_tree(rng: &mut ThreadRng, world: &mut World, cords: [i32; 3]) {
        // Decide if tree is purple
        let purple_tree_likleyhood = 0.05;
        let mut block_type = BlockType::BrownTrunk.id_as_u16();
        if rng.random_bool(purple_tree_likleyhood) {
            block_type = BlockType::PurpleTrunk.id_as_u16();
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
        if tree_height >= min_branch_height {
            let branch_height = rng.random_range(min_branch_height..max_branch_height);
            Self::generate_branch(world, rng, [cords[0], cords[1], cords[2] + branch_height]);
        }

    }

    fn generate_mushroom(rng: &mut ThreadRng, world: &mut World, cords: [i32; 3]) {

    }

    fn generate_dandelion(rng: &mut ThreadRng, world: &mut World, cords: [i32; 3]) {

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
            GroundItem {block_type: BlockType::white_flowers, weight: 20},
            GroundItem {block_type: BlockType::yellow_flowers, weight: 20},
            GroundItem {block_type: BlockType::flungle, weight: 1},
            GroundItem {block_type: BlockType::mushroom, weight: 3},
            GroundItem {block_type: BlockType::log, weight: 2},
            GroundItem {block_type: BlockType::rock, weight: 3},
        ];
        // Setup Plant generation probabilitys
        let plants:Vec<Plant> = vec![
            Plant {plant_type: PlantType::Tree, weight: 50},
            Plant {plant_type: PlantType::Mushroom, weight: 0},
            Plant {plant_type: PlantType::Dandelion, weight: 0},
        ];


        // Calculate total weights
        let mut total_ground_item_weight = 0;
        for i in 0..ground_items.len() {
            total_ground_item_weight += ground_items[i].weight;
        }
        let mut total_plant_weight = 0;
        for i in 0..plants.len() {
            total_plant_weight += ground_items[i].weight;
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
        world.set_world_value(BlockType::Grass.id_as_u16(), cords);

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
                world.set_world_value(item.block_type.id() as u16, cords);
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