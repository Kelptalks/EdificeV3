pub struct WorldDebugData {
    pub chunks_loaded: u32,
    pub chunks_generated: u32,
    pub chunks_to_unload: u32,
}

impl WorldDebugData {
    pub fn new() -> WorldDebugData {
        WorldDebugData {
            chunks_loaded: 0,
            chunks_generated: 0,
            chunks_to_unload: 0,
        }
    }
}