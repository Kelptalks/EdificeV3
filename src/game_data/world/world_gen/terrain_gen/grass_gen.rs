use crate::game_data::{types::BlockTexture, world::world::WorldEvent};
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
    fn generate(&self, rng: &mut ThreadRng, cords: [i32; 3]) -> Vec<WorldEvent> {
        match self {
            PlantType::Tree => Self::generate_tree(rng, cords),
            PlantType::Mushroom => Self::generate_mushroom(rng, cords),
            PlantType::Dandelion => Self::generate_dandelion(rng, cords),
        }
    }

    /*#####################
      ## Tree Generation ##
      #####################*/
    fn generate_leaves(cords: [i32; 3]) -> Vec<WorldEvent> {
        vec![
            WorldEvent::ModBlock([cords[0] - 1, cords[1], cords[2]], BlockTexture::Leaves),
            WorldEvent::ModBlock([cords[0] + 1, cords[1], cords[2]], BlockTexture::Leaves),
            WorldEvent::ModBlock([cords[0], cords[1] - 1, cords[2]], BlockTexture::Leaves),
            WorldEvent::ModBlock([cords[0], cords[1] + 1, cords[2]], BlockTexture::Leaves),
            WorldEvent::ModBlock([cords[0], cords[1], cords[2] + 1], BlockTexture::Leaves),
        ]
    }

    fn generate_branch(rng: &mut ThreadRng, cords: [i32; 3]) -> Vec<WorldEvent> {
        let mut events = Vec::new();

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
            events.push(WorldEvent::ModBlock(current_cords, BlockTexture::Leaves));
            if i == branch_length {
                events.extend(Self::generate_leaves(current_cords));
            }
        }

        events
    }

    fn generate_tree(rng: &mut ThreadRng, cords: [i32; 3]) -> Vec<WorldEvent> {
        let mut events = Vec::new();

        // Decide if tree is purple
        let purple_tree_likleyhood = 0.05;
        let mut block_type = BlockTexture::BrownTrunk;
        if rng.random_bool(purple_tree_likleyhood) {
            block_type = BlockTexture::PurpleTrunk;
        }

        // Tree generation propertys
        let min_tree_height = 5;
        let max_tree_height = 15;
        let tree_height = rng.random_range(min_tree_height..max_tree_height);

        // Create trunk
        for z in 0..=tree_height {
            let mut trunk_cor = cords;
            trunk_cor[2] += z;

            events.push(WorldEvent::ModBlock(trunk_cor, block_type));
            if z == tree_height {
                trunk_cor[2] += 1;
                events.extend(Self::generate_leaves(trunk_cor));
            }
        }

        // Branch generation proppertys.
        let min_branch_height = 7;
        let max_branch_height = 10;

        // Generate branch if tree is tall enough
        if tree_height - 3 >= min_branch_height {
            let branch_height = rng.random_range(min_branch_height..max_branch_height);
            events.extend(Self::generate_branch(rng, [cords[0], cords[1], cords[2] + branch_height]));
        }

        events
    }

    fn generate_mushroom(rng: &mut ThreadRng, cords: [i32; 3]) -> Vec<WorldEvent> {
        let mut events = Vec::new();

        let height = rng.gen_range(10..40);
        let stem_radius = (height / (rng.gen_range(5..25))) + 3;

        // Build the stem
        for z in 0..height {
            for x in -stem_radius..stem_radius {
                for y in -stem_radius..stem_radius {
                    let distance = ((x * x + y * y) as f64).sqrt();
                    if distance < stem_radius as f64 {
                        events.push(WorldEvent::ModBlock(
                            [cords[0] + x, cords[1] + y, cords[2] + z],
                            BlockTexture::MushroomStem,
                        ));
                    }
                }
            }
        }

        // Choose block type (blue or pink mushroom)
        let block_type = if rng.gen_range(0..2) == 0 {
            BlockTexture::BlueMushroom
        } else {
            BlockTexture::PinkMushroomBlock
        };

        let mut top_radius = stem_radius + 8 + rng.gen_range(0..5);
        let mut z_mod = 0;

        // Build the mushroom cap
        while top_radius > 2 {
            for x in -top_radius..top_radius {
                for y in -top_radius..top_radius {
                    let distance = ((x * x + y * y) as f64).sqrt();
                    if distance < top_radius as f64 {
                        events.push(WorldEvent::ModBlock(
                            [cords[0] + x, cords[1] + y, cords[2] + z_mod + height],
                            block_type,
                        ));
                    }
                }
            }
            z_mod += 1;
            top_radius -= rng.gen_range(0..2);
        }

        events
    }

    fn generate_dandelion(rng: &mut ThreadRng, cords: [i32; 3]) -> Vec<WorldEvent> {
        let mut events = Vec::new();

        let height = rng.gen_range(12..37);
        let stem_radius = height / 15;

        // Build the stem
        for z in 0..height {
            for x in -stem_radius..stem_radius {
                for y in -stem_radius..stem_radius {
                    let distance = ((x * x + y * y) as f64).sqrt();
                    if distance < stem_radius as f64 {
                        events.push(WorldEvent::ModBlock(
                            [cords[0] + x, cords[1] + y, cords[2] + z],
                            BlockTexture::DandiStem,
                        ));
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
                            events.push(WorldEvent::ModBlock(
                                [cords[0] + x, cords[1] + y, cords[2] + z + puff_top],
                                BlockTexture::PinkCloud,
                            ));
                        }
                    }
                }
            }
        }

        events
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



    pub fn gen_grass(&mut self, cords: [i32; 3]) -> Vec<WorldEvent> {
        let mut events = Vec::new();
        events.push(WorldEvent::ModBlock(cords, BlockTexture::Grass));

        // Get cords to generate item
        let mut above_grass = cords;
        above_grass[2] += 1;


        let total_weight = self.nothing_weight + self.plant_weight + self.ground_item_weight;
        let roll = self.rng.random_range(0..total_weight);

        let mut threshold  = 0;


        threshold += self.nothing_weight;
        if roll < threshold  {
            return events;
        }

        threshold += self.ground_item_weight;
        if roll < threshold{
            events.extend(self.gen_ground_item(above_grass));
            return events;
        }

        threshold += self.plant_weight;
        if roll < threshold {
            events.extend(self.spawn_plant(above_grass));
            return events;
        }

        events
    }

    fn gen_ground_item(&mut self, cords: [i32; 3]) -> Vec<WorldEvent> {
        let mut events = Vec::new();
        let mut roll = self.rng.random_range(0..self.ground_item_total_weight);

        for item in &self.ground_items {
            if roll < item.weight {
                events.push(WorldEvent::ModBlock(cords, item.block_type));
                return events;
            }
            roll -= item.weight;
        }

        events
    }

    fn spawn_plant(&mut self, cords: [i32; 3]) -> Vec<WorldEvent> {
        let mut roll = self.rng.gen_range(0..self.plant_total_weight);

        for plant in &self.plants {
            if roll < plant.weight {
                return plant.plant_type.generate(&mut self.rng, cords);
            }
            roll -= plant.weight;
        }

        Vec::new()
    }


}
