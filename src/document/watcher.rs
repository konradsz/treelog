use anyhow::Result;
use futures_util::StreamExt;
use inotify::{Inotify, WatchMask};
use std::sync::Arc;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    sync::{
        watch::{channel, Receiver, Sender},
        RwLock,
    },
};

use crate::Content;

pub struct Watcher {
    content: Arc<RwLock<Content>>,
    indices: Arc<RwLock<Vec<usize>>>,
    tx: Sender<usize>,
}

impl Watcher {
    pub fn new(content: Arc<RwLock<Content>>) -> (Self, Arc<RwLock<Vec<usize>>>, Receiver<usize>) {
        let indices = Arc::new(RwLock::new(Vec::new()));
        let (tx, rx) = channel(0);
        (
            Self {
                content,
                indices: indices.clone(),
                tx,
            },
            indices,
            rx,
        )
    }

    pub async fn watch(&self) -> Result<()> {
        let content = self.content.read().await;
        let file = File::open(content.get_path()).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut inotify = Inotify::init()?;
        inotify.add_watch(content.get_path(), WatchMask::CLOSE_WRITE)?;
        let mut event_buffer = [0u8; 32];

        drop(content);

        let mut read_lines = 0;
        loop {
            while let Some(line) = lines.next_line().await? {
                if !line.is_empty() {
                    let mut content = self.content.write().await;
                    content.add_line(line);
                    let mut indices_write_lock = self.indices.write().await;
                    indices_write_lock.push(read_lines);
                    self.tx.send(read_lines)?;
                    read_lines += 1;
                }
            }

            let mut stream = inotify.event_stream(&mut event_buffer)?;
            stream.next().await;
        }
    }
}
