use crate::game_data::{tik_manager::drones::drone::Drone, types::drone_item::DroneItem};


pub static TOTAL_BLOCKS: u32 = 100;

#[repr(u16)]
#[derive(Copy, Clone, PartialEq)]  // Add these
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    BrownTrunk = 4,
    Leaves = 5,
    PurpleTrunk = 6,
    Iron = 7,
    Granite = 8,
    Sand = 9,
    CopperOre = 10,
    PinkFungus = 11,
    BlueGrass = 12,
    MushroomStem = 13,
    PinkMushroomBlock = 14,
    MudBricks = 15,
    OrangeFungus = 16,
    StoneBrick = 17,
    FlowerStoneBrick = 18,
    Scaffolding = 19,
    PinkCloud = 20,
    DandiStem = 21,
    Hive = 22,
    CobbleStone = 23,
    Magma = 24,
    Core = 25,
    LBM = 26,
    CrackedEarth = 27,
    Debug = 28,
    Water = 29,
    Glass = 30,
    RedBrick = 31,
    DroneControler = 32,
    IronOre = 33,
    BlueMushroom = 34,
    StorageReceptacle = 35,
    SmokeStack = 40,
    BrownPlanks = 41,
    CloudBlock = 42,
    PurplePlanks = 43,
    FurnaceOff = 44,
    FurnaceOn = 45,
    TitaniumOre = 46,
    Battery = 56,
    yellow_flowers = 60,
    white_flowers = 61,
    mushroom = 62,
    flungle = 63,
    blulbo = 64,
    Rock = 65,
    log = 66,
    Factory = 67, 
    Conveyor = 70,
}

#[derive(Copy, Clone)]
pub struct BlockProperties {
    pub transparent: bool,
    pub translucent: bool,
    pub solid: bool,
    pub hardness: u16,
    pub friction: u16,
    
    pub item: DroneItem,
    pub item_quantity: u32,
}

static BLOCK_PROPERTIES: [BlockProperties; 71] = [
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 1,   item: DroneItem::Ash,          item_quantity: 1 },  // 0  Air
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 5,   item: DroneItem::Stone,        item_quantity: 1 },  // 1  Stone
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 2 },  // 2  Grass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 20,  item: DroneItem::Dirt,         item_quantity: 1 },  // 3  Dirt
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 60,  friction: 15,  item: DroneItem::BrownLog,     item_quantity: 5 },  // 4  BrownTrunk
    BlockProperties { transparent: false, translucent: false, solid: false, hardness: 20,  friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 5  Leaves
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 60,  friction: 15,  item: DroneItem::PurpleLog,    item_quantity: 5 },  // 6  PurpleTrunk
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 300, friction: 5,   item: DroneItem::IronOar,      item_quantity: 3 },  // 7  Iron
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 120, friction: 4,   item: DroneItem::Stone,        item_quantity: 10 }, // 8  Granite
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 30,  friction: 40,  item: DroneItem::Sand,         item_quantity: 3 },  // 9  Sand
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 5,   item: DroneItem::CopperOar,    item_quantity: 3 },  // 10 CopperOre
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 10,  friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 11 PinkFungus
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 5 },  // 12 BlueGrass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 13 MushroomStem
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 14 PinkMushroomBlock
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 90,  friction: 10,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 15 MudBricks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 10,  friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 16 OrangeFungus
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 110, friction: 100, item: DroneItem::PlantMatter,  item_quantity: 1 },  // 17 StoneBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 110, friction: 100, item: DroneItem::PlantMatter,  item_quantity: 1 },  // 18 FlowerStoneBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 1,   friction: 80,  item: DroneItem::BrownLog,     item_quantity: 1 },  // 19 Scaffolding
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 20 PinkCloud
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 21 DandiStem
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 80,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 22 Hive
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 90,  friction: 7,   item: DroneItem::PlantMatter,  item_quantity: 1 },  // 23 CobbleStone
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 20,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 24 Magma
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 500, friction: 120, item: DroneItem::PlantMatter,  item_quantity: 1 },  // 25 Core
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 90,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 26 LBM
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 70,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 27 CrackedEarth
    BlockProperties { transparent: false, translucent: false, solid: false, hardness: 0,   friction: 0,   item: DroneItem::TitaniumIngot, item_quantity: 99999 }, // 28 Debug
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 0,   friction: 10,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 29 Water
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 50,  friction: 20,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 30 Glass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 95,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 31 RedBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 32 DroneControler
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 5,   item: DroneItem::IronOar,      item_quantity: 1 },  // 33 IronOre
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 30,  friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 34 BlueMushroom
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 35 StorageReceptacle
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 36 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 37 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 38 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 39 (unused)
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 200, friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 40 SmokeStack
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 41 BrownPlanks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 42 CloudBlock
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 43 PurplePlanks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 44 FurnaceOff
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 45 FurnaceOn
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 180, friction: 90,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 46 TitaniumOre
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 47 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 48 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 49 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 50 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 51 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 52 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 53 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 54 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 55 (unused)
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },  // 56 Battery
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 57 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 58 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 59 (unused)
    BlockProperties { transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 5 },  // 60 yellow_flowers
    BlockProperties { transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 5 },  // 61 white_flowers
    BlockProperties { transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 7 },  // 62 mushroom
    BlockProperties { transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 10 }, // 63 flungle
    BlockProperties { transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 5 },  // 64 blulbo
    BlockProperties { transparent: false, translucent: true,  solid: false, hardness: 80,  friction: 1,   item: DroneItem::Stone,        item_quantity: 1 },  // 65 Rock
    BlockProperties { transparent: false, translucent: true,  solid: false, hardness: 60,  friction: 1,   item: DroneItem::BrownLog,     item_quantity: 1 },  // 66 log
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70,  item: DroneItem::IronIngot,    item_quantity: 1 },  // 67 Factory
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 68 (unused)
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::Ash,          item_quantity: 0 },  // 69 (unused)
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,    item_quantity: 1 },  // 70 Conveyor
];

