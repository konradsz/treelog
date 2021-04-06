use std::sync::Arc;
use tokio::sync::{
    watch::{channel, Receiver},
    RwLock,
};

use crate::Content;

pub trait OnNotify {
    fn get_receiver(&self) -> Receiver<u32>;
    fn set_parent_rx(&mut self, rx: Receiver<u32>);
    fn observe_node(&mut self);
}

pub trait Identifiable {
    fn set_id(&mut self, id: usize);
}

pub struct Node {
    _content: Arc<RwLock<Content>>,
    _indices: Vec<u32>,
    _last_read_index: u32,
    name: String,
    id: usize,
    parent_rx: Option<Receiver<u32>>,
    rx: Option<Receiver<u32>>,
}

impl Node {
    pub fn root(content: Arc<RwLock<Content>>, name: String, parent_rx: Receiver<u32>) -> Self {
        Self {
            _content: content,
            _indices: Vec::new(),
            _last_read_index: 0,
            name,
            id: 0,
            parent_rx: Some(parent_rx),
            rx: None,
        }
    }

    pub fn new(content: Arc<RwLock<Content>>, name: String, _pattern: &str) -> Self {
        Self {
            _content: content,
            _indices: Vec::new(),
            _last_read_index: 0,
            name,
            id: 0,
            parent_rx: None,
            rx: None,
        }
    }
}

impl OnNotify for Node {
    fn get_receiver(&self) -> Receiver<u32> {
        let a = self.rx.to_owned();
        a.unwrap()
    }

    fn set_parent_rx(&mut self, rx: Receiver<u32>) {
        self.parent_rx = Some(rx);
    }

    fn observe_node(&mut self) {
        let (tx, rx) = channel(0);
        self.rx = Some(rx);
        let mut parent_rx = self.parent_rx.as_ref().unwrap().clone();
        let name = self.name.clone();
        tokio::spawn(async move {
            let mut times = 0;
            while parent_rx.changed().await.is_ok() {
                let idx = *parent_rx.borrow();
                times += 1;

                //if a > 999_900 {
                println!("\"{}\" notified {} times about: {}", name, times, idx);
                //}

                //if matches
                tx.send(idx).unwrap();
            }
        });
    }
}

impl Identifiable for Node {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}
