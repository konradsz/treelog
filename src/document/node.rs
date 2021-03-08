use std::sync::Arc;
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    RwLock,
};

use crate::Content;

pub struct Node {
    parent: Receiver<u32>,
    tx: Sender<u32>,
    rx: Receiver<u32>,
    content: Arc<RwLock<Content>>,
    indices: Vec<u32>,
}

impl Node {
    fn root(parent: Receiver<u32>, content: Arc<RwLock<Content>>) -> Self {
        let (tx, rx) = channel(512);
        Self {
            parent,
            tx,
            rx,
            content,
            indices: Vec::new(),
        }
    }

    pub fn new(parent: Receiver<u32>, content: Arc<RwLock<Content>>, pattern: &str) -> Self {
        let (tx, rx) = channel(512);
        Self {
            parent,
            tx,
            rx,
            content,
            indices: Vec::new(),
        }
    }

    pub async fn watch(&mut self) {
        while let Some(index) = self.parent.recv().await {
            // let content = self.content.read().await;
            // if content.matches(pattern)
            self.indices.push(index);
            // push to children
        }
    }
}
