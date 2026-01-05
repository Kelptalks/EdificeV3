pub struct WorldConfig {
    scale: u32,
    height_variation: u32,
}

impl WorldConfig {
    pub fn new() -> Self {
        WorldConfig {
            scale: 200,
            height_variation: 100,
        }
    }

    pub fn set_scale(&mut self, scale: u32) {
        self.scale = scale;
    }
    pub fn get_scale(&self) -> u32 {
        self.scale
    }

    pub fn get_height_variation(&self) -> u32 {
        self.height_variation
    }
}