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
    StorageReceptacle1 = 35,
    StorageReceptacle2 = 36,
    StorageReceptacle3 = 37,
    StorageReceptacle4 = 38,
    StorageReceptacle5 = 39,
    SmokeStack = 40,
    BrownPlanks = 41,
    CloudBlock = 42,
    PurplePlanks = 43,
    FurnaceOff = 44,
    FurnaceOn = 45,
    TitaniumOre = 46,
    WormBody = 47,
    WormEyesFlat = 48,
    WormEyesUp = 49,
    WormMouth = 50,
    DroneBotLeft = 51,
    DroneBotRight = 52,
    DroneUpLeft = 53,
    DroneUpRight = 54,
    DroneDead = 55,
    Battery1 = 56,
    Battery2 = 57,
    Battery3 = 58,
    Battery4 = 59,
    yellow_flowers = 60,
    white_flowers = 61,
    mushroom = 62,
    flungle = 63,
    blulbo = 64,
    rock = 65,
    log = 66,
    factory1 = 67, 
    factory2 = 68,
    factory3 = 69,
    conveyor1 = 70,
    conveyor2 = 71,
    conveyor3 = 72,
    conveyor4 = 73,
    conveyor5 = 74,
    selector = 75,
    translucent_green = 76,
    translucent_red = 77,

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

static BLOCK_PROPERTIES: [BlockProperties; 78] = [
    BlockProperties { transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 1,   item: DroneItem::Ash,          item_quantity: 1},      // Air
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 5,   item: DroneItem::Stone,        item_quantity: 1},    // Stone
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 2},     // Grass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 20,  item: DroneItem::Dirt,         item_quantity: 1 },     // Dirt
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 60,  friction: 15,  item: DroneItem::BrownLog,     item_quantity: 5 },     // BrownTrunk
    BlockProperties { transparent: false, translucent: false, solid: false,  hardness: 20,  friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Leaves
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 60,  friction: 15,  item: DroneItem::PurpleLog,    item_quantity: 5  },     // PurpleTrunk
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 300, friction: 5,   item: DroneItem::IronOar,      item_quantity: 3},     // Iron
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 120, friction: 4,   item: DroneItem::Stone,        item_quantity: 10},    // Granite
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 30,  friction: 40,  item: DroneItem::Sand,         item_quantity: 3},     // Sand
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 5,   item: DroneItem::CopperOar,    item_quantity: 3},    // CopperOre
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 10,  friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1},     // PinkFungus
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 5},     // BlueGrass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // MushroomStem
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // PinkMushroomBlock
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 90,  friction: 10,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // MudBricks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 10,  friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // OrangeFungus
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 110, friction: 100, item: DroneItem::PlantMatter,  item_quantity: 1 },    // StoneBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 110, friction: 100, item: DroneItem::PlantMatter,  item_quantity: 1 },    // FlowerStoneBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 1,  friction: 80,   item: DroneItem::BrownLog,     item_quantity: 1 },     // Scaffolding
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1},     // PinkCloud
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1},     // DandiStem
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 80,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Hive
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 90,  friction: 7,   item: DroneItem::PlantMatter,  item_quantity: 1},    // CobbleStone
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 20,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Magma
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 500, friction: 120, item: DroneItem::PlantMatter,  item_quantity: 1 },    // Core
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 90,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // LBM
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 70,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // CrackedEarth
    BlockProperties { transparent: false, translucent: false, solid: false,  hardness: 0,   friction: 0,  item: DroneItem::TitaniumIngot,  item_quantity: 99999 },     // Debug
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 0,   friction: 10,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Water
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 50,  friction: 20,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Glass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 95,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // RedBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // DroneControler
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 5,   item: DroneItem::IronOar,      item_quantity: 1  },    // IronOre
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 30,  friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // BlueMushroom
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // StorageReceptacle1
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // StorageReceptacle2
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // StorageReceptacle3
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // StorageReceptacle4
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // StorageReceptacle5
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 200, friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // SmokeStack
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // BrownPlanks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // CloudBlock
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // PurplePlanks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // FurnaceOff
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // FurnaceOn
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 180, friction: 90,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // TitaniumOre
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // WormBody
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // WormEyesFlat
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // WormEyesUp
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // WormMouth
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // DroneBotLeft
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // DroneBotRight
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // DroneUpLeft
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // DroneUpRight
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // DroneDead
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Battery1
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Battery2
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Battery3
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1 },     // Battery4
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 1, item: DroneItem::PlantMatter,  item_quantity: 5 },     // yellow_flowers
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 1, item: DroneItem::PlantMatter,  item_quantity: 5 },     // white_flowers
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 1, item: DroneItem::PlantMatter,  item_quantity: 7 },     // mushroom
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 1, item: DroneItem::PlantMatter,  item_quantity: 10 },     // flungle
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 1, item: DroneItem::PlantMatter,  item_quantity: 5 },     // blulbo
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 80,  friction: 1, item: DroneItem::Stone,       item_quantity: 1},    // rock
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 60,  friction: 1, item: DroneItem::BrownLog,    item_quantity: 1 },     // log
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70,  item: DroneItem::IronIngot,   item_quantity: 1 },     // factory1
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70,  item: DroneItem::IronIngot,   item_quantity: 1 },     // factory2
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70,  item: DroneItem::IronIngot,   item_quantity: 1},     // factory3
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,   item_quantity: 1},     // conveyor1
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,   item_quantity: 1 },     // conveyor2
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,   item_quantity: 1 },     // conveyor3
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,   item_quantity: 1 },     // conveyor4
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,   item_quantity: 1 },     // conveyor5
    BlockProperties { transparent: false, translucent: true,  solid: false,  hardness: 0, friction: 0,  item: DroneItem::PlantMatter,   item_quantity: 1 },     // Selector
    BlockProperties { transparent: false, translucent: true,  solid: false,  hardness: 0, friction: 0,  item: DroneItem::PlantMatter,   item_quantity: 1 },     // Translucent Green
    BlockProperties { transparent: false, translucent: true,  solid: false,  hardness: 0, friction: 0,  item: DroneItem::PlantMatter,   item_quantity: 1 },     // Translucent Red
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
        if id < BlockType::conveyor5 as u16 {
            unsafe { std::mem::transmute(id) }
        } else {
            BlockType::Air
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
