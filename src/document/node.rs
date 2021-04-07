use std::sync::Arc;
use tokio::{
    select,
    sync::{
        watch::{channel, Receiver},
        RwLock,
    },
};
use tokio_util::sync::CancellationToken;

use crate::Content;

pub trait OnNotify {
    fn get_receiver(&self) -> Receiver<u32>;
    fn set_parent_rx(&mut self, rx: Receiver<u32>);
    fn observe_node(&mut self);
    fn cancel(&self);
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
    cancellation_token: CancellationToken,
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
            cancellation_token: CancellationToken::new(),
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
            cancellation_token: CancellationToken::new(),
        }
    }
}

impl OnNotify for Node {
    fn get_receiver(&self) -> Receiver<u32> {
        self.rx.to_owned().unwrap()
    }

    fn set_parent_rx(&mut self, rx: Receiver<u32>) {
        self.parent_rx = Some(rx);
    }

    fn observe_node(&mut self) {
        let (tx, rx) = channel(0);
        self.rx = Some(rx);
        let cancellation_token = self.cancellation_token.clone();

        let mut parent_rx = self.parent_rx.to_owned().unwrap();
        let name = self.name.clone();

        let observe_task = async move {
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
        };
        tokio::spawn(async move {
            select! {
                _ = cancellation_token.cancelled() => (),
                _ = observe_task => (),
            }
        });
    }

    fn cancel(&self) {
        self.cancellation_token.cancel();
    }
}

impl Identifiable for Node {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}
