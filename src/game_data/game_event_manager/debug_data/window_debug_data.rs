pub enum WindowDebugData {
    OpenWindows(usize),
}

impl WindowDebugData {
    pub fn to_string(&self) -> String {
        match self {
            WindowDebugData::OpenWindows(count) => format!("Windows Open: {}", count),
        }
    }
}