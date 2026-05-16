pub struct DebugData {
    // Ordered list of (category, lines) so tabs appear in first-use order.
    data: Vec<(String, Vec<String>)>,
}

impl DebugData {
    pub fn new() -> DebugData {
        DebugData { data: Vec::new() }
    }

    /// Append a line to `category`, creating the category if it doesn't exist yet.
    pub fn record(&mut self, category: &str, info: String) {
        if let Some((_, lines)) = self.data.iter_mut().find(|(c, _)| c == category) {
            lines.push(info);
        } else {
            self.data.push((category.to_string(), vec![info]));
        }
    }

    /// Clear all lines for `category` (keeps the tab alive so it doesn't flicker).
    pub fn clear(&mut self, category: &str) {
        if let Some((_, lines)) = self.data.iter_mut().find(|(c, _)| c == category) {
            lines.clear();
        }
    }

    pub fn categories(&self) -> impl Iterator<Item = &str> {
        self.data.iter().map(|(c, _)| c.as_str())
    }

    pub fn get_data(&self, category: &str) -> &[String] {
        self.data
            .iter()
            .find(|(c, _)| c == category)
            .map(|(_, lines)| lines.as_slice())
            .unwrap_or(&[])
    }
}
