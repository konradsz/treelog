use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

pub struct Content {
    data: Vec<String>,
}

impl Content {
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            data: io::BufReader::new(File::open(path)?)
                .lines()
                .collect::<io::Result<_>>()?,
        })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
