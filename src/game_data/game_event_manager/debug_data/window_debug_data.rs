pub struct WindowDebugData {
    pub open_windows: usize,
}

impl WindowDebugData {
    pub fn new() -> WindowDebugData {
        WindowDebugData {
            open_windows: 0,
        }
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        let mut strings = Vec::new();

        strings.push(format!("open_windows: {}", self.open_windows));

        strings
    }
}