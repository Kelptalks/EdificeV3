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

    pub fn from_id(id: u32) -> DroneItemTexture {
        match id {
            0 => DroneItemTexture::StoneDrill,
            1 => DroneItemTexture::StoneSaw,
            2 => DroneItemTexture::IronDrill,
            3 => DroneItemTexture::IronSaw,
            4 => DroneItemTexture::IronBattery,
            5 => DroneItemTexture::IronStorage,
            6 => DroneItemTexture::IronCamera,
            7 => DroneItemTexture::TitaniumDrill,
            8 => DroneItemTexture::TitaniumSaw,
            9 => DroneItemTexture::TitaniumBattery,
            10 => DroneItemTexture::TitaniumStorage,
            11 => DroneItemTexture::TitaniumCamera,
            12 => DroneItemTexture::TNT,
            13 => DroneItemTexture::Dirt,
            14 => DroneItemTexture::PlantMatter,
            15 => DroneItemTexture::BrownLog,
            16 => DroneItemTexture::Stone,
            17 => DroneItemTexture::StoneBrick,
            18 => DroneItemTexture::ClayBrick,
            19 => DroneItemTexture::IronOar,
            20 => DroneItemTexture::IronIngot,
            21 => DroneItemTexture::CopperOar,
            22 => DroneItemTexture::CopperIngot,
            23 => DroneItemTexture::Sand,
            24 => DroneItemTexture::Glass,
            25 => DroneItemTexture::TitaniumOar,
            26 => DroneItemTexture::TitaniumIngot,
            27 => DroneItemTexture::PurpleLens,
            28 => DroneItemTexture::Ash,
            29 => DroneItemTexture::Sulfur,
            30 => DroneItemTexture::DroneChassis,
            31 => DroneItemTexture::PurpleLog,
            32 => DroneItemTexture::GoldOar,
            33 => DroneItemTexture::GoldIngot,
            _ => DroneItemTexture::DroneChassis,
        }
    }

    pub fn ui_texture_to_sprite_sheet_src_rect(&self) -> [u32; 4] {
        let mut start_src_rect = [0, 176, 16, 16];
        start_src_rect[0] += (*self as u32) * 16;
        return start_src_rect;
    }
}

