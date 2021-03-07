use anyhow::Result;
use futures_util::StreamExt;
use inotify::{Inotify, WatchMask};
use std::{
    io::{self, BufRead},
    path::{Path, PathBuf},
};

pub struct Content {
    path: PathBuf,
    data: Vec<String>,
}

impl Content {
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            /*data: io::BufReader::new(File::open(path)?)
            .lines()
            .collect::<io::Result<_>>()?,*/
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

    pub fn get_line(&self, index: u32) -> &str {
        &self.data[index as usize]
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
