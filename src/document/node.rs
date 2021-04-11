use std::sync::Arc;
use tokio::{
    select,
    sync::{
        watch::{channel, Receiver},
        RwLock,
    },
};
use tokio_util::sync::CancellationToken;

use crate::matcher::Matcher;
use crate::Content;

pub trait OnNotify {
    fn get_receiver(&self) -> Receiver<u32>;
    fn set_parent_rx(&mut self, rx: Receiver<u32>);

    fn get_indices(&self) -> Arc<RwLock<Vec<u32>>>;
    fn set_parent_indices(&mut self, indices: Arc<RwLock<Vec<u32>>>);

    fn observe_node<M: 'static + Matcher + Send>(&mut self, matcher: M);
    fn cancel(&self);
}

pub trait Identifiable {
    fn set_id(&mut self, id: usize);
}

pub struct Node {
    content: Arc<RwLock<Content>>,
    indices: Arc<RwLock<Vec<u32>>>,
    parent_indices: Arc<RwLock<Vec<u32>>>,
    name: String,
    id: usize,
    parent_rx: Option<Receiver<u32>>,
    rx: Option<Receiver<u32>>,
    cancellation_token: CancellationToken,
}

impl Node {
    pub fn root(content: Arc<RwLock<Content>>, name: String, parent_rx: Receiver<u32>) -> Self {
        Self {
            content,
            indices: Arc::new(RwLock::new(Vec::new())),
            parent_indices: Arc::new(RwLock::new(Vec::new())),
            name,
            id: 0,
            parent_rx: Some(parent_rx),
            rx: None,
            cancellation_token: CancellationToken::new(),
        }
    }

    pub fn new(content: Arc<RwLock<Content>>, name: String) -> Self {
        Self {
            content,
            indices: Arc::new(RwLock::new(Vec::new())),
            parent_indices: Arc::new(RwLock::new(Vec::new())),
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

    fn get_indices(&self) -> Arc<RwLock<Vec<u32>>> {
        self.indices.clone()
    }

    fn set_parent_indices(&mut self, indices: Arc<RwLock<Vec<u32>>>) {
        self.parent_indices = indices;
    }

    fn observe_node<M: 'static + Matcher + Send>(&mut self, mut matcher: M) {
        let (tx, rx) = channel(0);
        self.rx = Some(rx);
        let cancellation_token = self.cancellation_token.clone();

        let mut parent_rx = self.parent_rx.to_owned().unwrap();
        let name = self.name.clone();

        let content = self.content.clone();
        let indices = self.indices.clone();
        let parent_indices = self.parent_indices.clone();

        let observe_task = async move {
            let mut next_index_to_read = 0;

            while parent_rx.changed().await.is_ok() {
                let notification_index = *parent_rx.borrow();

                for i in next_index_to_read..=notification_index {
                    let content_index = {
                        let parent_indices_read_lock = parent_indices.read().await;
                        *parent_indices_read_lock.get(i as usize).unwrap()
                    };

                    let content_read_lock = content.read().await;
                    let line = content_read_lock.get_line(content_index);
                    if matcher.matches(line) {
                        println!("\"{}\" matches for \"{}\"", line, name);
                        let mut indices_write_lock = indices.write().await;
                        let new_index = indices_write_lock.len();
                        indices_write_lock.push(content_index);
                        tx.send(new_index as u32).unwrap();
                    }
                }

                next_index_to_read = notification_index + 1;
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
