-- DroneItem enum-like table
local DroneItem = {
    StoneDrill = 0,
    StoneSaw = 1,
    IronDrill = 2,
    IronSaw = 3,
    IronBattery = 4,
    IronStorage = 5,
    IronCamera = 6,
    TitaniumDrill = 7,
    TitaniumSaw = 8,
    TitaniumBattery = 9,
    TitaniumStorage = 10,
    TitaniumCamera = 11,
    TNT = 12,
    Dirt = 13,
    PlantMatter = 14,
    BrownLog = 15,
    Stone = 16,
    StoneBrick = 17,
    ClayBrick = 18,
    IronOar = 19,
    IronIngot = 20,
    CopperOar = 21,
    CopperIngot = 22,
    Sand = 23,
    Glass = 24,
    TitaniumOar = 25,
    TitaniumIngot = 26,
    PurpleLens = 27,
    Ash = 28,
    Sulfur = 29,
    DroneChassis = 30,
    PurpleLog = 31,
    GoldOar = 32,
    GoldIngot = 33,
}

-- Reverse lookup table (ID -> name)
local DroneItemNames = {}
for name, id in pairs(DroneItem) do
    DroneItemNames[id] = name
end

-- Convert item name to ID
-- Returns the ID or nil if not found
function DroneItem.to_id(name)
    return DroneItem[name]
end

-- Convert item ID to name
-- Returns the name or nil if not found
function DroneItem.from_id(id)
    return DroneItemNames[id]
end

-- Check if an item ID is valid
function DroneItem.is_valid_id(id)
    return DroneItemNames[id] ~= nil
end

-- Check if an item name is valid
function DroneItem.is_valid_name(name)
    return DroneItem[name] ~= nil
end

return DroneItem
