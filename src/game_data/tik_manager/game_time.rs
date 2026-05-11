pub struct GameTime {
    
    
    
    
    
    pub tik: u64,
    pub second: u64,
    pub minute: u64,
    pub hour: u64,
    pub day: u64,
    
    
    
    
    pub is_tik: bool,
    pub is_second: bool,
    pub is_minute: bool,
    pub is_hour: bool,
    pub is_day: bool,
}

impl GameTime {
    pub fn new(time: u64) -> GameTime {
        GameTime {
            tik: time,
            second: time / 60,
            minute: time / (60 * 60), 
            hour: time / (60 * 60 * 60),
            day: time / (60 * 60 * 60 * 24),
            
            
            is_tik: true,
            is_second: time % 60 == 0,
            is_minute: time % 60 * 60 == 0, 
            is_hour: time % (60 * 60 * 60) == 0,
            is_day: time % (60 * 60 * 60 * 24) == 0,
        }
    }
}