use crate::game_data::{texture_manager::texture::Texture, player_data::drones::drone_inventory::InventorySlot, types::drone_item::DroneItem};

pub static TOTAL_BLOCKS: u32 = 400;

#[repr(u16)]
#[derive(Copy, Clone, PartialEq)]  // Add these
pub enum BlockTexture {
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
    Flungle = 63,
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
    Selector = 75,
    translucent_green = 76,
    translucent_red = 77,

    SelectorBarLeft = 78,
    SelectorBarRight = 79,

    SelectorBarLeftRed = 80,
    SelectorBarRightRed = 81,

    SelectorVertical = 82,
    SelectorVerticalRed = 83,

    Dot = 84,

    PathingHighlight = 85, 
    
    Blank86 = 86, PuffNorthEast = 87, PuffSouth = 88, PuffSouthEast = 89,
    PuffEast = 90, PuffSouthWest = 91, PuffNorth = 92, PuffNorthWest = 93, PuffWest = 94,
    
    Blank95 = 95, Blank96 = 96, Blank97 = 97, Blank98 = 98, Blank99 = 99,
    Blank100 = 100, Blank101 = 101, Blank102 = 102, Blank103 = 103, Blank104 = 104,
    Blank105 = 105, Blank106 = 106, Blank107 = 107, Blank108 = 108, Blank109 = 109,
    Blank110 = 110, Blank111 = 111, Blank112 = 112, Blank113 = 113, Blank114 = 114,
    Blank115 = 115, Blank116 = 116, Blank117 = 117, Blank118 = 118, Blank119 = 119,
    Blank120 = 120, Blank121 = 121, Blank122 = 122, Blank123 = 123, Blank124 = 124,
    Blank125 = 125, Blank126 = 126, Blank127 = 127, Blank128 = 128, Blank129 = 129,
    Blank130 = 130, Blank131 = 131, Blank132 = 132, Blank133 = 133, Blank134 = 134,
    Blank135 = 135, Blank136 = 136, Blank137 = 137, Blank138 = 138, Blank139 = 139,
    Blank140 = 140, Blank141 = 141, Blank142 = 142, Blank143 = 143, Blank144 = 144,
    Blank145 = 145, Blank146 = 146, Blank147 = 147, Blank148 = 148, Blank149 = 149,
    Blank150 = 150, Blank151 = 151, Blank152 = 152, Blank153 = 153, Blank154 = 154,
    Blank155 = 155, Blank156 = 156, Blank157 = 157, Blank158 = 158, Blank159 = 159,
    Blank160 = 160, Blank161 = 161, Blank162 = 162, Blank163 = 163, Blank164 = 164,
    Blank165 = 165, Blank166 = 166, Blank167 = 167, Blank168 = 168, Blank169 = 169,
    Blank170 = 170, Blank171 = 171, Blank172 = 172, Blank173 = 173, Blank174 = 174,
    Blank175 = 175, Blank176 = 176, Blank177 = 177, Blank178 = 178, Blank179 = 179,
    Blank180 = 180, Blank181 = 181, Blank182 = 182, Blank183 = 183, Blank184 = 184,
    Blank185 = 185, Blank186 = 186, Blank187 = 187, Blank188 = 188, Blank189 = 189,
    Blank190 = 190, Blank191 = 191, Blank192 = 192, Blank193 = 193, Blank194 = 194,
    Blank195 = 195, Blank196 = 196, Blank197 = 197, Blank198 = 198, Blank199 = 199,
    Blank200 = 200, Blank201 = 201, Blank202 = 202, Blank203 = 203, Blank204 = 204,
    Blank205 = 205, Blank206 = 206, Blank207 = 207, Blank208 = 208, Blank209 = 209,
    Blank210 = 210, Blank211 = 211, Blank212 = 212, Blank213 = 213, Blank214 = 214,
    Blank215 = 215, Blank216 = 216, Blank217 = 217, Blank218 = 218, Blank219 = 219,
    Blank220 = 220, Blank221 = 221, Blank222 = 222, Blank223 = 223, Blank224 = 224,
    Blank225 = 225, Blank226 = 226, Blank227 = 227, Blank228 = 228, Blank229 = 229,
    Blank230 = 230, Blank231 = 231, Blank232 = 232, Blank233 = 233, Blank234 = 234,
    Blank235 = 235, Blank236 = 236, Blank237 = 237, Blank238 = 238, Blank239 = 239,
    Blank240 = 240, Blank241 = 241, Blank242 = 242, Blank243 = 243, Blank244 = 244,
    Blank245 = 245, Blank246 = 246, Blank247 = 247, Blank248 = 248, Blank249 = 249,
    Blank250 = 250, Blank251 = 251, Blank252 = 252, Blank253 = 253, Blank254 = 254,
    Blank255 = 255, Blank256 = 256, Blank257 = 257, Blank258 = 258, Blank259 = 259,
    Blank260 = 260, Blank261 = 261, Blank262 = 262, Blank263 = 263, Blank264 = 264,
    Blank265 = 265, Blank266 = 266, Blank267 = 267, Blank268 = 268, Blank269 = 269,
    Blank270 = 270, Blank271 = 271, Blank272 = 272, Blank273 = 273, Blank274 = 274,
    Blank275 = 275, Blank276 = 276, Blank277 = 277, Blank278 = 278, Blank279 = 279,
    Blank280 = 280, Blank281 = 281, Blank282 = 282, Blank283 = 283, Blank284 = 284,
    Blank285 = 285, Blank286 = 286, Blank287 = 287, Blank288 = 288, Blank289 = 289,
    Blank290 = 290, Blank291 = 291, Blank292 = 292, Blank293 = 293, Blank294 = 294,
    Blank295 = 295, Blank296 = 296, Blank297 = 297, Blank298 = 298, Blank299 = 299,
    Blank300 = 300, Blank301 = 301, Blank302 = 302, Blank303 = 303, Blank304 = 304,
    Blank305 = 305, Blank306 = 306, Blank307 = 307, Blank308 = 308, Blank309 = 309,
    Blank310 = 310, Blank311 = 311, Blank312 = 312, Blank313 = 313, Blank314 = 314,
    Blank315 = 315, Blank316 = 316, Blank317 = 317, Blank318 = 318, Blank319 = 319,
    Blank320 = 320, Blank321 = 321, Blank322 = 322, Blank323 = 323, Blank324 = 324,
    Blank325 = 325, Blank326 = 326, Blank327 = 327, Blank328 = 328, Blank329 = 329,
    Blank330 = 330, Blank331 = 331, Blank332 = 332, Blank333 = 333, Blank334 = 334,
    Blank335 = 335, Blank336 = 336, Blank337 = 337, Blank338 = 338, Blank339 = 339,
    Blank340 = 340, Blank341 = 341, Blank342 = 342, Blank343 = 343, Blank344 = 344,
    Blank345 = 345, Blank346 = 346, Blank347 = 347, Blank348 = 348, Blank349 = 349,
    Blank350 = 350, Blank351 = 351, Blank352 = 352, Blank353 = 353, Blank354 = 354,
    Blank355 = 355, Blank356 = 356, Blank357 = 357, Blank358 = 358, Blank359 = 359,
    Blank360 = 360, Blank361 = 361, Blank362 = 362, Blank363 = 363, Blank364 = 364,
    Blank365 = 365, Blank366 = 366, Blank367 = 367, Blank368 = 368, Blank369 = 369,
    Blank370 = 370, Blank371 = 371, Blank372 = 372, Blank373 = 373, Blank374 = 374,
    Blank375 = 375, Blank376 = 376, Blank377 = 377, Blank378 = 378, Blank379 = 379,
    Blank380 = 380, Blank381 = 381, Blank382 = 382, Blank383 = 383, Blank384 = 384,
    Blank385 = 385, Blank386 = 386, Blank387 = 387, Blank388 = 388, Blank389 = 389,
    Blank390 = 390, Blank391 = 391, Blank392 = 392, Blank393 = 393, Blank394 = 394,
    Blank395 = 395, Blank396 = 396, Blank397 = 397, Blank398 = 398, Blank399 = 399,
}

