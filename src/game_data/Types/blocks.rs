
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

static TRANSPARENT: [bool; 75] = [
    true,  // Air
    false, // Stone
    false, // Grass
    false, // Dirt
    false, // BrownTrunk
    false, // Leaves
    false, // PurpleTrunk
    false, // Iron
    false, // Granite
    false, // Sand
    false, // CopperOre
    false, // PinkFungus
    false, // BlueGrass
    false, // MushroomStem
    false, // PinkMushroomBlock
    false, // MudBricks
    false, // OrangeFungus
    false, // StoneBrick
    false, // FlowerStoneBrick
    false, // Scaffolding
    false, // PinkCloud
    false, // DandiStem
    false, // Hive
    false, // CobbleStone
    false, // Magma
    false, // Core
    false, // LBM
    false, // CrackedEarth
    false, // Debug
    false, // Water
    false,  // Glass
    false, // RedBrick
    false, // DroneControler
    false, // IronOre
    false, // BlueMushroom
    false, // StorageReceptacle1
    false, // StorageReceptacle2
    false, // StorageReceptacle3
    false, // StorageReceptacle4
    false, // StorageReceptacle5
    false, // SmokeStack
    false, // BrownPlanks
    false, // CloudBlock
    false, // PurplePlanks
    false, // FurnaceOff
    false, // FurnaceOn
    false, // TitaniumOre
    false, // WormBody
    false, // WormEyesFlat
    false, // WormEyesUp
    false, // WormMouth
    false, // DroneBotLeft
    false, // DroneBotRight
    false, // DroneUpLeft
    false, // DroneUpRight
    false, // DroneDead
    false, // Battery1
    false, // Battery2
    false, // Battery3
    false, // Battery4
    false, // yellow_flowers
    false, // white_flowers
    false, // mushroom
    false, // flungle
    false, // blulbo
    false, // rock
    false, // log
    false, // factory1
    false, // factory2
    false, // factory3
    false, // conveyor1
    false, // conveyor2
    false, // conveyor3
    false, // conveyor4
    false, // conveyor5
];

static TRANSLUCENT: [bool; 75] = [
    true,  // Air
    false, // Stone
    false, // Grass
    false, // Dirt
    false, // BrownTrunk
    false, // Leaves
    false, // PurpleTrunk
    false, // Iron
    false, // Granite
    false, // Sand
    false, // CopperOre
    false, // PinkFungus
    false, // BlueGrass
    false, // MushroomStem
    false, // PinkMushroomBlock
    false, // MudBricks
    false, // OrangeFungus
    false, // StoneBrick
    false, // FlowerStoneBrick
    false, // Scaffolding
    false, // PinkCloud
    false, // DandiStem
    false, // Hive
    false, // CobbleStone
    false, // Magma
    false, // Core
    false, // LBM
    false, // CrackedEarth
    false, // Debug
    true,  // Water
    true,  // Glass
    false, // RedBrick
    false, // DroneControler
    false, // IronOre
    false, // BlueMushroom
    false, // StorageReceptacle1
    false, // StorageReceptacle2
    false, // StorageReceptacle3
    false, // StorageReceptacle4
    false, // StorageReceptacle5
    true, // SmokeStack
    false, // BrownPlanks
    false, // CloudBlock
    false, // PurplePlanks
    false, // FurnaceOff
    false, // FurnaceOn
    false, // TitaniumOre
    false, // WormBody
    false, // WormEyesFlat
    false, // WormEyesUp
    false, // WormMouth
    false, // DroneBotLeft
    false, // DroneBotRight
    false, // DroneUpLeft
    false, // DroneUpRight
    false, // DroneDead
    false, // Battery1
    false, // Battery2
    false, // Battery3
    false, // Battery4
    true, // yellow_flowers
    true, // white_flowers
    true, // mushroom
    true, // flungle
    true, // blulbo
    true, // rock
    true, // log
    false, // factory1
    false, // factory2
    false, // factory3
    false, // conveyor1
    false, // conveyor2
    false, // conveyor3
    false, // conveyor4
    false, // conveyor5
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

    pub fn is_transparent(&self) -> bool
    {
        return TRANSPARENT[*self as usize]
    }

    pub fn is_translucent(&self) -> bool
    {
        return TRANSLUCENT[*self as usize]
    }

    pub fn get_total_blocks() -> u32 {
        return TOTAL_BLOCKS;
    }

    pub fn from_id(id: u16) -> BlockType {
        if id < BlockType::conveyor5 as u16 {
            return unsafe { std::mem::transmute(id) }
        }
        else {
            return BlockType::Debug;
        }

    }
}
