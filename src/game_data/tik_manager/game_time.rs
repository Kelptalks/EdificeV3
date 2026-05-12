

const TIKS_PER_SECOND: u64 = 60;
const TIKS_PER_MINUTE: u64 = TIKS_PER_SECOND * 60;
const TIKS_PER_HOUR:   u64 = TIKS_PER_MINUTE * 60;
const TIKS_PER_DAY:    u64 = TIKS_PER_HOUR   * 24;


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
            tik:    time,
            second: time / TIKS_PER_SECOND,
            minute: time / TIKS_PER_MINUTE,
            hour:   time / TIKS_PER_HOUR,
            day:    time / TIKS_PER_DAY,

            is_tik:    true,
            is_second: time % TIKS_PER_SECOND == 0,
            is_minute: time % TIKS_PER_MINUTE == 0,
            is_hour:   time % TIKS_PER_HOUR   == 0,
            is_day:    time % TIKS_PER_DAY    == 0,
        }
    }
}

#[derive(Clone)]
pub enum GameTimeInterval {
    Tik,
    Second,

}

impl GameTimeInterval {
    pub fn is_interval(&self, game_time: &GameTime) -> bool {
        match self {
            GameTimeInterval::Tik => true,
            GameTimeInterval::Second => game_time.is_second,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            GameTimeInterval::Tik => "tik",
            GameTimeInterval::Second => "sec",
        }
    }
} 