#[derive(Copy, Clone)]
pub struct BlockProperties {
    pub name: &'static str,
    pub transparent: bool,
    pub translucent: bool,
    pub solid: bool,
    pub hardness: u16,
    pub friction: u16,

    pub item: DroneItem,
    pub item_quantity: u32,
}

static BLOCK_PROPERTIES: [BlockProperties; 400] = [
    BlockProperties { name: "Air",                 transparent: true,  translucent: true,  solid: false, hardness: 0,   friction: 1,   item: DroneItem::Ash,         item_quantity: 0   },
    BlockProperties { name: "Stone",              transparent: false, translucent: false, solid: true,  hardness: 100, friction: 5,   item: DroneItem::Stone,        item_quantity: 1   },
    BlockProperties { name: "Grass",              transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 2   },
    BlockProperties { name: "Dirt",               transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 20,  item: DroneItem::Dirt,         item_quantity: 1   },
    BlockProperties { name: "BrownTrunk",         transparent: false, translucent: false, solid: true,  hardness: 60,  friction: 15,  item: DroneItem::BrownLog,     item_quantity: 5   },
    BlockProperties { name: "Leaves",             transparent: false, translucent: false, solid: false, hardness: 20,  friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "PurpleTrunk",        transparent: false, translucent: false, solid: true,  hardness: 60,  friction: 15,  item: DroneItem::PurpleLog,    item_quantity: 5   },
    BlockProperties { name: "Iron",               transparent: false, translucent: false, solid: true,  hardness: 300, friction: 5,   item: DroneItem::IronOar,      item_quantity: 3   },
    BlockProperties { name: "Granite",            transparent: false, translucent: false, solid: true,  hardness: 120, friction: 4,   item: DroneItem::Stone,        item_quantity: 10  },
    BlockProperties { name: "Sand",               transparent: false, translucent: false, solid: true,  hardness: 30,  friction: 40,  item: DroneItem::Sand,         item_quantity: 3   },
    BlockProperties { name: "CopperOre",          transparent: false, translucent: false, solid: true,  hardness: 150, friction: 5,   item: DroneItem::CopperOar,    item_quantity: 3   },
    BlockProperties { name: "PinkFungus",         transparent: false, translucent: false, solid: true,  hardness: 10,  friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "BlueGrass",          transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 5   },
    BlockProperties { name: "MushroomStem",       transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "PinkMushroomBlock",  transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "MudBricks",          transparent: false, translucent: false, solid: true,  hardness: 90,  friction: 10,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "OrangeFungus",       transparent: false, translucent: false, solid: true,  hardness: 10,  friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "StoneBrick",         transparent: false, translucent: false, solid: true,  hardness: 110, friction: 100, item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "FlowerStoneBrick",   transparent: false, translucent: false, solid: true,  hardness: 110, friction: 100, item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Scaffolding",        transparent: false, translucent: false, solid: true,  hardness: 1,   friction: 80,  item: DroneItem::BrownLog,     item_quantity: 1   },
    BlockProperties { name: "PinkCloud",          transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "DandiStem",          transparent: false, translucent: false, solid: true,  hardness: 40,  friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Hive",               transparent: false, translucent: false, solid: true,  hardness: 80,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "CobbleStone",        transparent: false, translucent: false, solid: true,  hardness: 90,  friction: 7,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Magma",              transparent: false, translucent: false, solid: true,  hardness: 200, friction: 20,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Core",               transparent: false, translucent: false, solid: true,  hardness: 500, friction: 120, item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "LBM",                transparent: false, translucent: false, solid: true,  hardness: 100, friction: 90,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "CrackedEarth",       transparent: false, translucent: false, solid: true,  hardness: 70,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Debug",              transparent: false, translucent: false, solid: false, hardness: 0,   friction: 0,   item: DroneItem::TitaniumIngot, item_quantity: 99999 },
    BlockProperties { name: "Water",              transparent: false, translucent: true,  solid: true,  hardness: 0,   friction: 10,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Glass",              transparent: false, translucent: true,  solid: true,  hardness: 50,  friction: 20,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "RedBrick",           transparent: false, translucent: false, solid: true,  hardness: 100, friction: 95,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "DroneControler",     transparent: false, translucent: false, solid: true,  hardness: 150, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "IronOre",            transparent: false, translucent: false, solid: true,  hardness: 150, friction: 5,   item: DroneItem::IronOar,      item_quantity: 1   },
    BlockProperties { name: "BlueMushroom",       transparent: false, translucent: false, solid: true,  hardness: 30,  friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "StorageReceptacle1", transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "StorageReceptacle2", transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "StorageReceptacle3", transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "StorageReceptacle4", transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "StorageReceptacle5", transparent: false, translucent: false, solid: true,  hardness: 100, friction: 80,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "SmokeStack",         transparent: false, translucent: true,  solid: true,  hardness: 200, friction: 50,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "BrownPlanks",        transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "CloudBlock",         transparent: false, translucent: false, solid: true,  hardness: 20,  friction: 30,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "PurplePlanks",       transparent: false, translucent: false, solid: true,  hardness: 50,  friction: 85,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "FurnaceOff",         transparent: false, translucent: false, solid: true,  hardness: 150, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "FurnaceOn",          transparent: false, translucent: false, solid: true,  hardness: 150, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "TitaniumOre",        transparent: false, translucent: false, solid: true,  hardness: 180, friction: 90,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "WormBody",           transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "WormEyesFlat",       transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "WormEyesUp",         transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "WormMouth",          transparent: false, translucent: false, solid: true,  hardness: 100, friction: 75,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "DroneBotLeft",       transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "DroneBotRight",      transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "DroneUpLeft",        transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "DroneUpRight",       transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "DroneDead",          transparent: false, translucent: false, solid: true,  hardness: 200, friction: 60,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Battery1",           transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Battery2",           transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Battery3",           transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Battery4",           transparent: false, translucent: false, solid: true,  hardness: 100, friction: 70,  item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "yellow_flowers",     transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 5   },
    BlockProperties { name: "white_flowers",      transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 5   },
    BlockProperties { name: "mushroom",           transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 7   },
    BlockProperties { name: "flungle",            transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 10  },
    BlockProperties { name: "blulbo",             transparent: false, translucent: true,  solid: false, hardness: 5,   friction: 1,   item: DroneItem::PlantMatter,  item_quantity: 5   },
    BlockProperties { name: "rock",               transparent: false, translucent: true,  solid: false, hardness: 80,  friction: 1,   item: DroneItem::Stone,        item_quantity: 1   },
    BlockProperties { name: "log",                transparent: false, translucent: true,  solid: false, hardness: 60,  friction: 1,   item: DroneItem::BrownLog,     item_quantity: 1   },
    BlockProperties { name: "factory1",           transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70,  item: DroneItem::IronIngot,    item_quantity: 1   },
    BlockProperties { name: "factory2",           transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70,  item: DroneItem::IronIngot,    item_quantity: 1   },
    BlockProperties { name: "factory3",           transparent: false, translucent: false, solid: true,  hardness: 200, friction: 70,  item: DroneItem::IronIngot,    item_quantity: 1   },
    BlockProperties { name: "conveyor1",          transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,    item_quantity: 1   },
    BlockProperties { name: "conveyor2",          transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,    item_quantity: 1   },
    BlockProperties { name: "conveyor3",          transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,    item_quantity: 1   },
    BlockProperties { name: "conveyor4",          transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,    item_quantity: 1   },
    BlockProperties { name: "conveyor5",          transparent: false, translucent: false, solid: true,  hardness: 150, friction: 30,  item: DroneItem::IronIngot,    item_quantity: 1   },
    BlockProperties { name: "Selector",           transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "translucent_green",  transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "translucent_red",    transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "SelectorBarLeft",    transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "SelectorBarRight",   transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "SelectorBarLeftRed", transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "SelectorBarRightRed",transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "SelectorVertical",   transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "SelectorVerticalRed",transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "Dot",                transparent: false, translucent: true,  solid: false, hardness: 0,   friction: 0,   item: DroneItem::PlantMatter,  item_quantity: 1   },
    BlockProperties { name: "PathingHighlight",   transparent: false, translucent: true,  solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank85

    
    // Blank reserved slots (85-399)
    BlockProperties { name: "Blank",              transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank86
    BlockProperties { name: "PuffDown",           transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank87
    BlockProperties { name: "PuffLeftDown",       transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank88
    BlockProperties { name: "PuffLeft",           transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank89
    BlockProperties { name: "BuffUpLeft",         transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank90
    BlockProperties { name: "PuffUp",             transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank91
    BlockProperties { name: "PuffUpRight",        transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank92
    BlockProperties { name: "PuffRight",          transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank93
    BlockProperties { name: "PuffRightDown",      transparent: false, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank94
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank95
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank96
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank97
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank98
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank99
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank100
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank101
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank102
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank103
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank104
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank105
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank106
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank107
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank108
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank109
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank110
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank111
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank112
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank113
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank114
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank115
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank116
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank117
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank118
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank119
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank120
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank121
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank122
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank123
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank124
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank125
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank126
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank127
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank128
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank129
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank130
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank131
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank132
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank133
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank134
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank135
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank136
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank137
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank138
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank139
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank140
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank141
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank142
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank143
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank144
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank145
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank146
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank147
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank148
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank149
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank150
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank151
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank152
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank153
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank154
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank155
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank156
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank157
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank158
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank159
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank160
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank161
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank162
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank163
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank164
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank165
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank166
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank167
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank168
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank169
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank170
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank171
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank172
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank173
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank174
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank175
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank176
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank177
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank178
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank179
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank180
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank181
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank182
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank183
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank184
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank185
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank186
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank187
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank188
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank189
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank190
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank191
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank192
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank193
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank194
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank195
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank196
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank197
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank198
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank199
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank200
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank201
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank202
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank203
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank204
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank205
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank206
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank207
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank208
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank209
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank210
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank211
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank212
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank213
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank214
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank215
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank216
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank217
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank218
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank219
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank220
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank221
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank222
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank223
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank224
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank225
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank226
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank227
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank228
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank229
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank230
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank231
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank232
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank233
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank234
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank235
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank236
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank237
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank238
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank239
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank240
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank241
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank242
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank243
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank244
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank245
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank246
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank247
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank248
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank249
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank250
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank251
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank252
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank253
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank254
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank255
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank256
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank257
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank258
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank259
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank260
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank261
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank262
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank263
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank264
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank265
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank266
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank267
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank268
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank269
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank270
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank271
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank272
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank273
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank274
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank275
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank276
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank277
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank278
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank279
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank280
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank281
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank282
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank283
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank284
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank285
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank286
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank287
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank288
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank289
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank290
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank291
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank292
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank293
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank294
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank295
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank296
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank297
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank298
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank299
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank300
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank301
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank302
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank303
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank304
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank305
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank306
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank307
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank308
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank309
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank310
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank311
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank312
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank313
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank314
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank315
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank316
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank317
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank318
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank319
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank320
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank321
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank322
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank323
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank324
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank325
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank326
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank327
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank328
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank329
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank330
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank331
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank332
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank333
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank334
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank335
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank336
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank337
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank338
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank339
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank340
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank341
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank342
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank343
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank344
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank345
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank346
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank347
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank348
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank349
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank350
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank351
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank352
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank353
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank354
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank355
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank356
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank357
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank358
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank359
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank360
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank361
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank362
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank363
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank364
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank365
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank366
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank367
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank368
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank369
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank370
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank371
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank372
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank373
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank374
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank375
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank376
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank377
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank378
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank379
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank380
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank381
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank382
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank383
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank384
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank385
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank386
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank387
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank388
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank389
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank390
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank391
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank392
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank393
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank394
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank395
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank396
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank397
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank398
    BlockProperties { name: "Blank", transparent: true, translucent: true, solid: false, hardness: 0, friction: 0, item: DroneItem::Ash, item_quantity: 0 }, // Blank399
];

impl BlockTexture {
    pub fn wrap_into_texture(self) -> Texture {
        Texture::BlockTexture(self)
    }
    
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

    #[inline]
    pub fn is_visible(&self) -> bool {
        !self.is_transparent() && (self.is_translucent() || self.is_opaque())
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

    pub fn get_place_cost(&self) -> Vec<InventorySlot> {
        let mut slot = InventorySlot::new();
        
        slot.set_item(self.item());
        slot.set_quantity(self.item_quantity() as i32);

        vec![slot]
    }

    pub fn get_total_blocks() -> u32 {
        return BLOCK_PROPERTIES.len() as u32;
    }

    pub fn from_id(id: u16) -> BlockTexture {
        if id <= BlockTexture::Blank399 as u16 {
            unsafe { std::mem::transmute(id) }
        } else {
            BlockTexture::Air
        }
    }

    pub fn get_name(&self) -> &'static str {
        BLOCK_PROPERTIES[*self as usize].name
    }

    pub fn is_tikable(&self) -> bool {
        match self {
            BlockTexture::Leaves => {
                return true;
            }
            BlockTexture::Grass => {
                return true;
            }
            BlockTexture::Dirt => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn is_block_entity(&self) -> bool {
        match self {
            BlockTexture::DroneBotLeft => {
                return true;
            },
            BlockTexture::DroneBotRight => {
                return true;
            },
            BlockTexture::DroneUpLeft => {
                return true;
            },
            BlockTexture::DroneUpRight => {
                return true;
            },
            _ => {
                return false;
            }
        }
    }
}
