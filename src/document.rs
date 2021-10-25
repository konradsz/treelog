use std::sync::Arc;
use tokio::{
    select,
    sync::{
        watch::{channel, Receiver, Sender},
        RwLock,
    },
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::{content::Content, matcher::Matcher, tree::Node};

pub struct Document<C> {
    content: Arc<RwLock<C>>,
    matcher: Arc<dyn Matcher + Send + Sync>,
    indices: Arc<RwLock<Vec<usize>>>,
    parent_indices: Arc<RwLock<Vec<usize>>>,
    name: String,
    new_index_rx: Option<Receiver<usize>>,
    cancellation_token: CancellationToken,
}

impl<C: Content> Document<C> {
    pub fn new(
        name: String,
        content: Arc<RwLock<C>>,
        matcher: Arc<dyn Matcher + Send + Sync>,
    ) -> Self {
        Self {
            content,
            matcher,
            indices: Arc::new(RwLock::new(Vec::new())),
            parent_indices: Arc::new(RwLock::new(Vec::new())),
            name,
            new_index_rx: None,
            cancellation_token: CancellationToken::new(),
        }
    }

    async fn match_indices_range(
        next_index_to_read: usize,
        notification_index: usize,
        parent_indices: &Arc<RwLock<Vec<usize>>>,
        indices: &Arc<RwLock<Vec<usize>>>,
        content: &Arc<RwLock<C>>,
        matcher: &Arc<dyn Matcher + Send + Sync>,
        tx: &Sender<usize>,
        name: &str, // TO REMOVE
    ) {
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
    }
}

impl<C: 'static + Content + Send + Sync> Node for Document<C> {
    fn get_receiver(&self) -> Receiver<usize> {
        self.new_index_rx.to_owned().unwrap()
    }

    fn get_indices(&self) -> Arc<RwLock<Vec<usize>>> {
        self.indices.clone()
    }

    fn set_parent_indices(&mut self, indices: Arc<RwLock<Vec<usize>>>) {
        self.parent_indices = indices;
    }

    fn observe(&mut self, mut new_parent_index_rx: Receiver<usize>) -> JoinHandle<()> {
        let (tx, rx) = channel(0);
        self.new_index_rx = Some(rx);
        let cancellation_token = self.cancellation_token.clone();

        let name = self.name.clone();

        let content = self.content.clone();
        let indices = self.indices.clone();
        let parent_indices = self.parent_indices.clone();

        let matcher = self.matcher.clone();

        let observe_task = async move {
            let mut next_index_to_read = 0;

            while new_parent_index_rx.changed().await.is_ok() {
                let notification_index = *new_parent_index_rx.borrow();

                Self::match_indices_range(
                    next_index_to_read,
                    notification_index,
                    &parent_indices,
                    &indices,
                    &content,
                    &matcher,
                    &tx,
                    &name, // TO REMOVE
                )
                .await;

                next_index_to_read = notification_index + 1;
            }
        };

        tokio::spawn(async move {
            select! {
                _ = cancellation_token.cancelled() => (),
                _ = observe_task => (),
            }
        })
    }

    fn cancel(&self) {
        self.cancellation_token.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{content::Content, matcher::MockMatcher, tree::Node};
    use mockall::predicate::*;

    struct StubContent {
        lines: Vec<String>,
    }

    impl Content for StubContent {
        fn add_line(&mut self, _: String) {}

        fn get_line(&self, index: usize) -> &str {
            &self.lines[index]
        }
    }

    #[tokio::test]
    async fn document_is_notified_about_last_parent_index() {
        const LINES_COUNT: usize = 10;

        let stub_content = StubContent {
            lines: (0..LINES_COUNT).map(|i| i.to_string()).collect(),
        };
        let content = Arc::new(RwLock::new(stub_content));

        let mut matcher = MockMatcher::new();
        for i in 0..LINES_COUNT {
            matcher
                .expect_matches()
                // expected every element up until last sent index
                .withf(move |line: &str| &i.to_string() == line)
                .times(1)
                .return_const(true);
        }

        let mut document = Document::new("root".into(), content, Arc::new(matcher));

        let indices = Arc::new(RwLock::new((0..LINES_COUNT).collect::<Vec<_>>()));
        document.set_parent_indices(indices);

        let (new_parent_index_tx, new_parent_index_rx) = channel(0);
        let jh = document.observe(new_parent_index_rx);

        new_parent_index_tx.send(LINES_COUNT - 5).unwrap();
        new_parent_index_tx.send(LINES_COUNT - 1).unwrap();
        drop(new_parent_index_tx);
        jh.await.unwrap();
    }

    #[tokio::test]
    async fn document_matches_only_content_elements_belonging_to_parent() {
        let stub_content = StubContent {
            lines: (0..10).map(|i| i.to_string()).collect(),
        };
        let content = Arc::new(RwLock::new(stub_content));

        let indices_vector = vec![2, 5, 7];
        let indices = Arc::new(RwLock::new(indices_vector.clone()));
        // notify about second to last element from indices_vector
        let index_notified = indices_vector.len() - 2;

        let mut matcher = MockMatcher::new();
        for el in indices_vector.into_iter().take(index_notified + 1) {
            matcher
                .expect_matches()
                // expected only "2" and "5" from content
                .withf(move |line: &str| line == el.to_string())
                .times(1)
                .return_const(true);
        }

        let mut document = Document::new("document".into(), content, Arc::new(matcher));

        document.set_parent_indices(indices);

        let (new_parent_index_tx, new_parent_index_rx) = channel(0);
        let jh = document.observe(new_parent_index_rx);

        new_parent_index_tx.send(index_notified).unwrap();
        drop(new_parent_index_tx);
        jh.await.unwrap();
    }
}
