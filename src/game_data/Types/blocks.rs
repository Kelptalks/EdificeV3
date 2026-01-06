
pub static TOTAL_BLOCKS: u32 = 100;

#[repr(u16)]
#[derive(Copy, Clone)]  // Add these
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
}

#[derive(Copy, Clone)]
pub struct BlockProperties {
    pub transparent: bool,
    pub translucent: bool,
    pub solid: bool,
    pub hardness: u16,
    pub friction: u16,
}

static BLOCK_PROPERTIES: [BlockProperties; 75] = [
    BlockProperties { transparent: true,  translucent: true,  solid: false,   hardness: 0,   friction: 0 },      // Air
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 5 },    // Stone
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 30 },     // Grass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 20 },     // Dirt
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 60,  friction: 15 },     // BrownTrunk
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 60 },     // Leaves
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 60,  friction: 15 },     // PurpleTrunk
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 300, friction: 5 },     // Iron
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 120, friction: 4 },    // Granite
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 30,  friction: 40 },     // Sand
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 5 },    // CopperOre
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 10,  friction: 50 },     // PinkFungus
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 80 },     // BlueGrass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 30 },     // MushroomStem
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 30 },     // PinkMushroomBlock
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 90,  friction: 10 },     // MudBricks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 10,  friction: 50 },     // OrangeFungus
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 110, friction: 100 },    // StoneBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 110, friction: 100 },    // FlowerStoneBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 1,  friction: 80 },     // Scaffolding
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 30 },     // PinkCloud
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 70 },     // DandiStem
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 80,  friction: 85 },     // Hive
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 90,  friction: 7 },    // CobbleStone
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 20 },     // Magma
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 500, friction: 120 },    // Core
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 90 },     // LBM
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 70,  friction: 85 },     // CrackedEarth
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 1,   friction: 50 },     // Debug
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 0,   friction: 10 },     // Water
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 50,  friction: 20 },     // Glass
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 95 },     // RedBrick
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 80 },     // DroneControler
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 5  },    // IronOre
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 30,  friction: 60 },     // BlueMushroom
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80 },     // StorageReceptacle1
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80 },     // StorageReceptacle2
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80 },     // StorageReceptacle3
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80 },     // StorageReceptacle4
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80 },     // StorageReceptacle5
    BlockProperties { transparent: false, translucent: true,  solid: true,  hardness: 200, friction: 50 },     // SmokeStack
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 85 },     // BrownPlanks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 30 },     // CloudBlock
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 85 },     // PurplePlanks
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 70 },     // FurnaceOff
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 70 },     // FurnaceOn
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 180, friction: 90 },     // TitaniumOre
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75 },     // WormBody
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75 },     // WormEyesFlat
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75 },     // WormEyesUp
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75 },     // WormMouth
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60 },     // DroneBotLeft
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60 },     // DroneBotRight
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60 },     // DroneUpLeft
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60 },     // DroneUpRight
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60 },     // DroneDead
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70 },     // Battery1
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70 },     // Battery2
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70 },     // Battery3
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70 },     // Battery4
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 40 },     // yellow_flowers
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 40 },     // white_flowers
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 50 },     // mushroom
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 45 },     // flungle
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 5,   friction: 45 },     // blulbo
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 80,  friction: 120 },    // rock
    BlockProperties { transparent: false, translucent: true,  solid: false,   hardness: 60,  friction: 90 },     // log
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70 },     // factory1
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70 },     // factory2
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70 },     // factory3
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30 },     // conveyor1
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30 },     // conveyor2
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30 },     // conveyor3
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30 },     // conveyor4
    BlockProperties { transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30 },     // conveyor5
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

    pub fn is_solid(&self) -> bool {
        BLOCK_PROPERTIES[*self as usize].solid
    }

    pub fn hardness(&self) -> u16 {
        BLOCK_PROPERTIES[*self as usize].hardness
    }

    pub fn friction(&self) -> u16 {
        BLOCK_PROPERTIES[*self as usize].friction
    }

    pub fn get_total_blocks() -> u32 {
        75
    }

    pub fn from_id(id: u16) -> BlockType {
        if id < BlockType::conveyor5 as u16 {
            unsafe { std::mem::transmute(id) }
        } else {
            BlockType::Debug
        }
    }
}