impl BlockType {
    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn id_as_usize(&self) -> usize {
        *self as usize
    }

    pub fn id_as_u16(&self) -> u16 {
        *self as u16
    }

    pub fn is_transparent(&self) -> bool {
        BLOCK_PROPERTIES[*self as usize].transparent
    }

    pub fn is_translucent(&self) -> bool {
        BLOCK_PROPERTIES[*self as usize].translucent
    }

    #[inline]
    pub fn is_opaque(&self) -> bool {
        !self.is_transparent() && !self.is_translucent()
    }

    pub fn is_solid(&self) -> bool {
        BLOCK_PROPERTIES[*self as usize].solid
    }

    pub fn hardness(&self) -> u16 {
        BLOCK_PROPERTIES[*self as usize].hardness
    }

    pub fn friction(&self) -> u16 {
        BLOCK_PROPERTIES[*self as usize].friction
    }

    pub fn item(&self) -> DroneItem {
        BLOCK_PROPERTIES[*self as usize].item
    }

    pub fn item_quantity(&self) -> u32 {
        BLOCK_PROPERTIES[*self as usize].item_quantity
    }

    pub fn get_total_blocks() -> u32 {
        return BLOCK_PROPERTIES.len() as u32;
    }

    pub fn from_id(id: u16) -> BlockType {
        match id {
            0  => BlockType::Air,
            1  => BlockType::Stone,
            2  => BlockType::Grass,
            3  => BlockType::Dirt,
            4  => BlockType::BrownTrunk,
            5  => BlockType::Leaves,
            6  => BlockType::PurpleTrunk,
            7  => BlockType::Iron,
            8  => BlockType::Granite,
            9  => BlockType::Sand,
            10 => BlockType::CopperOre,
            11 => BlockType::PinkFungus,
            12 => BlockType::BlueGrass,
            13 => BlockType::MushroomStem,
            14 => BlockType::PinkMushroomBlock,
            15 => BlockType::MudBricks,
            16 => BlockType::OrangeFungus,
            17 => BlockType::StoneBrick,
            18 => BlockType::FlowerStoneBrick,
            19 => BlockType::Scaffolding,
            20 => BlockType::PinkCloud,
            21 => BlockType::DandiStem,
            22 => BlockType::Hive,
            23 => BlockType::CobbleStone,
            24 => BlockType::Magma,
            25 => BlockType::Core,
            26 => BlockType::LBM,
            27 => BlockType::CrackedEarth,
            28 => BlockType::Debug,
            29 => BlockType::Water,
            30 => BlockType::Glass,
            31 => BlockType::RedBrick,
            32 => BlockType::DroneControler,
            33 => BlockType::IronOre,
            34 => BlockType::BlueMushroom,
            35 => BlockType::StorageReceptacle,
            40 => BlockType::SmokeStack,
            41 => BlockType::BrownPlanks,
            42 => BlockType::CloudBlock,
            43 => BlockType::PurplePlanks,
            44 => BlockType::FurnaceOff,
            45 => BlockType::FurnaceOn,
            46 => BlockType::TitaniumOre,
            56 => BlockType::Battery,
            60 => BlockType::yellow_flowers,
            61 => BlockType::white_flowers,
            62 => BlockType::mushroom,
            63 => BlockType::flungle,
            64 => BlockType::blulbo,
            65 => BlockType::Rock,
            66 => BlockType::log,
            67 => BlockType::Factory,
            70 => BlockType::Conveyor,
            _  => BlockType::Air,
        }
    }

    pub fn is_tikable(&self) -> bool {
        match self {
            BlockType::Leaves => {
                return true;
            }
            BlockType::Grass => {
                return true;
            }
            BlockType::Dirt => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
