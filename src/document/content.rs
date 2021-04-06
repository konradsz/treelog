use anyhow::Result;
use std::path::{Path, PathBuf};

pub struct Content {
    path: PathBuf,
    data: Vec<String>,
}

impl Content {
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            path,
            data: Vec::new(),
        })
    }

    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn add_line(&mut self, line: String) {
        self.data.push(line);
    }

    #[allow(dead_code)]
    pub fn get_line(&self, index: u32) -> &str {
        &self.data[index as usize]
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
