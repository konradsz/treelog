use std::{borrow::Borrow, sync::Arc};
use tokio::sync::{watch, RwLock};

use crate::Content;

pub struct Node {
    parent: watch::Receiver<u32>,
    tx: watch::Sender<u32>,
    rx: watch::Receiver<u32>,
    content: Arc<RwLock<Content>>,
    indices: Vec<u32>,
}

impl Node {
    fn root(parent: watch::Receiver<u32>, content: Arc<RwLock<Content>>) -> Self {
        let (tx, rx) = watch::channel(0);
        Self {
            parent,
            tx,
            rx,
            content,
            indices: Vec::new(),
        }
    }

    pub fn new(parent: watch::Receiver<u32>, content: Arc<RwLock<Content>>, pattern: &str) -> Self {
        let (tx, rx) = watch::channel(0);
        Self {
            parent,
            tx,
            rx,
            content,
            indices: Vec::new(),
        }
    }

    pub async fn watch(&mut self) {
        loop {
            while self.parent.changed().await.is_ok() {
                // let content = self.content.read().await;
                // if content.matches(pattern)
                self.indices.push(*self.parent.borrow());
                // push to children
            }
        }
    }
}
