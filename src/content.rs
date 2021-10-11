pub trait Content {
    fn add_line(&mut self, line: String);
    fn get_line(&self, index: usize) -> &str;
}

pub struct TextContent {
    data: Vec<String>,
}

impl TextContent {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Content for TextContent {
    fn add_line(&mut self, line: String) {
        self.data.push(line);
    }

    fn get_line(&self, index: usize) -> &str {
        &self.data[index]
    }
}
