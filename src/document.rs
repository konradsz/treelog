use std::sync::Arc;
use tokio::{
    select,
    sync::{
        watch::{channel, Receiver},
        RwLock,
    },
};
use tokio_util::sync::CancellationToken;

use crate::content::Content;
use crate::matcher::Matcher;
use crate::tree::Node;

pub struct Document {
    content: Arc<RwLock<Content>>,
    indices: Arc<RwLock<Vec<usize>>>,
    parent_indices: Arc<RwLock<Vec<usize>>>,
    name: String,
    id: usize,
    parent_rx: Option<Receiver<usize>>,
    rx: Option<Receiver<usize>>,
    cancellation_token: CancellationToken,
}

impl Document {
    pub fn root(content: Arc<RwLock<Content>>, name: String, parent_rx: Receiver<usize>) -> Self {
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

impl Node for Document {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_receiver(&self) -> Receiver<usize> {
        self.rx.to_owned().unwrap()
    }

    fn set_parent_rx(&mut self, rx: Receiver<usize>) {
        self.parent_rx = Some(rx);
    }

    fn get_indices(&self) -> Arc<RwLock<Vec<usize>>> {
        self.indices.clone()
    }

    fn set_parent_indices(&mut self, indices: Arc<RwLock<Vec<usize>>>) {
        self.parent_indices = indices;
    }

    fn observe<M: 'static + Matcher + Send>(&mut self, mut matcher: M) {
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
                        *parent_indices_read_lock.get(i).unwrap()
                    };

                    let content_read_lock = content.read().await;
                    let line = content_read_lock.get_line(content_index);
                    if matcher.matches(line) {
                        println!("\"{}\" matches for \"{}\"", line, name);
                        let mut indices_write_lock = indices.write().await;
                        let new_index = indices_write_lock.len();
                        indices_write_lock.push(content_index);
                        tx.send(new_index).unwrap();
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
