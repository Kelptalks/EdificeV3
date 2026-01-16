-- BlockType enum-like table
local BlockType = {
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

-- Test
-- Reverse lookup table (ID -> name)
local BlockTypeNames = {}
for name, id in pairs(BlockType) do
    BlockTypeNames[id] = name
end

-- Convert block name to ID
-- Returns the ID or nil if not found
function BlockType.to_id(name)
    return BlockType[name]
end

-- Convert block ID to name
-- Returns the name or nil if not found
function BlockType.from_id(id)
    return BlockTypeNames[id]
end

-- Check if a block ID is valid
function BlockType.is_valid_id(id)
    return BlockTypeNames[id] ~= nil
end

-- Check if a block name is valid
function BlockType.is_valid_name(name)
    return BlockType[name] ~= nil
end

-- Check if a block is solid
function BlockType.is_solid(id)
    return rust_block_is_solid(id)
end

return BlockType