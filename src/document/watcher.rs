use anyhow::Result;
use futures_util::StreamExt;
use inotify::{Inotify, WatchMask};
use std::sync::Arc;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    sync::{watch, RwLock},
};

use crate::Content;

pub struct Watcher {
    content: Arc<RwLock<Content>>,
    tx: watch::Sender<u32>,
    rx: watch::Receiver<u32>,
}

impl Watcher {
    pub fn new(content: Arc<RwLock<Content>>) -> Self {
        let (tx, rx) = watch::channel(0);
        Self { content, tx, rx }
    }

    pub fn new_receiver(&self) -> watch::Receiver<u32> {
        self.rx.clone()
    }

    pub async fn watch(&self) -> Result<()> {
        let content = self.content.read().await;
        let file = File::open(content.get_path()).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut inotify = Inotify::init()?;
        inotify.add_watch(content.get_path(), WatchMask::MODIFY)?;
        let mut event_buffer = [0u8; 32];

        drop(content);

        let mut read_lines = 0;
        loop {
            while let Some(line) = lines.next_line().await? {
                let mut content = self.content.write().await;
                content.add_line(line);
                self.tx.send(read_lines)?;
                read_lines += 1;
            }

            let mut stream = inotify.event_stream(&mut event_buffer)?;
            stream.next().await;
        }
    }
}
