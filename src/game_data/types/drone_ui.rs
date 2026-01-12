#[derive(Clone, Copy)]
pub enum DroneUITexture {
    // Menu
    DroneMiniWindow = 0,
    DroneSpectateWindow = 1,
}

impl DroneUITexture {
    pub fn get_total_UI_elements() -> u32 {
        DroneUITexture::DroneSpectateWindow.get_id() + 1 // DroneSpectateWindow is the last element at index 1, so total is 2
    }
    
    pub fn get_id(&self) -> u32 {
        *self as u32
    }

    pub fn from_id(id: u32) -> Option<DroneUITexture> {
        match id {
            0 => Some(DroneUITexture::DroneMiniWindow),
            1 => Some(DroneUITexture::DroneSpectateWindow),            
            _ => None,
        }
    }

    pub fn ui_texture_to_sprite_sheet_src_rect(&self) -> [u32; 4] {
        match self {
            DroneUITexture::DroneMiniWindow => [480, 128, 120, 28],
            DroneUITexture::DroneSpectateWindow => [720, 0, 263, 196],            
        }
    }

    pub fn get_y_to_x_ratio(&self) -> f32 {
        // Modify the y value based off texture src rect | Could cache this
        let texture_src_rect = self.ui_texture_to_sprite_sheet_src_rect();
        let y_to_x_scale = texture_src_rect [3] as f32 / texture_src_rect[2] as f32;
        return y_to_x_scale;
    }
}



#[derive(Clone, Copy)]
pub enum DroneItemTexture {
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

impl DroneItemTexture {

    pub fn get_total_UI_elements() -> u32 {
        return DroneItemTexture::GoldIngot.get_id() + 1;
    }
    
    pub fn get_id(&self) -> u32 {
        *self as u32
    }

    pub fn from_id(id: u32) -> Option<DroneItemTexture> {
        match id {
            0 => Some(DroneItemTexture::StoneDrill),
            1 => Some(DroneItemTexture::StoneSaw),
            2 => Some(DroneItemTexture::IronDrill),
            3 => Some(DroneItemTexture::IronSaw),
            4 => Some(DroneItemTexture::IronBattery),
            5 => Some(DroneItemTexture::IronStorage),
            6 => Some(DroneItemTexture::IronCamera),
            7 => Some(DroneItemTexture::TitaniumDrill),
            8 => Some(DroneItemTexture::TitaniumSaw),
            9 => Some(DroneItemTexture::TitaniumBattery),
            10 => Some(DroneItemTexture::TitaniumStorage),
            11 => Some(DroneItemTexture::TitaniumCamera),
            12 => Some(DroneItemTexture::TNT),
            13 => Some(DroneItemTexture::Dirt),
            14 => Some(DroneItemTexture::PlantMatter),
            15 => Some(DroneItemTexture::BrownLog),
            16 => Some(DroneItemTexture::Stone),
            17 => Some(DroneItemTexture::StoneBrick),
            18 => Some(DroneItemTexture::ClayBrick),
            19 => Some(DroneItemTexture::IronOar),
            20 => Some(DroneItemTexture::IronIngot),
            21 => Some(DroneItemTexture::CopperOar),
            22 => Some(DroneItemTexture::CopperIngot),
            23 => Some(DroneItemTexture::Sand),
            24 => Some(DroneItemTexture::Glass),
            25 => Some(DroneItemTexture::TitaniumOar),
            26 => Some(DroneItemTexture::TitaniumIngot),
            27 => Some(DroneItemTexture::PurpleLens),
            28 => Some(DroneItemTexture::Ash),
            29 => Some(DroneItemTexture::Sulfur),
            30 => Some(DroneItemTexture::DroneChassis),
            31 => Some(DroneItemTexture::PurpleLog),
            32 => Some(DroneItemTexture::GoldOar),
            33 => Some(DroneItemTexture::GoldIngot),            
            _ => None,
        }
    }

    pub fn ui_texture_to_sprite_sheet_src_rect(&self) -> [u32; 4] {
        let mut start_src_rect = [0, 176, 16, 16];
        start_src_rect[0] += (*self as u32) * 16;
        return start_src_rect;
    }
}

