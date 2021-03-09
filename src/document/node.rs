use anyhow::Result;
use async_channel::{unbounded, Receiver, Sender};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::Content;

pub struct Node {
    parent_channel: Receiver<u32>,
    children_channel: Sender<u32>,
    content: Arc<RwLock<Content>>,
    indices: Vec<u32>,
}

impl Node {
    pub fn root(
        parent_channel: Receiver<u32>,
        content: Arc<RwLock<Content>>,
    ) -> (Self, Receiver<u32>) {
        let (tx, rx) = unbounded();
        (
            Self {
                parent_channel,
                children_channel: tx,
                content,
                indices: Vec::new(),
            },
            rx,
        )
    }

    pub fn new(
        parent_channel: Receiver<u32>,
        content: Arc<RwLock<Content>>,
        pattern: &str,
    ) -> (Self, Receiver<u32>) {
        let (tx, rx) = unbounded();
        (
            Self {
                parent_channel,
                children_channel: tx,
                content,
                indices: Vec::new(),
            },
            rx,
        )
    }

    pub async fn observe_root(&mut self) -> Result<()> {
        loop {
            let index = self.parent_channel.recv().await?;
            // let content = self.content.read().await;
            // if content.matches(pattern)
            self.indices.push(index);
            self.children_channel.send(index).await.unwrap();
            // push to children
        }
    }

    pub async fn observe_node(&mut self) -> Result<()> {
        loop {
            //println!("I am here");
            let index = self.parent_channel.recv().await?;
            let content = self.content.read().await;
            let line = content.get_line(index);
            // if line.matches(pattern)
            self.indices.push(index);
            self.children_channel.send(index).await.unwrap();
        }
    }
}
